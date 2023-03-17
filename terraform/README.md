# Echo - Push Server

<!-- BEGINNING OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
## Requirements

| Name | Version |
|------|---------|
| <a name="requirement_terraform"></a> [terraform](#requirement\_terraform) | ~> 1.0 |
| <a name="requirement_aws"></a> [aws](#requirement\_aws) | ~> 4.31 |
| <a name="requirement_github"></a> [github](#requirement\_github) | 5.7.0 |
| <a name="requirement_grafana"></a> [grafana](#requirement\_grafana) | ~> 1.28 |
| <a name="requirement_random"></a> [random](#requirement\_random) | 3.4.3 |

## Providers

| Name | Version |
|------|---------|
| <a name="provider_assert"></a> [assert](#provider\_assert) | n/a |
| <a name="provider_aws"></a> [aws](#provider\_aws) | ~> 4.31 |
| <a name="provider_github"></a> [github](#provider\_github) | 5.7.0 |

## Modules

| Name | Source | Version |
|------|--------|---------|
| <a name="module_analytics"></a> [analytics](#module\_analytics) | ./analytics | n/a |
| <a name="module_database_cluster"></a> [database\_cluster](#module\_database\_cluster) | terraform-aws-modules/rds-aurora/aws | n/a |
| <a name="module_dns"></a> [dns](#module\_dns) | github.com/WalletConnect/terraform-modules.git//modules/dns | n/a |
| <a name="module_ecs"></a> [ecs](#module\_ecs) | ./ecs | n/a |
| <a name="module_monitoring"></a> [monitoring](#module\_monitoring) | ./monitoring | n/a |
| <a name="module_tags"></a> [tags](#module\_tags) | github.com/WalletConnect/terraform-modules.git//modules/tags | n/a |
| <a name="module_tenant_database_cluster"></a> [tenant\_database\_cluster](#module\_tenant\_database\_cluster) | terraform-aws-modules/rds-aurora/aws | n/a |
| <a name="module_vpc"></a> [vpc](#module\_vpc) | terraform-aws-modules/vpc/aws | n/a |

## Resources

| Name | Type |
|------|------|
| [aws_prometheus_workspace.prometheus](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/prometheus_workspace) | resource |
| [assert_test.workspace](https://registry.terraform.io/providers/bwoznicki/assert/latest/docs/data-sources/test) | data source |
| [aws_ecr_repository.aws_otel_collector](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/ecr_repository) | data source |
| [aws_ecr_repository.repository](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/data-sources/ecr_repository) | data source |
| [github_release.latest_release](https://registry.terraform.io/providers/integrations/github/5.7.0/docs/data-sources/release) | data source |

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| <a name="input_azs"></a> [azs](#input\_azs) | n/a | `list(string)` | <pre>[<br>  "eu-central-1a",<br>  "eu-central-1b",<br>  "eu-central-1c"<br>]</pre> | no |
| <a name="input_geoip_db_key"></a> [geoip\_db\_key](#input\_geoip\_db\_key) | The key to the GeoIP database | `string` | `"GeoLite2-City.mmdb"` | no |
| <a name="input_grafana_endpoint"></a> [grafana\_endpoint](#input\_grafana\_endpoint) | n/a | `string` | n/a | yes |
| <a name="input_image_version"></a> [image\_version](#input\_image\_version) | n/a | `string` | `""` | no |
| <a name="input_public_url"></a> [public\_url](#input\_public\_url) | n/a | `string` | `"echo.walletconnect.com"` | no |
| <a name="input_region"></a> [region](#input\_region) | n/a | `string` | `"eu-central-1"` | no |

## Outputs

No outputs.
<!-- END OF PRE-COMMIT-TERRAFORM DOCS HOOK -->
