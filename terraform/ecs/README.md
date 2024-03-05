<!-- BEGIN_TF_DOCS -->

## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | ~> 1.0 |
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | ~> 4.31 |
## Providers

| Name | Version |
|------|---------|
| <a name="provider_aws"></a> [aws](#provider\_aws) | ~> 4.31 |
## Modules

No modules.

## Inputs
  | Name | Description | Type | Default | Required |
  |------|-------------|------|---------|:--------:|
      | <a name="input_acm_certificate_arn"></a> [acm\_certificate\_arn](#input\_acm\_certificate\_arn) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_allowed_origins"></a> [allowed\_origins](#input\_allowed\_origins) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_analytics_datalake_bucket_name"></a> [analytics\_datalake\_bucket\_name](#input\_analytics\_datalake\_bucket\_name) | The name of the bucket where the analytics data will be stored |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_analytics_geoip_db_bucket_name"></a> [analytics\_geoip\_db\_bucket\_name](#input\_analytics\_geoip\_db\_bucket\_name) | The name of the bucket where the geoip database is stored |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_analytics_geoip_db_key"></a> [analytics\_geoip\_db\_key](#input\_analytics\_geoip\_db\_key) | The key of the geoip database in the bucket |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_analytics_key_arn"></a> [analytics\_key\_arn](#input\_analytics\_key\_arn) | The ARN of the KMS key used to encrypt the analytics data |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_app_name"></a> [app\_name](#input\_app\_name) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_autoscaling_max_capacity"></a> [autoscaling\_max\_capacity](#input\_autoscaling\_max\_capacity) | n/a |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_autoscaling_min_capacity"></a> [autoscaling\_min\_capacity](#input\_autoscaling\_min\_capacity) | n/a |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_aws_otel_collector_ecr_repository_url"></a> [aws\_otel\_collector\_ecr\_repository\_url](#input\_aws\_otel\_collector\_ecr\_repository\_url) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_backup_acm_certificate_arn"></a> [backup\_acm\_certificate\_arn](#input\_backup\_acm\_certificate\_arn) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_backup_fqdn"></a> [backup\_fqdn](#input\_backup\_fqdn) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_backup_route53_zone_id"></a> [backup\_route53\_zone\_id](#input\_backup\_route53\_zone\_id) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_cloud_api_key"></a> [cloud\_api\_key](#input\_cloud\_api\_key) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_cloud_api_url"></a> [cloud\_api\_url](#input\_cloud\_api\_url) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_cpu"></a> [cpu](#input\_cpu) | n/a |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_database_url"></a> [database\_url](#input\_database\_url) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_desired_count"></a> [desired\_count](#input\_desired\_count) | n/a |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_environment"></a> [environment](#input\_environment) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_fqdn"></a> [fqdn](#input\_fqdn) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_image"></a> [image](#input\_image) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_image_version"></a> [image\_version](#input\_image\_version) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_jwt_secret"></a> [jwt\_secret](#input\_jwt\_secret) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_memory"></a> [memory](#input\_memory) | n/a |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_private_subnets"></a> [private\_subnets](#input\_private\_subnets) | n/a |  <pre lang="json">set(string)</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_prometheus_endpoint"></a> [prometheus\_endpoint](#input\_prometheus\_endpoint) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_public_subnets"></a> [public\_subnets](#input\_public\_subnets) | n/a |  <pre lang="json">set(string)</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_relay_public_key"></a> [relay\_public\_key](#input\_relay\_public\_key) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_route53_zone_id"></a> [route53\_zone\_id](#input\_route53\_zone\_id) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_telemetry_sample_ratio"></a> [telemetry\_sample\_ratio](#input\_telemetry\_sample\_ratio) | n/a |  <pre lang="json">number</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_tenant_database_url"></a> [tenant\_database\_url](#input\_tenant\_database\_url) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_vpc_cidr"></a> [vpc\_cidr](#input\_vpc\_cidr) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
      | <a name="input_vpc_id"></a> [vpc\_id](#input\_vpc\_id) | n/a |  <pre lang="json">string</pre> |  <pre lang="json">n/a</pre> |  yes |
## Outputs

| Name | Description |
|------|-------------|
| <a name="output_load_balancer_arn"></a> [load\_balancer\_arn](#output\_load\_balancer\_arn) | n/a |

<!-- END_TF_DOCS -->