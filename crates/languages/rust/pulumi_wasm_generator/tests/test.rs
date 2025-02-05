use anyhow::{Context, Result};
use assert_cmd::assert::OutputAssertExt;
use pulumi_wasm_generator::generate_combined;
use rinja::Template;
use std::fs;
use std::fs::{File, FileTimes};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
// DO NOT EDIT - START

#[test]
#[cfg_attr(not(feature = "generator_array-of-enum-map"), ignore)]
fn array_of_enum_map() -> Result<()> {
    run_pulumi_generator_test("array-of-enum-map", "array-of-enum-map", None)
}

#[test]
#[cfg_attr(not(feature = "generator_azure-native-nested-types"), ignore)]
fn azure_native_nested_types() -> Result<()> {
    run_pulumi_generator_test(
        "azure-native-nested-types",
        "azure-native-nested-types",
        None,
    )
}

#[test]
#[cfg_attr(not(feature = "generator_cloudflare"), ignore)]
fn cloudflare() -> Result<()> {
    run_pulumi_generator_test("cloudflare", "cloudflare", None)
}

#[test]
#[cfg_attr(not(feature = "generator_cyclic-types"), ignore)]
fn cyclic_types() -> Result<()> {
    run_pulumi_generator_test("cyclic-types", "cyclic-types", None)
}

#[test]
#[cfg_attr(not(feature = "generator_different-enum"), ignore)]
fn different_enum() -> Result<()> {
    run_pulumi_generator_test("different-enum", "different-enum", None)
}

#[test]
#[cfg_attr(not(feature = "generator_docker"), ignore)]
fn docker() -> Result<()> {
    run_pulumi_generator_test("docker", "docker", None)
}

#[test]
#[cfg_attr(not(feature = "generator_functions-secrets"), ignore)]
fn functions_secrets() -> Result<()> {
    run_pulumi_generator_test("functions-secrets", "functions-secrets", None)
}

#[test]
#[cfg_attr(not(feature = "generator_mini-awsnative"), ignore)]
fn mini_awsnative() -> Result<()> {
    run_pulumi_generator_test("mini-awsnative", "mini-awsnative", None)
}

#[test]
#[cfg_attr(not(feature = "generator_nested-module"), ignore)]
fn nested_module() -> Result<()> {
    run_pulumi_generator_test("nested-module", "nested-module", None)
}

#[test]
#[cfg_attr(not(feature = "generator_nested-module-thirdparty"), ignore)]
fn nested_module_thirdparty() -> Result<()> {
    run_pulumi_generator_test("nested-module-thirdparty", "nested-module-thirdparty", None)
}

#[test]
#[cfg_attr(not(feature = "generator_output-funcs"), ignore)]
fn output_funcs() -> Result<()> {
    run_pulumi_generator_test("output-funcs", "output-funcs", None)
}

#[test]
#[cfg_attr(not(feature = "generator_output-funcs-edgeorder"), ignore)]
fn output_funcs_edgeorder() -> Result<()> {
    run_pulumi_generator_test("output-funcs-edgeorder", "output-funcs-edgeorder", None)
}

#[test]
#[cfg_attr(not(feature = "generator_plain-object-defaults"), ignore)]
fn plain_object_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-defaults", "plain-object-defaults", None)
}

#[test]
#[cfg_attr(not(feature = "generator_plain-object-disable-defaults"), ignore)]
fn plain_object_disable_defaults() -> Result<()> {
    run_pulumi_generator_test(
        "plain-object-disable-defaults",
        "plain-object-disable-defaults",
        None,
    )
}

#[test]
#[cfg_attr(not(feature = "generator_random"), ignore)]
fn random() -> Result<()> {
    run_pulumi_generator_test("random", "random", None)
}

#[test]
#[cfg_attr(not(feature = "generator_reserved_names"), ignore)]
fn reserved_names() -> Result<()> {
    run_pulumi_generator_test("reserved_names", "reserved_names", None)
}

