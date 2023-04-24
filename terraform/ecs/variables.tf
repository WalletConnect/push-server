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

variable "allowed_origins" {
  type = string
}

variable "analytics_datalake_bucket_name" {
  description = "The name of the bucket where the analytics data will be stored"
  type        = string
}

variable "analytics_geoip_db_bucket_name" {
  description = "The name of the bucket where the geoip database is stored"
  type        = string
}

variable "analytics_geoip_db_key" {
  description = "The key of the geoip database in the bucket"
  type        = string
}

variable "analytics_key_arn" {
  description = "The ARN of the KMS key used to encrypt the analytics data"
  type        = string
}

variable "desired_count" {
  type = number
}

variable "autoscaling_max_capacity" {
  type = number
}

variable "autoscaling_min_capacity" {
  type = number
}

variable "cloud_api_key" {
  type      = string
  sensitive = true
}

variable "cloud_api_url" {
  type = string
}