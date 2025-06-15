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
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_sessions::{
    cookie::{time::Duration, SameSite},
    Expiry,
    MemoryStore,
    Session,
    SessionManagerLayer,
};
use tower_sessions_sqlx_store::PostgresStore;
use axum_inertia::Inertia;
use serde_json::json;
use db_core::{DbPool, repositories::UserRepository, models::user::User};

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
        .route("/auth/user", get(get_current_user))
}

pub async fn create_auth_router<S>() -> Result<Router<S>, Box<dyn std::error::Error + Send + Sync>>
where
    S: Clone + Send + Sync + 'static,
    DbPool: axum::extract::FromRef<S>,
    axum_inertia::InertiaConfig: axum::extract::FromRef<S>,
{
    // Get environment variables
    let app_url = std::env::var("OIDC_REDIRECT_URL")
        .or_else(|_| std::env::var("APP_URL"))
        .map(|url| {
            if url.ends_with("/auth/callback") {
                url.trim_end_matches("/auth/callback").to_string()
            } else {
                url
            }
        })
        .unwrap_or_else(|_| "http://localhost:8000".to_string());
    
    let issuer = std::env::var("OIDC_ISSUER_URL")
        .unwrap_or_else(|_| "https://accounts.google.com".to_string());
    
    let client_id = std::env::var("OIDC_CLIENT_ID")
        .unwrap_or_else(|_| "dummy_client_id".to_string());
    
    let client_secret = std::env::var("OIDC_CLIENT_SECRET").ok();

    println!("=== OIDC Configuration Debug ===");
    println!("App URL: {}", app_url);
    println!("Issuer: {}", issuer);
    println!("Client ID: {}", client_id);
    println!("Client Secret: {:?}", client_secret.as_ref().map(|_| "***"));
    println!("================================");

    // Initialize database pool and session store
    let db_pool = db_core::init_pool().await?;
    let session_store = PostgresStore::new((*db_pool).clone());
    session_store.migrate().await?;

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production with HTTPS
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(3600))); // 1 hour

    // Create OIDC login service (requires authentication)
    let oidc_login_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(OidcLoginLayer::<EmptyAdditionalClaims>::new());

    // Create OIDC auth service (optional authentication)
    let oidc_auth_layer = OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
        Uri::from_maybe_shared(app_url.clone())?,
        issuer.clone(),
        client_id.clone(),
        client_secret.clone(),
        vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
    )
    .await?;

    let oidc_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(oidc_auth_layer);

    // Create the router with OIDC middleware layers
    let app = Router::new()
        // Routes that REQUIRE authentication (with OidcLoginLayer)
        .route("/auth/callback", get(auth_callback))
        .route("/auth/protected", get(authenticated))
        .route("/auth/logout", get(logout))
        .layer(oidc_login_service)
        // Routes with OPTIONAL authentication (with OidcAuthLayer)
        .route("/auth/login", get(login_page))
        .route("/auth/signin", get(start_signin))
        .route("/auth/maybe-protected", get(maybe_authenticated))
        .route("/auth/user", get(get_current_user))
        .layer(oidc_auth_service)
        .layer(session_layer);

    Ok(app)
}

// Login page handler
async fn login_page(inertia: Inertia) -> impl IntoResponse {
    inertia.render("Login", json!({
        "loginUrl": "/auth/signin"
    }))
}

// Start signin process
async fn start_signin() -> impl IntoResponse {
    Redirect::to("/auth/callback")
}

// Handler that requires authentication
async fn authenticated(
    claims: OidcClaims<EmptyAdditionalClaims>,
    session: Session,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    // Log all available claims for debugging
    println!("=== OIDC Claims Debug ===");
    println!("Subject: {}", claims.subject().as_str());
    println!("Email: {:?}", claims.email());
    println!("Name: {:?}", claims.name());
    println!("Given name: {:?}", claims.given_name());
    println!("Family name: {:?}", claims.family_name());
    println!("Preferred username: {:?}", claims.preferred_username());
    println!("Picture: {:?}", claims.picture());
    println!("========================");
    
    let user_id = claims.subject().as_str();
    let email = claims.email().map(|e| e.as_str());
    let name = claims.name().and_then(|n| n.get(None)).map(|s| s.as_str());
    
    println!("Extracted values - user_id: {}, email: {:?}, name: {:?}", user_id, email, name);
    
    // Use UserRepository to find or create user
    let user_repo = UserRepository::new(&db_pool);
    match user_repo.find_or_create_by_oidc_subject(user_id, email, name).await {
        Ok(user) => {
            // Store user ID in session
            if let Err(e) = session.insert("user_id", user.id).await {
                eprintln!("Failed to store user ID in session: {}", e);
            }
            
            format!("Hello {}! You are authenticated. Database ID: {}", user_id, user.id)
        }
        Err(e) => {
            eprintln!("Failed to find or create user: {}", e);
            format!("Hello {}! You are authenticated but there was a database error.", user_id)
        }
    }
}

