resource "aws_kms_key" "analytics_bucket" {
  description             = "${var.app_name}.${var.environment} - analytics bucket encryption"
  enable_key_rotation     = true
  deletion_window_in_days = 10
}

resource "aws_kms_alias" "analytics_bucket" {
  target_key_id = aws_kms_key.analytics_bucket.id
  name          = "alias/analytics/${var.app_name}/${var.environment}"
}
