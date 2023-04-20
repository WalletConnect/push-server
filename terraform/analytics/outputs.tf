output "bucket-arn" {
  description = "The ARN of the analytics bucket."
  value       = aws_s3_bucket.analytics-data-lake_bucket.arn
}

output "bucket-name" {
  description = "The name of the analytics bucket."
  value       = aws_s3_bucket.analytics-data-lake_bucket.bucket
}

output "kms-key_arn" {
  description = "The ARN of the KMS key used to encrypt the analytics bucket."
  value       = aws_kms_key.analytics_bucket.arn
}
