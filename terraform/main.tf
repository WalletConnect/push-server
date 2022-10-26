locals {
  app_name = "push"
  fqdn     = terraform.workspace == "prod" ? var.public_url : "${terraform.workspace}.${var.public_url}"
}

data "assert_test" "workspace" {
  test  = terraform.workspace != "default"
  throw = "default workspace is not valid in this project"
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

module "ecs" {
  source = "./ecs"

  app_name                  = "${terraform.workspace}-${local.app_name}"
  prometheus_endpoint       = aws_prometheus_workspace.prometheus.prometheus_endpoint
  database_url              = "postgres://${module.database_cluster.cluster_master_username}:${module.database_cluster.cluster_master_password}@${module.database_cluster.cluster_endpoint}:${module.database_cluster.cluster_port}/postgres"
  image                     = "${data.aws_ecr_repository.repository.repository_url}:${var.image_version}"
  acm_certificate_arn       = module.dns.certificate_arn
  cpu                       = 512
  fqdn                      = local.fqdn
  memory                    = 1024
  private_subnets           = module.vpc.private_subnets
  public_subnets            = module.vpc.public_subnets
  region                    = var.region
  route53_zone_id           = module.dns.zone_id
  vpc_cidr                  = module.vpc.vpc_cidr_block
  vpc_id                    = module.vpc.vpc_id
  fcm_api_key               = var.fcm_api_key
  apns_topic                = var.apns_topic
  apns_certificate          = var.apns_certificate
  apns_certificate_password = var.apns_certificate_password
}

data "aws_ecr_repository" "repository" {
  name = "echo-server"
}

resource "aws_prometheus_workspace" "prometheus" {
  alias = "prometheus-${terraform.workspace}-${local.app_name}"
}