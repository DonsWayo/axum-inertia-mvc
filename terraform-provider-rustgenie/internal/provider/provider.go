package provider

import (
	"context"
	"fmt"

	"github.com/hashicorp/terraform-plugin-framework/datasource"
	"github.com/hashicorp/terraform-plugin-framework/provider"
	"github.com/hashicorp/terraform-plugin-framework/provider/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/types"
)

// Ensure StatusMonitorProvider satisfies various provider interfaces.
var _ provider.Provider = &StatusMonitorProvider{}

// StatusMonitorProvider defines the provider implementation.
type StatusMonitorProvider struct {
	// version is set to the provider version on release, "dev" when the
	// provider is built and ran locally, and "test" when running acceptance
	// testing.
	version string
}

// StatusMonitorProviderModel describes the provider data model.
type StatusMonitorProviderModel struct {
	Endpoint types.String `tfsdk:"endpoint"`
	ApiKey   types.String `tfsdk:"api_key"`
}

func (p *StatusMonitorProvider) Metadata(ctx context.Context, req provider.MetadataRequest, resp *provider.MetadataResponse) {
	resp.TypeName = "statusmonitor"
	resp.Version = p.version
}

func (p *StatusMonitorProvider) Schema(ctx context.Context, req provider.SchemaRequest, resp *provider.SchemaResponse) {
	resp.Schema = schema.Schema{
		Attributes: map[string]schema.Attribute{
			"endpoint": schema.StringAttribute{
				MarkdownDescription: "The endpoint URL for the RustGenie API",
				Optional:            true,
			},
			"api_key": schema.StringAttribute{
				MarkdownDescription: "API key for authenticating with the RustGenie API",
				Optional:            true,
				Sensitive:           true,
			},
		},
	}
}

func (p *StatusMonitorProvider) Configure(ctx context.Context, req provider.ConfigureRequest, resp *provider.ConfigureResponse) {
	var data StatusMonitorProviderModel

	resp.Diagnostics.Append(req.Config.Get(ctx, &data)...)

	if resp.Diagnostics.HasError() {
		return
	}

	// Configuration values are now available.
	endpoint := data.Endpoint.ValueString()
	apiKey := data.ApiKey.ValueString()

	// Default endpoint if not configured
	if endpoint == "" {
		endpoint = "http://localhost:8000"
	}

	// Create API client configuration that will be passed to resources
	client := &APIClient{
		Endpoint: endpoint,
		APIKey:   apiKey,
	}

	// Make the client available to resources and data sources
	resp.DataSourceData = client
	resp.ResourceData = client
}

func (p *StatusMonitorProvider) Resources(ctx context.Context) []func() resource.Resource {
	return []func() resource.Resource{
		NewMonitorResource,
	}
}

func (p *StatusMonitorProvider) DataSources(ctx context.Context) []func() datasource.DataSource {
	return []func() datasource.DataSource{
		NewMonitorDataSource,
	}
}

func New(version string) func() provider.Provider {
	return func() provider.Provider {
		return &StatusMonitorProvider{
			version: version,
		}
	}
}

