variable "region" {
  type    = string
  default = "eu-central-1"
}

variable "azs" {
  type    = list(string)
  default = ["eu-central-1a", "eu-central-1b", "eu-central-1c"]
}

variable "public_url" {
  type    = string
  default = "echo.walletconnect.com"
}

variable "grafana_endpoint" {
  type = string
}

variable "image_url" {
  type    = string
  default = "ghcr.io/walletconnect/echo-server"
}

variable "image_version" {
  type    = string
  default = ""
}

variable "tenant_database_url" {
  type      = string
  sensitive = true
}