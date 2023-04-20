# Terraform Configuration
terraform {
  required_version = "~> 1.0"

  backend "remote" {
    hostname     = "app.terraform.io"
    organization = "wallet-connect"
    workspaces {
      prefix = "echo-server-"
    }
  }

  required_providers {
    assert = {
      source = "bwoznicki/assert"
    }
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.31"
    }
    grafana = {
      source  = "grafana/grafana"
      version = "~> 1.28"
    }
    random = {
      source  = "hashicorp/random"
      version = "3.4.3"
    }
    github = {
      source  = "integrations/github"
      version = "5.7.0"
    }
  }
}