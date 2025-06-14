use axum::{
    error_handling::HandleErrorLayer,
    extract::{Query, State},
    http::Uri,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_oidc::{
    error::MiddlewareError,
    EmptyAdditionalClaims,
    OidcAuthLayer,
    OidcClaims,
    OidcLoginLayer,
    OidcRpInitiatedLogout,
};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_sessions::{
    cookie::{time::Duration, SameSite},
    Expiry,
    SessionManagerLayer,
};
use tower_sessions_sqlx_store::PostgresStore;
use axum_inertia::Inertia;
use serde_json::json;
use db_core::DbPool;

// Follow the same pattern as other routes with generic state
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    DbPool: axum::extract::FromRef<S>,
    axum_inertia::InertiaConfig: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/auth/login", get(login_page))
        .route("/auth/signin", get(start_signin))
        .route("/auth/maybe-protected", get(maybe_authenticated))
}

pub async fn create_auth_router() -> Result<Router, Box<dyn std::error::Error + Send + Sync>> {
    println!("🔧 Setting up OIDC authentication...");
    
    // Get environment variables with better error handling
    let app_url = std::env::var("OIDC_REDIRECT_URL")
        .or_else(|_| std::env::var("APP_URL"))
        .map(|url| {
            // Extract base URL from callback URL if it contains /auth/callback
            if url.ends_with("/auth/callback") {
                url.trim_end_matches("/auth/callback").to_string()
            } else {
                url
            }
        })
        .unwrap_or_else(|_| {
            println!("⚠️  OIDC_REDIRECT_URL not set, using default: http://localhost:8000");
            "http://localhost:8000".to_string()
        });
    
    let issuer = match std::env::var("OIDC_ISSUER_URL") {
        Ok(url) => {
            println!("✅ OIDC_ISSUER_URL: {}", url);
            url
        }
        Err(_) => {
            println!("❌ OIDC_ISSUER_URL not set! Using Google as fallback for testing.");
            println!("   Set OIDC_ISSUER_URL environment variable for production use.");
            "https://accounts.google.com".to_string()
        }
    };
    
    let client_id = match std::env::var("OIDC_CLIENT_ID") {
        Ok(id) => {
            println!("✅ OIDC_CLIENT_ID: {}", id);
            id
        }
        Err(_) => {
            println!("❌ OIDC_CLIENT_ID not set! Using dummy value for testing.");
            println!("   Set OIDC_CLIENT_ID environment variable for production use.");
            "dummy_client_id".to_string()
        }
    };
    
    let client_secret = std::env::var("OIDC_CLIENT_SECRET").ok();
    if client_secret.is_some() {
        println!("✅ OIDC_CLIENT_SECRET: [REDACTED]");
    } else {
        println!("⚠️  OIDC_CLIENT_SECRET not set (optional for some providers)");
    }

    println!("🔧 Initializing database pool for sessions...");
    // Initialize database pool for session store
    let db_pool = db_core::init_pool().await?;
    
    // Create PostgreSQL session store
    println!("🔧 Setting up PostgreSQL session store...");
    let session_store = PostgresStore::new((*db_pool).clone());
    session_store.migrate().await?;
    println!("✅ Session store migrated successfully");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production with HTTPS
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(3600))); // 1 hour

    println!("🔧 Creating OIDC login service...");
    // Create OIDC login service (requires authentication)
    let oidc_login_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            println!("🚨 OIDC Login Error: {:?}", e);
            e.into_response()
        }))
        .layer(OidcLoginLayer::<EmptyAdditionalClaims>::new());

    println!("🔧 Discovering OIDC client configuration...");
    println!("   App URL: {}", app_url);
    println!("   Issuer: {}", issuer);
    
    // Create OIDC auth service (optional authentication)
    let oidc_auth_layer = match OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
        Uri::from_maybe_shared(app_url.clone())?,
        issuer.clone(),
        client_id.clone(),
        client_secret.clone(),
        vec![],
    )
    .await
    {
        Ok(layer) => {
            println!("✅ OIDC client discovery successful!");
            layer
        }
        Err(e) => {
            println!("❌ OIDC client discovery failed: {:?}", e);
            println!("   This might be due to:");
            println!("   - Invalid OIDC_ISSUER_URL");
            println!("   - Network connectivity issues");
            println!("   - Invalid client credentials");
            return Err(e.into());
        }
    };

    let oidc_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            println!("🚨 OIDC Auth Error: {:?}", e);
            e.into_response()
        }))
        .layer(oidc_auth_layer);

    println!("🔧 Building auth router...");
    // Create the router with OIDC middleware layers
    let app = Router::new()
        // Routes that REQUIRE authentication (with OidcLoginLayer)
        .route("/auth/callback", get(auth_callback))
        .route("/auth/protected", get(authenticated))
        .route("/auth/logout", get(logout))
        .layer(oidc_login_service)
        // Routes with OPTIONAL authentication (with OidcAuthLayer)
        .layer(oidc_auth_service)
        .layer(session_layer);

    println!("✅ Auth router created successfully!");
    Ok(app)
}

// Login page handler - shows the login form
async fn login_page(inertia: Inertia) -> impl IntoResponse {
    println!("📄 Serving login page");
    inertia.render("Login", json!({
        "loginUrl": "/auth/signin"
    }))
}

// Start signin process - redirects to OIDC provider
async fn start_signin() -> impl IntoResponse {
    println!("🔄 Starting OIDC signin process");
    // Redirect to the callback route, which is protected and will trigger OIDC flow
    Redirect::to("/auth/callback")
}

// Handler that requires authentication
async fn authenticated(claims: OidcClaims<EmptyAdditionalClaims>) -> impl IntoResponse {
    println!("🔐 Authenticated user: {}", claims.subject().as_str());
    format!("Hello {}! You are authenticated.", claims.subject().as_str())
}

// Handler with optional authentication
async fn maybe_authenticated(
    claims: Result<OidcClaims<EmptyAdditionalClaims>, axum_oidc::error::ExtractorError>,
) -> impl IntoResponse {
    match &claims {
        Ok(claims) => {
            println!("🔐 Optional auth - user authenticated: {}", claims.subject().as_str());
        }
        Err(e) => {
            println!("🔓 Optional auth - no authentication: {:?}", e);
        }
    }
    
    if let Ok(claims) = claims {
        format!(
            "Hello {}! You are already logged in from another handler.",
            claims.subject().as_str()
        )
    } else {
        "Hello anonymous user! You can log in at /auth/login".to_string()
    }
}

#[derive(Deserialize)]
struct CallbackQuery {
    origin: Option<String>,
}

async fn auth_callback(
    oidc_claims: OidcClaims<EmptyAdditionalClaims>,
    Query(query): Query<CallbackQuery>,
) -> impl IntoResponse {
    let user_id = oidc_claims.subject().to_string();
    println!("🔄 OIDC callback received for user: {}", user_id);
    
    // Redirect to origin if provided, otherwise to home page
    let redirect_url = query.origin.unwrap_or_else(|| "/".to_string());
    
    println!("✅ Authentication successful! Redirecting to: {}", redirect_url);
    Redirect::to(&redirect_url)
}

// Logout handler
async fn logout(logout: OidcRpInitiatedLogout) -> impl IntoResponse {
    println!("🚪 User logging out");
    let redirect_url = std::env::var("OIDC_REDIRECT_URL")
        .unwrap_or_else(|_| "http://localhost:8000".to_string());
    
    logout.with_post_logout_redirect(
        Uri::from_maybe_shared(redirect_url)
            .unwrap_or_else(|_| Uri::from_static("http://localhost:8000"))
    )
} 