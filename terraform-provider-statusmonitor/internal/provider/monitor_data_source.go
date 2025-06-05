package provider

import (
	"context"
	"fmt"

	"github.com/hashicorp/terraform-plugin-framework/datasource"
	"github.com/hashicorp/terraform-plugin-framework/datasource/schema"
	"github.com/hashicorp/terraform-plugin-framework/types"
	"github.com/hashicorp/terraform-plugin-log/tflog"
)

// Ensure provider defined types fully satisfy framework interfaces.
var _ datasource.DataSource = &MonitorDataSource{}

func NewMonitorDataSource() datasource.DataSource {
	return &MonitorDataSource{}
}

// MonitorDataSource defines the data source implementation.
type MonitorDataSource struct {
	client *APIClient
}

// MonitorDataSourceModel describes the data source data model.
type MonitorDataSourceModel struct {
	Id            types.String `tfsdk:"id"`
	Name          types.String `tfsdk:"name"`
	DisplayName   types.String `tfsdk:"display_name"`
	Description   types.String `tfsdk:"description"`
	Url           types.String `tfsdk:"url"`
	MonitorType   types.String `tfsdk:"monitor_type"`
	CheckInterval types.Int64  `tfsdk:"check_interval"`
	Timeout       types.Int64  `tfsdk:"timeout"`
	IsActive      types.Bool   `tfsdk:"is_active"`
}

func (d *MonitorDataSource) Metadata(ctx context.Context, req datasource.MetadataRequest, resp *datasource.MetadataResponse) {
	resp.TypeName = req.ProviderTypeName + "_monitor"
}

func (d *MonitorDataSource) Schema(ctx context.Context, req datasource.SchemaRequest, resp *datasource.SchemaResponse) {
	resp.Schema = schema.Schema{
		// This description is used by the documentation generator and the language server.
		MarkdownDescription: "Monitor data source for reading existing monitors",

		Attributes: map[string]schema.Attribute{
			"id": schema.StringAttribute{
				Computed:            true,
				MarkdownDescription: "Monitor identifier",
			},
			"name": schema.StringAttribute{
				Required:            true,
				MarkdownDescription: "Unique name of the monitor to look up",
			},
			"display_name": schema.StringAttribute{
				Computed:            true,
				MarkdownDescription: "Display name for the monitor",
			},
			"description": schema.StringAttribute{
				Computed:            true,
				MarkdownDescription: "Description of the monitor",
			},
			"url": schema.StringAttribute{
				Computed:            true,
				MarkdownDescription: "URL being monitored (for HTTP monitors)",
			},
			"monitor_type": schema.StringAttribute{
				Computed:            true,
				MarkdownDescription: "Type of monitor (http, tcp, ping, dns, custom)",
			},
			"check_interval": schema.Int64Attribute{
				Computed:            true,
				MarkdownDescription: "Check interval in seconds",
			},
			"timeout": schema.Int64Attribute{
				Computed:            true,
				MarkdownDescription: "Timeout in seconds",
			},
			"is_active": schema.BoolAttribute{
				Computed:            true,
				MarkdownDescription: "Whether the monitor is active",
			},
		},
	}
}

func (d *MonitorDataSource) Configure(ctx context.Context, req datasource.ConfigureRequest, resp *datasource.ConfigureResponse) {
	// Prevent panic if the provider has not been configured.
	if req.ProviderData == nil {
		return
	}

	client, ok := req.ProviderData.(*APIClient)

	if !ok {
		resp.Diagnostics.AddError(
			"Unexpected Data Source Configure Type",
			fmt.Sprintf("Expected *APIClient, got: %T. Please report this issue to the provider developers.", req.ProviderData),
		)

		return
	}

	d.client = client
}

func (d *MonitorDataSource) Read(ctx context.Context, req datasource.ReadRequest, resp *datasource.ReadResponse) {
	var data MonitorDataSourceModel

	// Read Terraform configuration data into the model
	resp.Diagnostics.Append(req.Config.Get(ctx, &data)...)

	if resp.Diagnostics.HasError() {
		return
	}

	// Get monitor by name from API
	tflog.Debug(ctx, "Reading monitor by name", map[string]interface{}{
		"name": data.Name.ValueString(),
	})

	monitor, err := d.client.GetMonitorByName(ctx, data.Name.ValueString())
	if err != nil {
		resp.Diagnostics.AddError(
			"Error Reading Monitor",
			"Could not read monitor by name "+data.Name.ValueString()+": "+err.Error(),
		)
		return
	}

	if monitor == nil {
		resp.Diagnostics.AddError(
			"Monitor Not Found",
			"No monitor found with name: "+data.Name.ValueString(),
		)
		return
	}

	// Map response body to model
	data.Id = types.StringValue(fmt.Sprintf("%d", monitor.ID))
	data.Name = types.StringValue(monitor.Name)
	data.DisplayName = types.StringValue(monitor.DisplayName)
	data.MonitorType = types.StringValue(monitor.MonitorType)
	data.CheckInterval = types.Int64Value(int64(monitor.CheckInterval))
	data.Timeout = types.Int64Value(int64(monitor.Timeout))
	data.IsActive = types.BoolValue(monitor.IsActive)

	if monitor.Description != nil {
		data.Description = types.StringValue(*monitor.Description)
	} else {
		data.Description = types.StringNull()
	}

	if monitor.URL != nil {
		data.Url = types.StringValue(*monitor.URL)
	} else {
		data.Url = types.StringNull()
	}

	tflog.Trace(ctx, "Read monitor data source", map[string]interface{}{
		"id":   data.Id.ValueString(),
		"name": data.Name.ValueString(),
	})

	// Save data into Terraform state
	resp.Diagnostics.Append(resp.State.Set(ctx, &data)...)
}