variable "region" {
  type = string
}

variable "app_name" {
  type = string
}

variable "environment" {
  type = string
}

variable "image" {
  type = string
}

variable "image_version" {
  type = string
}

variable "database_url" {
  type      = string
  sensitive = true
}

variable "tenant_database_url" {
  type      = string
  sensitive = true
}

variable "prometheus_endpoint" {
  type = string
}

variable "vpc_id" {
  type = string
}

variable "vpc_cidr" {
  type = string
}

variable "route53_zone_id" {
  type = string
}

variable "fqdn" {
  type = string
}

variable "acm_certificate_arn" {
  type = string
}

variable "public_subnets" {
  type = set(string)
}

variable "private_subnets" {
  type = set(string)
}

variable "cpu" {
  type = number
}

variable "memory" {
  type = number
}

variable "telemetry_sample_ratio" {
  type = number
}

variable "aws_otel_collector_ecr_repository_url" {
  type = string
}