output "database_url" {
  value = aws_rds_cluster.postgres_cluster.endpoint
}