// Handler with optional authentication
async fn maybe_authenticated(
    claims: Result<OidcClaims<EmptyAdditionalClaims>, axum_oidc::error::ExtractorError>,
    session: Session,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let user_repo = UserRepository::new(&db_pool);
    
    if let Ok(claims) = claims {
        let user_id = claims.subject().as_str();
        let email = claims.email().map(|e| e.as_str());
        let name = claims.name().and_then(|n| n.get(None)).map(|s| s.as_str());
        
        // Use UserRepository to find or create user
        match user_repo.find_or_create_by_oidc_subject(user_id, email, name).await {
            Ok(user) => {
                // Store user ID in session
                if let Err(e) = session.insert("user_id", user.id).await {
                    eprintln!("Failed to store user ID in session: {}", e);
                }
                
                format!("Hello {}! You are logged in via OIDC. Database ID: {}", user_id, user.id)
            }
            Err(e) => {
                eprintln!("Failed to find or create user: {}", e);
                format!("Hello {}! You are authenticated but there was a database error.", user_id)
            }
        }
    } else {
        // Check if user exists in session
        match session.get::<i32>("user_id").await {
            Ok(Some(user_id)) => {
                // Load user from database using repository
                match user_repo.find_by_id(user_id).await {
                    Ok(Some(user)) => {
                        format!("Hello {}! You are logged in via session. Database ID: {}", 
                               user.oidc_subject, user.id)
                    }
                    Ok(None) => {
                        // User not found in database, clear session
                        let _ = session.remove::<i32>("user_id").await;
                        "Hello anonymous user! You can log in at /auth/login".to_string()
                    }
                    Err(e) => {
                        eprintln!("Failed to load user from database: {}", e);
                        "Hello anonymous user! You can log in at /auth/login".to_string()
                    }
                }
            }
            Ok(None) => "Hello anonymous user! You can log in at /auth/login".to_string(),
            Err(e) => {
                eprintln!("Failed to get user ID from session: {}", e);
                "Hello anonymous user! You can log in at /auth/login".to_string()
            }
        }
    }
}

// Get current user from session and database
async fn get_current_user(
    session: Session,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    let user_repo = UserRepository::new(&db_pool);
    
    println!("=== Get Current User Debug ===");
    println!("Session ID: {:?}", session.id());
    
    // Try to get all session data for debugging
    match session.get::<i32>("user_id").await {
        Ok(Some(user_id)) => {
            println!("Found user_id in session: {}", user_id);
            // Load user from database using repository
            match user_repo.find_by_id(user_id).await {
                Ok(Some(user)) => {
                    axum::Json(json!({
                        "authenticated": true,
                        "user": user
                    }))
                }
                Ok(None) => {
                    // User not found in database, clear session
                    let _ = session.remove::<i32>("user_id").await;
                    axum::Json(json!({
                        "authenticated": false,
                        "user": null
                    }))
                }
                Err(e) => {
                    eprintln!("Failed to load user from database: {}", e);
                    axum::Json(json!({
                        "authenticated": false,
                        "user": null,
                        "error": "Database error"
                    }))
                }
            }
        }
        Ok(None) => {
            println!("No user_id found in session");
            axum::Json(json!({
                "authenticated": false,
                "user": null
            }))
        }
        Err(e) => {
            eprintln!("Failed to get user ID from session: {}", e);
            axum::Json(json!({
                "authenticated": false,
                "user": null,
                "error": "Session error"
            }))
        }
    }
}

#[derive(Deserialize)]
struct CallbackQuery {
    origin: Option<String>,
}

async fn auth_callback(
    oidc_claims: OidcClaims<EmptyAdditionalClaims>,
    Query(query): Query<CallbackQuery>,
    session: Session,
    State(db_pool): State<DbPool>,
) -> impl IntoResponse {
    // Log all available claims for debugging
    println!("=== OIDC Callback Claims Debug ===");
    println!("Subject: {}", oidc_claims.subject().as_str());
    println!("Email: {:?}", oidc_claims.email());
    println!("Name: {:?}", oidc_claims.name());
    println!("Given name: {:?}", oidc_claims.given_name());
    println!("Family name: {:?}", oidc_claims.family_name());
    println!("Preferred username: {:?}", oidc_claims.preferred_username());
    println!("Picture: {:?}", oidc_claims.picture());
    println!("==================================");
    
    let user_id = oidc_claims.subject().as_str();
    let email = oidc_claims.email().map(|e| e.as_str());
    let name = oidc_claims.name().and_then(|n| n.get(None)).map(|s| s.as_str());
    
    println!("Extracted values - user_id: {}, email: {:?}, name: {:?}", user_id, email, name);
    
    // Use UserRepository to find or create user
    let user_repo = UserRepository::new(&db_pool);
    match user_repo.find_or_create_by_oidc_subject(user_id, email, name).await {
        Ok(user) => {
            // Store user ID in session
            println!("Storing user ID {} in session", user.id);
            println!("Session ID during callback: {:?}", session.id());
            if let Err(e) = session.insert("user_id", user.id).await {
                eprintln!("Failed to store user ID in session: {}", e);
                return Redirect::to("/auth/login?error=session_error");
            }
            println!("Successfully stored user ID in session");
            
            // Verify the data was stored
            match session.get::<i32>("user_id").await {
                Ok(Some(stored_id)) => println!("Verified: user_id {} is stored in session", stored_id),
                Ok(None) => println!("WARNING: user_id not found immediately after storing!"),
                Err(e) => println!("ERROR: Failed to verify stored user_id: {}", e),
            }
            
            let redirect_url = query.origin.unwrap_or_else(|| "/".to_string());
            Redirect::to(&redirect_url)
        }
        Err(e) => {
            eprintln!("Failed to find or create user: {}", e);
            Redirect::to("/auth/login?error=database_error")
        }
    }
}

// Logout handler
async fn logout(
    logout: OidcRpInitiatedLogout,
    session: Session,
) -> impl IntoResponse {
    // Clear user ID from session
    if let Err(e) = session.remove::<i32>("user_id").await {
        eprintln!("Failed to remove user ID from session: {}", e);
    }
    
    let redirect_url = std::env::var("OIDC_REDIRECT_URL")
        .unwrap_or_else(|_| "http://localhost:8000".to_string());
    
    logout.with_post_logout_redirect(
        Uri::from_maybe_shared(redirect_url)
            .unwrap_or_else(|_| Uri::from_static("http://localhost:8000"))
    )
} 