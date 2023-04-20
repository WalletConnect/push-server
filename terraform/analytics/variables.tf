variable "app_name" {
  description = "The name of the application"
  type        = string
}

variable "environment" {
  description = "The environment to deploy into, should typically dev|staging|prod"
  type        = string
}
