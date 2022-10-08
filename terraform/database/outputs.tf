locals {
  database_host = aws_rds_cluster.postgres_cluster.endpoint
  database_port = aws_rds_cluster.postgres_cluster.port
  database_name = aws_rds_cluster.postgres_cluster.database_name

  database_user = onepassword_item.database_password.username
  database_pass = onepassword_item.database_password.password
}

output "database_url" {
  value = "postgres://${local.database_user}:${local.database_pass}@${local.database_host}:${local.database_port}/${local.database_name}"
}