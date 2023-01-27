locals {
  app_name            = "push"
  fqdn                = terraform.workspace == "prod" ? var.public_url : "${terraform.workspace}.${var.public_url}"
  latest_release_name = data.github_release.latest_release.name
  version             = coalesce(var.image_version, substr(local.latest_release_name, 1, length(local.latest_release_name)))
  database_url = "postgres://${module.database_cluster.cluster_master_username}:${module.database_cluster.cluster_master_password}@${module.database_cluster.cluster_endpoint}:${module.database_cluster.cluster_port}/postgres"
}

data "assert_test" "workspace" {
  test  = terraform.workspace != "default"
  throw = "default workspace is not valid in this project"
}

data "github_release" "latest_release" {
  repository  = "echo-server"
  owner       = "walletconnect"
  retrieve_by = "latest"
}

module "vpc" {
  source = "terraform-aws-modules/vpc/aws"
  name   = "${terraform.workspace}-${local.app_name}"

  cidr = "10.0.0.0/16"

  azs             = var.azs
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.4.0/24", "10.0.5.0/24", "10.0.6.0/24"]

  private_subnet_tags = {
    Visibility = "private"
  }
  public_subnet_tags = {
    Visibility = "public"
  }

  enable_dns_support     = true
  enable_dns_hostnames   = true
  enable_nat_gateway     = true
  single_nat_gateway     = true
  one_nat_gateway_per_az = false
}

module "tags" {
  source = "github.com/WalletConnect/terraform-modules.git//modules/tags"

  application = local.app_name
  env         = terraform.workspace
}

module "dns" {
  source = "github.com/WalletConnect/terraform-modules.git//modules/dns"

  hosted_zone_name = var.public_url
  fqdn             = local.fqdn
}

module "database_cluster" {
  source = "terraform-aws-modules/rds-aurora/aws"

  name           = "${terraform.workspace}-${local.app_name}-database"
  engine         = "aurora-postgresql"
  engine_version = "13.6"
  engine_mode    = "provisioned"
  instance_class = "db.serverless"
  instances = {
    1 = {}
  }

  database_name = "postgres"

  vpc_id  = module.vpc.vpc_id
  subnets = module.vpc.private_subnets

  allowed_cidr_blocks = [module.vpc.vpc_cidr_block]

  storage_encrypted = true
  apply_immediately = true

  allow_major_version_upgrade = true

  serverlessv2_scaling_configuration = {
    min_capacity = 2
    max_capacity = 10
  }
}

resource "aws_secretsmanager_secret" "database_url" {
  name = "${local.app_name}-${terraform.workspace}-database-url"
}

resource "aws_secretsmanager_secret_version" "database_url" {
  secret_id     = aws_secretsmanager_secret.database_url.id
  secret_string = local.database_url
}

data "aws_secretsmanager_secret" "tenant_db_url" {
  name = "batcave-${terraform.workspace}-database-url"
}

data "aws_secretsmanager_secret_version" "tenant_db_url" {
  secret_id = data.aws_secretsmanager_secret.tenant_db_url.id
}

module "ecs" {
  source = "./ecs"

  app_name               = "${terraform.workspace}-${local.app_name}"
  environment            = terraform.workspace
  prometheus_endpoint    = aws_prometheus_workspace.prometheus.prometheus_endpoint
  database_url           = local.database_url
  tenant_database_url    = data.aws_secretsmanager_secret_version.tenant_db_url.secret_string
  image                  = "${data.aws_ecr_repository.repository.repository_url}:${local.version}"
  image_version          = local.version
  acm_certificate_arn    = module.dns.certificate_arn
  cpu                    = 512
  fqdn                   = local.fqdn
  memory                 = 1024
  private_subnets        = module.vpc.private_subnets
  public_subnets         = module.vpc.public_subnets
  region                 = var.region
  route53_zone_id        = module.dns.zone_id
  vpc_cidr               = module.vpc.vpc_cidr_block
  vpc_id                 = module.vpc.vpc_id
  telemetry_sample_ratio = terraform.workspace == "prod" ? 0.25 : 1.0

  aws_otel_collector_ecr_repository_url = data.aws_ecr_repository.aws_otel_collector.repository_url
}

module "monitoring" {
  source = "./monitoring"

  app_name                = "${terraform.workspace}-${local.app_name}"
  prometheus_workspace_id = aws_prometheus_workspace.prometheus.id
  load_balancer_arn       = module.ecs.load_balancer_arn
  environment             = terraform.workspace
}

data "aws_ecr_repository" "repository" {
  name = "echo-server"
}

data "aws_ecr_repository" "aws_otel_collector" {
  name = "aws-otel-collector"
}

resource "aws_prometheus_workspace" "prometheus" {
  alias = "prometheus-${terraform.workspace}-${local.app_name}"
}
