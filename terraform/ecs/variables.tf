variable "region" {
  type = string
}

variable "app_name" {
  type = string
}

variable "image" {
  type = string
}

variable "database_url" {
  type = string
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

variable "fcm_api_key" {
  type      = string
  sensitive = true
}