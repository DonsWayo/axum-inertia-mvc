# Makefile for Terraform Provider StatusMonitor

# Go parameters
GOCMD=go
GOBUILD=$(GOCMD) build
GOTEST=$(GOCMD) test
GOGET=$(GOCMD) get
BINARY_NAME=terraform-provider-statusmonitor
VERSION=0.1.0

# Terraform parameters
OS_ARCH=darwin_amd64
# For Apple Silicon Macs, use: darwin_arm64

# Installation directory
INSTALL_DIR=~/.terraform.d/plugins/hashicorp.com/edu/statusmonitor/$(VERSION)/$(OS_ARCH)

.PHONY: all build test clean install dev

all: test build

build:
	$(GOBUILD) -o $(BINARY_NAME) -v

test:
	$(GOTEST) -v ./...

clean:
	$(GOCMD) clean
	rm -f $(BINARY_NAME)

install: build
	mkdir -p $(INSTALL_DIR)
	cp $(BINARY_NAME) $(INSTALL_DIR)/

# Development build with debug symbols
dev:
	$(GOBUILD) -gcflags="all=-N -l" -o $(BINARY_NAME) -v

# Run the provider in debug mode
debug:
	dlv exec --accept-multiclient --continue --headless ./$(BINARY_NAME) -- -debug

# Format code
fmt:
	$(GOCMD) fmt ./...

# Run linters
lint:
	golangci-lint run

# Update dependencies
deps:
	$(GOCMD) mod download
	$(GOCMD) mod tidy

# Run example
example: install
	cd examples && terraform init && terraform plan