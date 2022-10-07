resource "onepassword_item" "database_password" {
  vault = var.onepassword_vault_id

  title    = "postgres/${terraform.workspace}/${var.name}"
  category = "login"
  username = "postgres"

  password_recipe {
    length  = 25
    digits  = true
    letters = true
    symbols = true
  }
}

resource "aws_rds_cluster" "postgres_cluster" {
  cluster_identifier = "${terraform.workspace}-${var.name}"
  engine             = "aurora-postgresql"
  engine_mode        = "provisioned"
  engine_version     = "13.6"
  database_name      = "postgres"
  master_username    = onepassword_item.database_password.username
  master_password    = onepassword_item.database_password.password

  serverlessv2_scaling_configuration {
    max_capacity = 1.0
    min_capacity = 0.5
  }
}

resource "aws_rds_cluster_instance" "postgres_default_instance" {
  cluster_identifier = aws_rds_cluster.postgres_cluster.id
  instance_class     = "db.serverless"
  engine             = aws_rds_cluster.postgres_cluster.engine
  engine_version     = aws_rds_cluster.postgres_cluster.engine_version
}