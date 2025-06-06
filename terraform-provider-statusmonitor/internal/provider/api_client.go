package provider

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

// Monitor represents the monitor data structure from the API
type Monitor struct {
	ID            int                    `json:"id,omitempty"`
	Name          string                 `json:"name"`
	DisplayName   string                 `json:"display_name"`
	Description   *string                `json:"description,omitempty"`
	URL           *string                `json:"url,omitempty"`
	MonitorType   string                 `json:"monitor_type"`
	CheckInterval int                    `json:"check_interval"`
	Timeout       int                    `json:"timeout"`
	IsActive      bool                   `json:"is_active"`
	Metadata      map[string]interface{} `json:"metadata,omitempty"`
	CreatedAt     *time.Time             `json:"created_at,omitempty"`
	UpdatedAt     *time.Time             `json:"updated_at,omitempty"`
}

// CreateMonitorRequest represents the request body for creating a monitor
type CreateMonitorRequest struct {
	Name          string                 `json:"name"`
	DisplayName   string                 `json:"display_name"`
	Description   *string                `json:"description,omitempty"`
	URL           *string                `json:"url,omitempty"`
	MonitorType   string                 `json:"monitor_type"`
	CheckInterval int                    `json:"check_interval"`
	Timeout       int                    `json:"timeout"`
	IsActive      bool                   `json:"is_active"`
	Metadata      map[string]interface{} `json:"metadata,omitempty"`
}

// UpdateMonitorRequest represents the request body for updating a monitor
type UpdateMonitorRequest struct {
	Name          *string                `json:"name,omitempty"`
	DisplayName   *string                `json:"display_name,omitempty"`
	Description   *string                `json:"description,omitempty"`
	URL           *string                `json:"url,omitempty"`
	MonitorType   *string                `json:"monitor_type,omitempty"`
	CheckInterval *int                   `json:"check_interval,omitempty"`
	Timeout       *int                   `json:"timeout,omitempty"`
	IsActive      *bool                  `json:"is_active,omitempty"`
	Metadata      map[string]interface{} `json:"metadata,omitempty"`
}

// doRequest performs an HTTP request and handles common error cases
func (c *APIClient) doRequest(ctx context.Context, method, path string, body interface{}) (*http.Response, error) {
	var bodyReader io.Reader
	if body != nil {
		jsonBody, err := json.Marshal(body)
		if err != nil {
			return nil, fmt.Errorf("failed to marshal request body: %w", err)
		}
		bodyReader = bytes.NewReader(jsonBody)
	}

	req, err := http.NewRequestWithContext(ctx, method, fmt.Sprintf("%s%s", c.Endpoint, path), bodyReader)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")
	if c.APIKey != "" {
		req.Header.Set("Authorization", fmt.Sprintf("Bearer %s", c.APIKey))
	}

	client := &http.Client{Timeout: 30 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("request failed: %w", err)
	}

	return resp, nil
}

// CreateMonitor creates a new monitor
func (c *APIClient) CreateMonitor(ctx context.Context, monitor CreateMonitorRequest) (*Monitor, error) {
	resp, err := c.doRequest(ctx, "POST", "/status/api/monitors", monitor)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusCreated {
		body, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("failed to create monitor: %s (status %d)", string(body), resp.StatusCode)
	}

	var result Monitor
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &result, nil
}

// GetMonitor retrieves a monitor by ID
func (c *APIClient) GetMonitor(ctx context.Context, id int) (*Monitor, error) {
	resp, err := c.doRequest(ctx, "GET", fmt.Sprintf("/status/api/monitors/%d", id), nil)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusNotFound {
		return nil, nil
	}

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("failed to get monitor: %s (status %d)", string(body), resp.StatusCode)
	}

	var result Monitor
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &result, nil
}

// GetMonitorByName retrieves a monitor by name
func (c *APIClient) GetMonitorByName(ctx context.Context, name string) (*Monitor, error) {
	// First, get all monitors
	resp, err := c.doRequest(ctx, "GET", "/status/api/monitors", nil)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("failed to list monitors: %s (status %d)", string(body), resp.StatusCode)
	}

	var monitors []Monitor
	if err := json.NewDecoder(resp.Body).Decode(&monitors); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	// Find monitor by name
	for _, m := range monitors {
		if m.Name == name {
			return &m, nil
		}
	}

	return nil, nil
}

// UpdateMonitor updates an existing monitor
func (c *APIClient) UpdateMonitor(ctx context.Context, id int, monitor UpdateMonitorRequest) (*Monitor, error) {
	resp, err := c.doRequest(ctx, "PUT", fmt.Sprintf("/status/api/monitors/%d", id), monitor)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("failed to update monitor: %s (status %d)", string(body), resp.StatusCode)
	}

	var result Monitor
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &result, nil
}

// DeleteMonitor deletes a monitor
func (c *APIClient) DeleteMonitor(ctx context.Context, id int) error {
	resp, err := c.doRequest(ctx, "DELETE", fmt.Sprintf("/status/api/monitors/%d", id), nil)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusNoContent {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("failed to delete monitor: %s (status %d)", string(body), resp.StatusCode)
	}

	return nil
}