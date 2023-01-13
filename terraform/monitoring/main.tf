locals {
  # Turns the arn into the format expected by
  # the Grafana provider e.g.
  # net/prod-relay-load-balancer/e9a51c46020a0f85
  load_balancer                 = join("/", slice(split("/", var.load_balancer_arn), 1, 4))
  opsgenie_notification_channel = "l_iaPw6nk"
  notifications = (
    var.environment == "prod" ?
    "[{\"uid\": \"${local.opsgenie_notification_channel}\"}]" :
    "[]"
  )
}

resource "grafana_data_source" "prometheus" {
  type = "prometheus"
  name = "${var.app_name}-amp"
  url  = "https://aps-workspaces.eu-central-1.amazonaws.com/workspaces/${var.prometheus_workspace_id}/"

  json_data {
    http_method     = "GET"
    sigv4_auth      = true
    sigv4_auth_type = "workspace-iam-role"
    sigv4_region    = "eu-central-1"
  }
}

resource "grafana_data_source" "cloudwatch" {
  type = "cloudwatch"
  name = "${var.app_name}-cloudwatch"

  json_data {
    default_region = "eu-central-1"
  }
}

resource "grafana_dashboard" "at_a_glance" {
  overwrite = true
  message   = "Updated by Terraform"
  config_json = jsonencode({
    annotations : {
      list : [
        {
          builtIn : 1,
          datasource : "-- Grafana --",
          enable : true,
          hide : true,
          iconColor : "rgba(0, 211, 255, 1)",
          name : "Annotations & Alerts",
          target : {
            limit : 100,
            matchAny : false,
            tags : [],
            type : "dashboard"
          },
          type : "dashboard"
        }
      ]
    },
    editable : true,
    fiscalYearStartMonth : 0,
    graphTooltip : 0,
    id : 19,
    links : [],
    liveNow : false,
    panels : [
      {
        "datasource" : {
          "type" : "cloudwatch",
          "uid" : grafana_data_source.cloudwatch.uid
        },
        "fieldConfig" : {
          "defaults" : {
            "color" : {
              "mode" : "palette-classic"
            },
            "custom" : {
              "axisLabel" : "",
              "axisPlacement" : "auto",
              "barAlignment" : 0,
              "drawStyle" : "line",
              "fillOpacity" : 0,
              "gradientMode" : "none",
              "hideFrom" : {
                "legend" : false,
                "tooltip" : false,
                "viz" : false
              },
              "lineInterpolation" : "linear",
              "lineWidth" : 1,
              "pointSize" : 5,
              "scaleDistribution" : {
                "type" : "linear"
              },
              "showPoints" : "auto",
              "spanNulls" : false,
              "stacking" : {
                "group" : "A",
                "mode" : "none"
              },
              "thresholdsStyle" : {
                "mode" : "off"
              }
            },
            "mappings" : [],
            "thresholds" : {
              "mode" : "absolute",
              "steps" : [
                {
                  "color" : "green",
                  "value" : null
                },
                {
                  "color" : "red",
                  "value" : 80
                }
              ]
            }
          },
          "overrides" : []
        },
        "gridPos" : {
          "h" : 9,
          "w" : 7,
          "x" : 0,
          "y" : 0
        },
        "id" : 2,
        "options" : {
          "legend" : {
            "calcs" : [],
            "displayMode" : "list",
            "placement" : "bottom"
          },
          "tooltip" : {
            "mode" : "single",
            "sort" : "none"
          }
        },
        "targets" : [
          {
            "alias" : "",
            "datasource" : {
              "type" : "cloudwatch",
              "uid" : grafana_data_source.cloudwatch.uid
            },
            "dimensions" : {
              "LoadBalancer" : local.load_balancer
            },
            "expression" : "",
            "id" : "",
            "matchExact" : true,
            "metricEditorMode" : 0,
            "metricName" : "RequestCount",
            "metricQueryType" : 0,
            "namespace" : "AWS/ApplicationELB",
            "period" : "",
            "queryMode" : "Metrics",
            "refId" : "A",
            "region" : "default",
            "sqlExpression" : "",
            "statistic" : "Sum"
          }
        ],
        "title" : "Requests",
        "type" : "timeseries"
      },
      {
        "alert" : {
          "alertRuleTags" : {},
          "conditions" : [
            {
              "evaluator" : {
                "params" : [
                  5
                ],
                "type" : "gt"
              },
              "operator" : {
                "type" : "and"
              },
              "query" : {
                "params" : [
                  "A",
                  "5m",
                  "now"
                ]
              },
              "reducer" : {
                "params" : [],
                "type" : "sum"
              },
              "type" : "query"
            },
            {
              "evaluator" : {
                "params" : [
                  5
                ],
                "type" : "gt"
              },
              "operator" : {
                "type" : "or"
              },
              "query" : {
                "params" : [
                  "B",
                  "5m",
                  "now"
                ]
              },
              "reducer" : {
                "params" : [],
                "type" : "sum"
              },
              "type" : "query"
            }
          ],
          "executionErrorState" : "alerting",
          "for" : "5m",
          "frequency" : "1m",
          "handler" : 1,
          "name" : "${var.environment} Echo Server 5XX alert",
          "noDataState" : "no_data",
          "notifications" : []
        },
        "datasource" : {
          "type" : "cloudwatch",
          "uid" : grafana_data_source.cloudwatch.uid
        },
        "fieldConfig" : {
          "defaults" : {
            "color" : {
              "mode" : "palette-classic"
            },
            "custom" : {
              "axisLabel" : "",
              "axisPlacement" : "auto",
              "barAlignment" : 0,
              "drawStyle" : "line",
              "fillOpacity" : 0,
              "gradientMode" : "none",
              "hideFrom" : {
                "legend" : false,
                "tooltip" : false,
                "viz" : false
              },
              "lineInterpolation" : "linear",
              "lineWidth" : 1,
              "pointSize" : 5,
              "scaleDistribution" : {
                "type" : "linear"
              },
              "showPoints" : "auto",
              "spanNulls" : false,
              "stacking" : {
                "group" : "A",
                "mode" : "none"
              },
              "thresholdsStyle" : {
                "mode" : "off"
              }
            },
            "mappings" : [],
            "thresholds" : {
              "mode" : "absolute",
              "steps" : [
                {
                  "color" : "green",
                  "value" : null
                },
                {
                  "color" : "red",
                  "value" : 80
                }
              ]
            }
          },
          "overrides" : []
        },
        "gridPos" : {
          "h" : 9,
          "w" : 7,
          "x" : 7,
          "y" : 0
        },
        "id" : 3,
        "options" : {
          "legend" : {
            "calcs" : [],
            "displayMode" : "list",
            "placement" : "bottom"
          },
          "tooltip" : {
            "mode" : "single",
            "sort" : "none"
          }
        },
        "targets" : [
          {
            "alias" : "",
            "datasource" : {
              "type" : "cloudwatch",
              "uid" : grafana_data_source.cloudwatch.uid
            },
            "dimensions" : {
              "LoadBalancer" : local.load_balancer
            },
            "expression" : "",
            "id" : "",
            "matchExact" : true,
            "metricEditorMode" : 0,
            "metricName" : "HTTPCode_ELB_5XX_Count",
            "metricQueryType" : 0,
            "namespace" : "AWS/ApplicationELB",
            "period" : "",
            "queryMode" : "Metrics",
            "refId" : "A",
            "region" : "default",
            "sqlExpression" : "",
            "statistic" : "Sum"
          },
          {
            "alias" : "",
            "datasource" : {
              "type" : "cloudwatch",
              "uid" : grafana_data_source.cloudwatch.uid
            },
            "dimensions" : {
              "LoadBalancer" : local.load_balancer
            },
            "expression" : "",
            "id" : "",
            "matchExact" : true,
            "metricEditorMode" : 0,
            "metricName" : "HTTPCode_Target_5XX_Count",
            "metricQueryType" : 0,
            "namespace" : "AWS/ApplicationELB",
            "period" : "",
            "queryMode" : "Metrics",
            "refId" : "B",
            "region" : "default",
            "sqlExpression" : "",
            "statistic" : "Sum"
          }
        ],
        "title" : "5XX",
        "type" : "timeseries"
      },
      {
        "datasource" : {
          "type" : "cloudwatch",
          "uid" : grafana_data_source.cloudwatch.uid
        },
        "fieldConfig" : {
          "defaults" : {
            "color" : {
              "mode" : "palette-classic"
            },
            "custom" : {
              "axisLabel" : "",
              "axisPlacement" : "auto",
              "barAlignment" : 0,
              "drawStyle" : "line",
              "fillOpacity" : 0,
              "gradientMode" : "none",
              "hideFrom" : {
                "legend" : false,
                "tooltip" : false,
                "viz" : false
              },
              "lineInterpolation" : "linear",
              "lineWidth" : 1,
              "pointSize" : 5,
              "scaleDistribution" : {
                "type" : "linear"
              },
              "showPoints" : "auto",
              "spanNulls" : false,
              "stacking" : {
                "group" : "A",
                "mode" : "none"
              },
              "thresholdsStyle" : {
                "mode" : "off"
              }
            },
            "mappings" : [],
            "thresholds" : {
              "mode" : "absolute",
              "steps" : [
                {
                  "color" : "green",
                  "value" : null
                },
                {
                  "color" : "red",
                  "value" : 80
                }
              ]
            }
          },
          "overrides" : []
        },
        "gridPos" : {
          "h" : 9,
          "w" : 7,
          "x" : 14,
          "y" : 0
        },
        "id" : 4,
        "options" : {
          "legend" : {
            "calcs" : [],
            "displayMode" : "list",
            "placement" : "bottom"
          },
          "tooltip" : {
            "mode" : "single",
            "sort" : "none"
          }
        },
        "targets" : [
          {
            "alias" : "",
            "datasource" : {
              "type" : "cloudwatch",
              "uid" : grafana_data_source.cloudwatch.uid
            },
            "dimensions" : {
              "LoadBalancer" : local.load_balancer
            },
            "expression" : "",
            "id" : "",
            "matchExact" : true,
            "metricEditorMode" : 0,
            "metricName" : "HTTPCode_ELB_4XX_Count",
            "metricQueryType" : 0,
            "namespace" : "AWS/ApplicationELB",
            "period" : "",
            "queryMode" : "Metrics",
            "refId" : "A",
            "region" : "default",
            "sqlExpression" : "",
            "statistic" : "Sum"
          },
          {
            "alias" : "",
            "datasource" : {
              "type" : "cloudwatch",
              "uid" : grafana_data_source.cloudwatch.uid
            },
            "dimensions" : {
              "LoadBalancer" : local.load_balancer
            },
            "expression" : "",
            "id" : "",
            "matchExact" : true,
            "metricEditorMode" : 0,
            "metricName" : "HTTPCode_Target_4XX_Count",
            "metricQueryType" : 0,
            "namespace" : "AWS/ApplicationELB",
            "period" : "",
            "queryMode" : "Metrics",
            "refId" : "B",
            "region" : "default",
            "sqlExpression" : "",
            "statistic" : "Sum"
          }
        ],
        "title" : "4XX",
        "type" : "timeseries"
      }
    ],
    schemaVersion : 36,
    style : "dark",
    tags : [],
    templating : {
      list : []
    },
    time : {
      from : "now-6h",
      to : "now"
    },
    timepicker : {},
    timezone : "",
    title : "${var.app_name}",
    uid : "${var.app_name}",
    version : 1,
    weekStart : ""
  })
}