#[test]
#[cfg_attr(not(feature = "generator_unions-inline"), ignore)]
fn unions_inline() -> Result<()> {
    run_pulumi_generator_test("unions-inline", "unions-inline", None)
}

#[test]
#[cfg_attr(not(feature = "generator_unions-inside-arrays"), ignore)]
fn unions_inside_arrays() -> Result<()> {
    run_pulumi_generator_test("unions-inside-arrays", "unions-inside-arrays", None)
}

#[test]
#[cfg_attr(not(feature = "generator_workarounds"), ignore)]
fn workarounds() -> Result<()> {
    run_pulumi_generator_test("workarounds", "workarounds", None)
}

#[test]
#[cfg_attr(not(feature = "generator_aws-0"), ignore)]
fn aws_0() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-0",
        Some(&[
            "accessanalyzer",
            "account",
            "acm",
            "acmpca",
            "alb",
            "amp",
            "amplify",
            "apigateway",
            "apigatewayv2",
            "appautoscaling",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-1"), ignore)]
fn aws_1() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-1",
        Some(&[
            "appconfig",
            "appfabric",
            "appflow",
            "appintegrations",
            "applicationinsights",
            "applicationloadbalancing",
            "appmesh",
            "apprunner",
            "appstream",
            "appsync",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-2"), ignore)]
fn aws_2() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-2",
        Some(&[
            "athena",
            "auditmanager",
            "autoscaling",
            "autoscalingplans",
            "backup",
            "batch",
            "bcmdata",
            "bedrock",
            "bedrockfoundation",
            "bedrockmodel",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-3"), ignore)]
fn aws_3() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-3",
        Some(&[
            "budgets",
            "cfg",
            "chatbot",
            "chime",
            "chimesdkmediapipelines",
            "cleanrooms",
            "cloud9",
            "cloudcontrol",
            "cloudformation",
            "cloudfront",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-4"), ignore)]
fn aws_4() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-4",
        Some(&[
            "cloudhsmv2",
            "cloudsearch",
            "cloudtrail",
            "cloudwatch",
            "codeartifact",
            "codebuild",
            "codecatalyst",
            "codecommit",
            "codeconnections",
            "codedeploy",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-5"), ignore)]
fn aws_5() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-5",
        Some(&[
            "codeguruprofiler",
            "codegurureviewer",
            "codepipeline",
            "codestarconnections",
            "codestarnotifications",
            "cognito",
            "comprehend",
            "computeoptimizer",
            "config",
            "connect",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-6"), ignore)]
fn aws_6() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-6",
        Some(&[
            "controltower",
            "costexplorer",
            "costoptimizationhub",
            "cur",
            "customerprofiles",
            "dataexchange",
            "datapipeline",
            "datasync",
            "datazone",
            "dax",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-7"), ignore)]
fn aws_7() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-7",
        Some(&[
            "detective",
            "devicefarm",
            "devopsguru",
            "directconnect",
            "directoryservice",
            "dlm",
            "dms",
            "docdb",
            "drs",
            "dynamodb",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-8"), ignore)]
fn aws_8() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-8",
        Some(&[
            "ebs",
            "ec2",
            "ec2clientvpn",
            "ec2transitgateway",
            "ecr",
            "ecrpublic",
            "ecs",
            "efs",
            "eks",
            "elasticache",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-9"), ignore)]
fn aws_9() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-9",
        Some(&[
            "elasticbeanstalk",
            "elasticsearch",
            "elastictranscoder",
            "elb",
            "emr",
            "emrcontainers",
            "emrserverless",
            "evidently",
            "finspace",
            "fis",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-10"), ignore)]
fn aws_10() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-10",
        Some(&[
            "fms",
            "fsx",
            "gamelift",
            "glacier",
            "globalaccelerator",
            "glue",
            "grafana",
            "guardduty",
            "iam",
            "identitystore",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-11"), ignore)]
fn aws_11() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-11",
        Some(&[
            "imagebuilder",
            "index",
            "inspector",
            "inspector2",
            "iot",
            "ivs",
            "ivschat",
            "kendra",
            "keyspaces",
            "kinesis",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-12"), ignore)]
fn aws_12() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-12",
        Some(&[
            "kinesisanalyticsv2",
            "kms",
            "lakeformation",
            "lambda",
            "lb",
            "lex",
            "licensemanager",
            "lightsail",
            "location",
            "m2",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-13"), ignore)]
fn aws_13() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-13",
        Some(&[
            "macie",
            "macie2",
            "mediaconvert",
            "medialive",
            "mediapackage",
            "mediastore",
            "memorydb",
            "mq",
            "msk",
            "mskconnect",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-14"), ignore)]
fn aws_14() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-14",
        Some(&[
            "mwaa",
            "neptune",
            "networkfirewall",
            "networkmanager",
            "networkmonitor",
            "oam",
            "opensearch",
            "opensearchingest",
            "opsworks",
            "organizations",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-15"), ignore)]
fn aws_15() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-15",
        Some(&[
            "outposts",
            "paymentcryptography",
            "pinpoint",
            "pipes",
            "polly",
            "pricing",
            "qldb",
            "quicksight",
            "ram",
            "rbin",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-16"), ignore)]
fn aws_16() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-16",
        Some(&[
            "rds",
            "redshift",
            "redshiftdata",
            "redshiftserverless",
            "rekognition",
            "resiliencehub",
            "resourceexplorer",
            "resourcegroups",
            "resourcegroupstaggingapi",
            "rolesanywhere",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-17"), ignore)]
fn aws_17() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-17",
        Some(&[
            "route53",
            "route53domains",
            "route53recoverycontrol",
            "route53recoveryreadiness",
            "rum",
            "s3",
            "s3control",
            "s3outposts",
            "s3tables",
            "sagemaker",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-18"), ignore)]
fn aws_18() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-18",
        Some(&[
            "scheduler",
            "schemas",
            "secretsmanager",
            "securityhub",
            "securitylake",
            "serverlessrepository",
            "servicecatalog",
            "servicediscovery",
            "servicequotas",
            "ses",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-19"), ignore)]
fn aws_19() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-19",
        Some(&[
            "sesv2",
            "sfn",
            "shield",
            "signer",
            "simpledb",
            "sns",
            "sqs",
            "ssm",
            "ssmcontacts",
            "ssmincidents",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-20"), ignore)]
fn aws_20() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-20",
        Some(&[
            "ssoadmin",
            "storagegateway",
            "swf",
            "synthetics",
            "timestreaminfluxdb",
            "timestreamwrite",
            "transcribe",
            "transfer",
            "verifiedaccess",
            "verifiedpermissions",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_aws-21"), ignore)]
fn aws_21() -> Result<()> {
    run_pulumi_generator_test(
        "aws",
        "aws-21",
        Some(&[
            "vpc",
            "vpclattice",
            "waf",
            "wafregional",
            "wafv2",
            "worklink",
            "workspaces",
            "xray",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-0"), ignore)]
fn azure_0() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-0",
        Some(&[
            "aadb2c",
            "advisor",
            "analysisservices",
            "apimanagement",
            "appconfiguration",
            "appinsights",
            "appplatform",
            "appservice",
            "arc",
            "arckubernetes",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-1"), ignore)]
fn azure_1() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-1",
        Some(&[
            "arcmachine",
            "armmsi",
            "attestation",
            "authorization",
            "automanage",
            "automation",
            "avs",
            "backup",
            "batch",
            "billing",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-2"), ignore)]
fn azure_2() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-2",
        Some(&[
            "blueprint",
            "bot",
            "cdn",
            "chaosstudio",
            "cognitive",
            "communication",
            "compute",
            "confidentialledger",
            "config",
            "connections",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-3"), ignore)]
fn azure_3() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-3",
        Some(&[
            "consumption",
            "containerapp",
            "containerservice",
            "core",
            "cosmosdb",
            "costmanagement",
            "customip",
            "dashboard",
            "databasemigration",
            "databoxedge",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-4"), ignore)]
fn azure_4() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-4",
        Some(&[
            "databricks",
            "datadog",
            "datafactory",
            "dataprotection",
            "datashare",
            "desktopvirtualization",
            "devcenter",
            "devtest",
            "digitaltwins",
            "dns",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-5"), ignore)]
fn azure_5() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-5",
        Some(&[
            "domainservices",
            "dynatrace",
            "elasticcloud",
            "elasticsan",
            "eventgrid",
            "eventhub",
            "expressroute",
            "extendedlocation",
            "fabric",
            "fluidrelay",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-6"), ignore)]
fn azure_6() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-6",
        Some(&[
            "frontdoor",
            "graph",
            "hdinsight",
            "healthcare",
            "hpc",
            "hsm",
            "index",
            "iot",
            "iotcentral",
            "keyvault",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-7"), ignore)]
fn azure_7() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-7",
        Some(&[
            "kusto",
            "lb",
            "lighthouse",
            "loadtest",
            "loganalytics",
            "logicapps",
            "machinelearning",
            "maintenance",
            "managedapplication",
            "managedlustre",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-8"), ignore)]
fn azure_8() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-8",
        Some(&[
            "management",
            "managementgroups",
            "managementresource",
            "maps",
            "marketplace",
            "mixedreality",
            "mobile",
            "monitoring",
            "msi",
            "mssql",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-9"), ignore)]
fn azure_9() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-9",
        Some(&[
            "mysql",
            "netapp",
            "network",
            "networkfunction",
            "newrelic",
            "nginx",
            "notificationhub",
            "operationalinsights",
            "oracle",
            "orbital",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-10"), ignore)]
fn azure_10() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-10",
        Some(&[
            "paloalto",
            "pim",
            "policy",
            "portal",
            "postgresql",
            "powerbi",
            "privatedns",
            "privatelink",
            "proximity",
            "purview",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-11"), ignore)]
fn azure_11() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-11",
        Some(&[
            "recoveryservices",
            "redhatopenshift",
            "redis",
            "relay",
            "role",
            "search",
            "securitycenter",
            "sentinel",
            "servicebus",
            "servicefabric",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-12"), ignore)]
fn azure_12() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-12",
        Some(&[
            "signalr",
            "siterecovery",
            "stack",
            "storage",
            "streamanalytics",
            "synapse",
            "systemcenter",
            "trafficmanager",
            "trustedsigning",
            "videoindexer",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_azure-13"), ignore)]
fn azure_13() -> Result<()> {
    run_pulumi_generator_test(
        "azure",
        "azure-13",
        Some(&["voice", "waf", "webpubsub", "workloadssap"]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_filtering-0"), ignore)]
fn filtering_0() -> Result<()> {
    run_pulumi_generator_test("filtering", "filtering-0", Some(&["ns1"]))
}

#[test]
#[cfg_attr(not(feature = "generator_filtering-1"), ignore)]
fn filtering_1() -> Result<()> {
    run_pulumi_generator_test("filtering", "filtering-1", Some(&["ns2"]))
}

#[test]
#[cfg_attr(not(feature = "generator_filtering-2"), ignore)]
fn filtering_2() -> Result<()> {
    run_pulumi_generator_test("filtering", "filtering-2", Some(&["ns1", "ns2"]))
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-0"), ignore)]
fn gcp_0() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-0",
        Some(&[
            "accessapproval",
            "accesscontextmanager",
            "activedirectory",
            "alloydb",
            "apigateway",
            "apigee",
            "appengine",
            "apphub",
            "applicationintegration",
            "artifactregistry",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-1"), ignore)]
fn gcp_1() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-1",
        Some(&[
            "assuredworkloads",
            "backupdisasterrecovery",
            "beyondcorp",
            "biglake",
            "bigquery",
            "bigqueryanalyticshub",
            "bigquerydatapolicy",
            "bigtable",
            "billing",
            "binaryauthorization",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-2"), ignore)]
fn gcp_2() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-2",
        Some(&[
            "blockchainnodeengine",
            "certificateauthority",
            "certificatemanager",
            "cloudasset",
            "cloudbuild",
            "cloudbuildv2",
            "clouddeploy",
            "clouddomains",
            "cloudfunctions",
            "cloudfunctionsv2",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-3"), ignore)]
fn gcp_3() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-3",
        Some(&[
            "cloudidentity",
            "cloudids",
            "cloudquota",
            "cloudrun",
            "cloudrunv2",
            "cloudscheduler",
            "cloudtasks",
            "composer",
            "config",
            "container",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-4"), ignore)]
fn gcp_4() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-4",
        Some(&[
            "containeranalysis",
            "databasemigrationservice",
            "datacatalog",
            "dataflow",
            "dataform",
            "datafusion",
            "dataloss",
            "dataplex",
            "dataproc",
            "datastream",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-5"), ignore)]
fn gcp_5() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-5",
        Some(&[
            "deploymentmanager",
            "developerconnect",
            "diagflow",
            "discoveryengine",
            "dns",
            "edgecontainer",
            "edgenetwork",
            "endpoints",
            "essentialcontacts",
            "eventarc",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-6"), ignore)]
fn gcp_6() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-6",
        Some(&[
            "filestore",
            "firebase",
            "firebaserules",
            "firestore",
            "folder",
            "gemini",
            "gkebackup",
            "gkehub",
            "gkeonprem",
            "healthcare",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-7"), ignore)]
fn gcp_7() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-7",
        Some(&[
            "iam",
            "iap",
            "identityplatform",
            "index",
            "integrationconnectors",
            "kms",
            "logging",
            "looker",
            "managedkafka",
            "memcache",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-8"), ignore)]
fn gcp_8() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-8",
        Some(&[
            "memorystore",
            "migrationcenter",
            "ml",
            "monitoring",
            "netapp",
            "networkconnectivity",
            "networkmanagement",
            "networksecurity",
            "networkservices",
            "notebooks",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-9"), ignore)]
fn gcp_9() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-9",
        Some(&[
            "oracledatabase",
            "organizations",
            "orgpolicy",
            "osconfig",
            "oslogin",
            "parallelstore",
            "privilegedaccessmanager",
            "projects",
            "pubsub",
            "recaptcha",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-10"), ignore)]
fn gcp_10() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-10",
        Some(&[
            "redis",
            "resourcemanager",
            "runtimeconfig",
            "secretmanager",
            "securesourcemanager",
            "securitycenter",
            "securityposture",
            "serviceaccount",
            "servicedirectory",
            "servicenetworking",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-11"), ignore)]
fn gcp_11() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-11",
        Some(&[
            "serviceusage",
            "siteverification",
            "sourcerepo",
            "spanner",
            "sql",
            "storage",
            "tags",
            "tpu",
            "transcoder",
            "vertex",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-12"), ignore)]
fn gcp_12() -> Result<()> {
    run_pulumi_generator_test(
        "gcp",
        "gcp-12",
        Some(&[
            "vmwareengine",
            "vpcaccess",
            "workbench",
            "workflows",
            "workstations",
        ]),
    )
}

#[test]
#[cfg_attr(not(feature = "generator_gcp-13"), ignore)]
fn gcp_13() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-13", Some(&["compute"]))
}
// DO NOT EDIT - END

// provider_name is `name` from yaml file
pub fn run_pulumi_generator_test(
    schema_name: &str,
    directory_name: &str,
    modules: Option<&[&str]>,
) -> Result<()> {
    let root_path = format!("tests/output/{directory_name}");
    let root = Path::new(&root_path);

    let schema = find_schema_files(schema_name);
    fs::create_dir_all(root)?;

    create_symlink(&schema, &root.join(schema.file_name().unwrap())).with_context(|| {
        format!(
            "Failed to create symlink from [{:?}] to [{:?}]",
            schema, root
        )
    })?;

    generate_combined(
        schema.as_path(),
        &root.join("src").join("generated"),
        modules,
    )?;

    let times = FileTimes::new().set_modified(SystemTime::UNIX_EPOCH);

    let cargo_toml_content = CargoToml {
        name: directory_name,
    }
    .render()?;
    let lib_rs = root.join("src/lib.rs");
    fs::write(root.join("Cargo.toml"), cargo_toml_content)?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(&lib_rs, "include!(\"generated/main.rs\");")?;
    fs::copy(
        "../../../../rust-toolchain.toml",
        root.join("rust-toolchain.toml"),
    )?;

    if let Some(env) = std::env::var_os("DO_NOT_COMPILE") {
        if env == "true" {
            if !root.join("Cargo.lock").exists() {
                Command::new("cargo")
                    .args(["generate-lockfile"])
                    .env_remove("CARGO_LLVM_COV")
                    .env_remove("RUSTFLAGS")
                    .current_dir(root)
                    .assert()
                    .success();
            }
            return Ok(());
        }
    }

    File::options()
        .write(true)
        .open(lib_rs)
        .context("Cannot open file")?
        .set_times(times)
        .context("Cannot set times")?;

    Command::new("cargo")
        .args(["component", "build"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .env("CARGO_TARGET_DIR", "../__target/target")
        .env("CARGO_INCREMENTAL", "0")
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args(["test", "--doc"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .env("CARGO_TARGET_DIR", "../__target/target")
        .env("CARGO_INCREMENTAL", "0")
        .current_dir(root)
        .assert()
        .success();

    Command::new("cargo")
        .args(["doc", "--no-deps"])
        .env_remove("CARGO_LLVM_COV")
        .env_remove("RUSTFLAGS")
        .env("CARGO_TARGET_DIR", "../__target/target")
        .env("CARGO_INCREMENTAL", "0")
        .current_dir(root)
        .assert()
        .success();

    Ok(())
}

pub fn find_schema_files(name: &str) -> PathBuf {
    let possible_paths = vec![
        Path::new("tests/test_cases").join(format!("{name}.json")),
        Path::new("../../../../providers").join(format!("{name}.json")),
        Path::new("../../../../external/pulumi/tests/testdata/codegen")
            .join(name)
            .join("schema.yaml"),
        Path::new("../../../../external/pulumi/tests/testdata/codegen")
            .join(name)
            .join("schema.json"),
        Path::new("../../../../external/pulumi/tests/testdata/codegen")
            .join(format!("{name}.yaml")),
        Path::new("../../../../external/pulumi/tests/testdata/codegen")
            .join(format!("{name}.json")),
        Path::new("../../../../external/pulumi-java/pkg/codegen/testing/test/testdata")
            .join(name)
            .join("schema.yaml"),
        Path::new("../../../../external/pulumi-java/pkg/codegen/testing/test/testdata")
            .join(name)
            .join("schema.json"),
        Path::new("../../../../external/pulumi-java/pkg/codegen/testing/test/testdata")
            .join(format!("{name}.yaml")),
        Path::new("../../../../external/pulumi-java/pkg/codegen/testing/test/testdata")
            .join(format!("{name}.json")),
    ];

    for path in possible_paths {
        if path.exists() {
            return path;
        }
    }

    panic!("No schema file found for provider: {name}");
}

fn create_symlink(src: &Path, dst: &Path) -> std::io::Result<()> {
    if dst.exists() {
        fs::remove_file(dst)?;
    }
    use pathdiff::diff_paths;
    let relative_path = diff_paths(src, dst.parent().unwrap()).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&relative_path, dst)?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&relative_path, dst)?;
    Ok(())
}

#[derive(Template)]
#[template(path = "test_Cargo.toml.jinja")]
struct CargoToml<'a> {
    name: &'a str,
}
