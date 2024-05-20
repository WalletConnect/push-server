variable "app_name" {
  type = string
}

variable "environment" {
  type = string
}

variable "prometheus_workspace_id" {
  type = string
}

variable "load_balancer_arn" {
  type = string
}

variable "notification_channels" {
  description = "The notification channels to send alerts to"
  type        = list(any)
}
