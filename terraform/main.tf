locals {
  app_name         = "echo-server"
  fqdn             = terraform.workspace == "prod" ? var.public_url : "${terraform.workspace}.${var.public_url}"
}

data "assert_test" "workspace" {
  test  = terraform.workspace != "default"
  throw = "default workspace is not valid in this project"
}

module "tags" {
  source = "github.com/WalletConnect/terraform-modules/modules/tags"

  application = local.app_name
  env         = terraform.workspace
}

module "dns" {
  source = "github.com/WalletConnect/terraform-modules/modules/dns"

  hosted_zone_name = var.public_url
  fqdn             = local.fqdn
}

data "aws_ecr_repository" "repository" {
  name = local.app_name
}

resource "aws_prometheus_workspace" "prometheus" {
  alias = "prometheus-${terraform.workspace}-${local.app_name}"
}