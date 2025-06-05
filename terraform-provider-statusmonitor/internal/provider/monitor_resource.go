package provider

import (
	"context"
	"fmt"
	"strconv"

	"github.com/hashicorp/terraform-plugin-framework/path"
	"github.com/hashicorp/terraform-plugin-framework/resource"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/booldefault"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/int64default"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/planmodifier"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringdefault"
	"github.com/hashicorp/terraform-plugin-framework/resource/schema/stringplanmodifier"
	"github.com/hashicorp/terraform-plugin-framework/types"
	"github.com/hashicorp/terraform-plugin-log/tflog"
)

// Ensure provider defined types fully satisfy framework interfaces.
var _ resource.Resource = &MonitorResource{}
var _ resource.ResourceWithImportState = &MonitorResource{}

func NewMonitorResource() resource.Resource {
	return &MonitorResource{}
}

// MonitorResource defines the resource implementation.
type MonitorResource struct {
	client *APIClient
}

// MonitorResourceModel describes the resource data model.
type MonitorResourceModel struct {
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

func (r *MonitorResource) Metadata(ctx context.Context, req resource.MetadataRequest, resp *resource.MetadataResponse) {
	resp.TypeName = req.ProviderTypeName + "_monitor"
}

func (r *MonitorResource) Schema(ctx context.Context, req resource.SchemaRequest, resp *resource.SchemaResponse) {
	resp.Schema = schema.Schema{
		// This description is used by the documentation generator and the language server.
		MarkdownDescription: "Monitor resource for status monitoring",

		Attributes: map[string]schema.Attribute{
			"id": schema.StringAttribute{
				Computed:            true,
				MarkdownDescription: "Monitor identifier",
				PlanModifiers: []planmodifier.String{
					stringplanmodifier.UseStateForUnknown(),
				},
			},
			"name": schema.StringAttribute{
				Required:            true,
				MarkdownDescription: "Unique name for the monitor",
			},
			"display_name": schema.StringAttribute{
				Required:            true,
				MarkdownDescription: "Display name for the monitor",
			},
			"description": schema.StringAttribute{
				Optional:            true,
				MarkdownDescription: "Description of the monitor",
			},
			"url": schema.StringAttribute{
				Optional:            true,
				MarkdownDescription: "URL to monitor (for HTTP monitors)",
			},
			"monitor_type": schema.StringAttribute{
				Optional:            true,
				Computed:            true,
				Default:             stringdefault.StaticString("http"),
				MarkdownDescription: "Type of monitor (http, tcp, ping, dns, custom)",
			},
			"check_interval": schema.Int64Attribute{
				Optional:            true,
				Computed:            true,
				Default:             int64default.StaticInt64(60),
				MarkdownDescription: "Check interval in seconds",
			},
			"timeout": schema.Int64Attribute{
				Optional:            true,
				Computed:            true,
				Default:             int64default.StaticInt64(30),
				MarkdownDescription: "Timeout in seconds",
			},
			"is_active": schema.BoolAttribute{
				Optional:            true,
				Computed:            true,
				Default:             booldefault.StaticBool(true),
				MarkdownDescription: "Whether the monitor is active",
			},
		},
	}
}

func (r *MonitorResource) Configure(ctx context.Context, req resource.ConfigureRequest, resp *resource.ConfigureResponse) {
	// Prevent panic if the provider has not been configured.
	if req.ProviderData == nil {
		return
	}

	client, ok := req.ProviderData.(*APIClient)

	if !ok {
		resp.Diagnostics.AddError(
			"Unexpected Resource Configure Type",
			fmt.Sprintf("Expected *APIClient, got: %T. Please report this issue to the provider developers.", req.ProviderData),
		)

		return
	}

	r.client = client
}

func (r *MonitorResource) Create(ctx context.Context, req resource.CreateRequest, resp *resource.CreateResponse) {
	var data MonitorResourceModel

	// Read Terraform plan data into the model
	resp.Diagnostics.Append(req.Plan.Get(ctx, &data)...)

	if resp.Diagnostics.HasError() {
		return
	}

	// Convert from Terraform model to API request
	createReq := CreateMonitorRequest{
		Name:          data.Name.ValueString(),
		DisplayName:   data.DisplayName.ValueString(),
		MonitorType:   data.MonitorType.ValueString(),
		CheckInterval: int(data.CheckInterval.ValueInt64()),
		Timeout:       int(data.Timeout.ValueInt64()),
		IsActive:      data.IsActive.ValueBool(),
	}

	if !data.Description.IsNull() {
		desc := data.Description.ValueString()
		createReq.Description = &desc
	}

	if !data.Url.IsNull() {
		url := data.Url.ValueString()
		createReq.URL = &url
	}

	// Create monitor via API
	tflog.Debug(ctx, "Creating monitor", map[string]interface{}{
		"name": createReq.Name,
	})

	monitor, err := r.client.CreateMonitor(ctx, createReq)
	if err != nil {
		resp.Diagnostics.AddError(
			"Error Creating Monitor",
			"Could not create monitor, unexpected error: "+err.Error(),
		)
		return
	}

	// Update model with response data
	data.Id = types.StringValue(strconv.Itoa(monitor.ID))
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

	tflog.Trace(ctx, "Created monitor resource")

	// Save data into Terraform state
	resp.Diagnostics.Append(resp.State.Set(ctx, &data)...)
}

func (r *MonitorResource) Read(ctx context.Context, req resource.ReadRequest, resp *resource.ReadResponse) {
	var data MonitorResourceModel

	// Read Terraform prior state data into the model
	resp.Diagnostics.Append(req.State.Get(ctx, &data)...)

	if resp.Diagnostics.HasError() {
		return
	}

	// Parse ID
	id, err := strconv.Atoi(data.Id.ValueString())
	if err != nil {
		resp.Diagnostics.AddError(
			"Invalid Monitor ID",
			"Could not parse monitor ID: "+err.Error(),
		)
		return
	}

	// Get monitor from API
	monitor, err := r.client.GetMonitor(ctx, id)
	if err != nil {
		resp.Diagnostics.AddError(
			"Error Reading Monitor",
			"Could not read monitor ID "+data.Id.ValueString()+": "+err.Error(),
		)
		return
	}

	// Remove resource if not found
	if monitor == nil {
		resp.State.RemoveResource(ctx)
		return
	}

	// Update model with response data
	data.Id = types.StringValue(strconv.Itoa(monitor.ID))
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

	// Save updated data into Terraform state
	resp.Diagnostics.Append(resp.State.Set(ctx, &data)...)
}

func (r *MonitorResource) Update(ctx context.Context, req resource.UpdateRequest, resp *resource.UpdateResponse) {
	var data MonitorResourceModel

	// Read Terraform plan data into the model
	resp.Diagnostics.Append(req.Plan.Get(ctx, &data)...)

	if resp.Diagnostics.HasError() {
		return
	}

	// Parse ID
	id, err := strconv.Atoi(data.Id.ValueString())
	if err != nil {
		resp.Diagnostics.AddError(
			"Invalid Monitor ID",
			"Could not parse monitor ID: "+err.Error(),
		)
		return
	}

	// Convert from Terraform model to API request
	updateReq := UpdateMonitorRequest{}

	// Only include fields that are being updated
	name := data.Name.ValueString()
	updateReq.Name = &name

	displayName := data.DisplayName.ValueString()
	updateReq.DisplayName = &displayName

	monitorType := data.MonitorType.ValueString()
	updateReq.MonitorType = &monitorType

	checkInterval := int(data.CheckInterval.ValueInt64())
	updateReq.CheckInterval = &checkInterval

	timeout := int(data.Timeout.ValueInt64())
	updateReq.Timeout = &timeout

	isActive := data.IsActive.ValueBool()
	updateReq.IsActive = &isActive

	if !data.Description.IsNull() {
		desc := data.Description.ValueString()
		updateReq.Description = &desc
	}

	if !data.Url.IsNull() {
		url := data.Url.ValueString()
		updateReq.URL = &url
	}

	// Update monitor via API
	tflog.Debug(ctx, "Updating monitor", map[string]interface{}{
		"id": id,
	})

	monitor, err := r.client.UpdateMonitor(ctx, id, updateReq)
	if err != nil {
		resp.Diagnostics.AddError(
			"Error Updating Monitor",
			"Could not update monitor ID "+data.Id.ValueString()+": "+err.Error(),
		)
		return
	}

	// Update model with response data
	data.Id = types.StringValue(strconv.Itoa(monitor.ID))
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

	// Save updated data into Terraform state
	resp.Diagnostics.Append(resp.State.Set(ctx, &data)...)
}

func (r *MonitorResource) Delete(ctx context.Context, req resource.DeleteRequest, resp *resource.DeleteResponse) {
	var data MonitorResourceModel

	// Read Terraform prior state data into the model
	resp.Diagnostics.Append(req.State.Get(ctx, &data)...)

	if resp.Diagnostics.HasError() {
		return
	}

	// Parse ID
	id, err := strconv.Atoi(data.Id.ValueString())
	if err != nil {
		resp.Diagnostics.AddError(
			"Invalid Monitor ID",
			"Could not parse monitor ID: "+err.Error(),
		)
		return
	}

	// Delete monitor via API
	tflog.Debug(ctx, "Deleting monitor", map[string]interface{}{
		"id": id,
	})

	err = r.client.DeleteMonitor(ctx, id)
	if err != nil {
		resp.Diagnostics.AddError(
			"Error Deleting Monitor",
			"Could not delete monitor ID "+data.Id.ValueString()+": "+err.Error(),
		)
		return
	}

	tflog.Trace(ctx, "Deleted monitor resource")
}

func (r *MonitorResource) ImportState(ctx context.Context, req resource.ImportStateRequest, resp *resource.ImportStateResponse) {
	resource.ImportStatePassthroughID(ctx, path.Root("id"), req, resp)
}