#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct Endpoints {
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "accessanalyzer")]
    pub r#accessanalyzer: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "account")]
    pub r#account: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "acm")]
    pub r#acm: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "acmpca")]
    pub r#acmpca: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "amg")]
    pub r#amg: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "amp")]
    pub r#amp: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "amplify")]
    pub r#amplify: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "apigateway")]
    pub r#apigateway: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "apigatewayv2")]
    pub r#apigatewayv_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "appautoscaling")]
    pub r#appautoscaling: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "appconfig")]
    pub r#appconfig: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "appfabric")]
    pub r#appfabric: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "appflow")]
    pub r#appflow: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "appintegrations")]
    pub r#appintegrations: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "appintegrationsservice")]
    pub r#appintegrationsservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "applicationautoscaling")]
    pub r#applicationautoscaling: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "applicationinsights")]
    pub r#applicationinsights: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "applicationsignals")]
    pub r#applicationsignals: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "appmesh")]
    pub r#appmesh: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "appregistry")]
    pub r#appregistry: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "apprunner")]
    pub r#apprunner: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "appstream")]
    pub r#appstream: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "appsync")]
    pub r#appsync: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "athena")]
    pub r#athena: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "auditmanager")]
    pub r#auditmanager: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "autoscaling")]
    pub r#autoscaling: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "autoscalingplans")]
    pub r#autoscalingplans: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "backup")]
    pub r#backup: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "batch")]
    pub r#batch: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "bcmdataexports")]
    pub r#bcmdataexports: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "beanstalk")]
    pub r#beanstalk: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "bedrock")]
    pub r#bedrock: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "bedrockagent")]
    pub r#bedrockagent: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "budgets")]
    pub r#budgets: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ce")]
    pub r#ce: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "chatbot")]
    pub r#chatbot: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "chime")]
    pub r#chime: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "chimesdkmediapipelines")]
    pub r#chimesdkmediapipelines: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "chimesdkvoice")]
    pub r#chimesdkvoice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cleanrooms")]
    pub r#cleanrooms: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloud9")]
    pub r#cloud_9: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudcontrol")]
    pub r#cloudcontrol: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudcontrolapi")]
    pub r#cloudcontrolapi: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudformation")]
    pub r#cloudformation: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudfront")]
    pub r#cloudfront: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudfrontkeyvaluestore")]
    pub r#cloudfrontkeyvaluestore: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudhsm")]
    pub r#cloudhsm: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudhsmv2")]
    pub r#cloudhsmv_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudsearch")]
    pub r#cloudsearch: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudtrail")]
    pub r#cloudtrail: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudwatch")]
    pub r#cloudwatch: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudwatchevents")]
    pub r#cloudwatchevents: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudwatchevidently")]
    pub r#cloudwatchevidently: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudwatchlog")]
    pub r#cloudwatchlog: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudwatchlogs")]
    pub r#cloudwatchlogs: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudwatchobservabilityaccessmanager")]
    pub r#cloudwatchobservabilityaccessmanager: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cloudwatchrum")]
    pub r#cloudwatchrum: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codeartifact")]
    pub r#codeartifact: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codebuild")]
    pub r#codebuild: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codecatalyst")]
    pub r#codecatalyst: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codecommit")]
    pub r#codecommit: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codeconnections")]
    pub r#codeconnections: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codedeploy")]
    pub r#codedeploy: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codeguruprofiler")]
    pub r#codeguruprofiler: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codegurureviewer")]
    pub r#codegurureviewer: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codepipeline")]
    pub r#codepipeline: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codestarconnections")]
    pub r#codestarconnections: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "codestarnotifications")]
    pub r#codestarnotifications: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cognitoidentity")]
    pub r#cognitoidentity: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cognitoidentityprovider")]
    pub r#cognitoidentityprovider: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cognitoidp")]
    pub r#cognitoidp: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "comprehend")]
    pub r#comprehend: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "computeoptimizer")]
    pub r#computeoptimizer: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "config")]
    pub r#config: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "configservice")]
    pub r#configservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "connect")]
    pub r#connect: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "connectcases")]
    pub r#connectcases: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "controltower")]
    pub r#controltower: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "costandusagereportservice")]
    pub r#costandusagereportservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "costexplorer")]
    pub r#costexplorer: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "costoptimizationhub")]
    pub r#costoptimizationhub: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "cur")]
    pub r#cur: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "customerprofiles")]
    pub r#customerprofiles: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "databasemigration")]
    pub r#databasemigration: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "databasemigrationservice")]
    pub r#databasemigrationservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "databrew")]
    pub r#databrew: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "dataexchange")]
    pub r#dataexchange: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "datapipeline")]
    pub r#datapipeline: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "datasync")]
    pub r#datasync: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "datazone")]
    pub r#datazone: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "dax")]
    pub r#dax: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "deploy")]
    pub r#deploy: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "detective")]
    pub r#detective: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "devicefarm")]
    pub r#devicefarm: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "devopsguru")]
    pub r#devopsguru: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "directconnect")]
    pub r#directconnect: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "directoryservice")]
    pub r#directoryservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "dlm")]
    pub r#dlm: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "dms")]
    pub r#dms: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "docdb")]
    pub r#docdb: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "docdbelastic")]
    pub r#docdbelastic: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "drs")]
    pub r#drs: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ds")]
    pub r#ds: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "dynamodb")]
    pub r#dynamodb: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ec2")]
    pub r#ec_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ecr")]
    pub r#ecr: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ecrpublic")]
    pub r#ecrpublic: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ecs")]
    pub r#ecs: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "efs")]
    pub r#efs: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "eks")]
    pub r#eks: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "elasticache")]
    pub r#elasticache: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "elasticbeanstalk")]
    pub r#elasticbeanstalk: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "elasticloadbalancing")]
    pub r#elasticloadbalancing: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "elasticloadbalancingv2")]
    pub r#elasticloadbalancingv_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "elasticsearch")]
    pub r#elasticsearch: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "elasticsearchservice")]
    pub r#elasticsearchservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "elastictranscoder")]
    pub r#elastictranscoder: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "elb")]
    pub r#elb: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "elbv2")]
    pub r#elbv_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "emr")]
    pub r#emr: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "emrcontainers")]
    pub r#emrcontainers: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "emrserverless")]
    pub r#emrserverless: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "es")]
    pub r#es: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "eventbridge")]
    pub r#eventbridge: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "events")]
    pub r#events: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "evidently")]
    pub r#evidently: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "finspace")]
    pub r#finspace: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "firehose")]
    pub r#firehose: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "fis")]
    pub r#fis: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "fms")]
    pub r#fms: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "fsx")]
    pub r#fsx: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "gamelift")]
    pub r#gamelift: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "glacier")]
    pub r#glacier: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "globalaccelerator")]
    pub r#globalaccelerator: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "glue")]
    pub r#glue: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "gluedatabrew")]
    pub r#gluedatabrew: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "grafana")]
    pub r#grafana: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "greengrass")]
    pub r#greengrass: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "groundstation")]
    pub r#groundstation: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "guardduty")]
    pub r#guardduty: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "healthlake")]
    pub r#healthlake: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "iam")]
    pub r#iam: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "identitystore")]
    pub r#identitystore: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "imagebuilder")]
    pub r#imagebuilder: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "inspector")]
    pub r#inspector: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "inspector2")]
    pub r#inspector_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "inspectorv2")]
    pub r#inspectorv_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "internetmonitor")]
    pub r#internetmonitor: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "iot")]
    pub r#iot: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "iotanalytics")]
    pub r#iotanalytics: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "iotevents")]
    pub r#iotevents: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ivs")]
    pub r#ivs: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ivschat")]
    pub r#ivschat: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "kafka")]
    pub r#kafka: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "kafkaconnect")]
    pub r#kafkaconnect: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "kendra")]
    pub r#kendra: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "keyspaces")]
    pub r#keyspaces: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "kinesis")]
    pub r#kinesis: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "kinesisanalytics")]
    pub r#kinesisanalytics: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "kinesisanalyticsv2")]
    pub r#kinesisanalyticsv_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "kinesisvideo")]
    pub r#kinesisvideo: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "kms")]
    pub r#kms: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "lakeformation")]
    pub r#lakeformation: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "lambda")]
    pub r#lambda: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "launchwizard")]
    pub r#launchwizard: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "lex")]
    pub r#lex: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "lexmodelbuilding")]
    pub r#lexmodelbuilding: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "lexmodelbuildingservice")]
    pub r#lexmodelbuildingservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "lexmodels")]
    pub r#lexmodels: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "lexmodelsv2")]
    pub r#lexmodelsv_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "lexv2models")]
    pub r#lexv_2_models: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "licensemanager")]
    pub r#licensemanager: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "lightsail")]
    pub r#lightsail: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "locationservice")]
    pub r#locationservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "logs")]
    pub r#logs: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "lookoutmetrics")]
    pub r#lookoutmetrics: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "m2")]
    pub r#m_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "macie2")]
    pub r#macie_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "managedgrafana")]
    pub r#managedgrafana: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "mediaconnect")]
    pub r#mediaconnect: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "mediaconvert")]
    pub r#mediaconvert: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "medialive")]
    pub r#medialive: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "mediapackage")]
    pub r#mediapackage: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "mediapackagev2")]
    pub r#mediapackagev_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "mediastore")]
    pub r#mediastore: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "memorydb")]
    pub r#memorydb: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "mgn")]
    pub r#mgn: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "mq")]
    pub r#mq: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "msk")]
    pub r#msk: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "mwaa")]
    pub r#mwaa: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "neptune")]
    pub r#neptune: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "neptunegraph")]
    pub r#neptunegraph: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "networkfirewall")]
    pub r#networkfirewall: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "networkmanager")]
    pub r#networkmanager: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "networkmonitor")]
    pub r#networkmonitor: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "oam")]
    pub r#oam: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "opensearch")]
    pub r#opensearch: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "opensearchingestion")]
    pub r#opensearchingestion: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "opensearchserverless")]
    pub r#opensearchserverless: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "opensearchservice")]
    pub r#opensearchservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "opsworks")]
    pub r#opsworks: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "organizations")]
    pub r#organizations: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "osis")]
    pub r#osis: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "outposts")]
    pub r#outposts: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "paymentcryptography")]
    pub r#paymentcryptography: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "pcaconnectorad")]
    pub r#pcaconnectorad: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "pcs")]
    pub r#pcs: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "pinpoint")]
    pub r#pinpoint: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "pinpointsmsvoicev2")]
    pub r#pinpointsmsvoicev_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "pipes")]
    pub r#pipes: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "polly")]
    pub r#polly: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "pricing")]
    pub r#pricing: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "prometheus")]
    pub r#prometheus: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "prometheusservice")]
    pub r#prometheusservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "qbusiness")]
    pub r#qbusiness: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "qldb")]
    pub r#qldb: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "quicksight")]
    pub r#quicksight: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ram")]
    pub r#ram: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "rbin")]
    pub r#rbin: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "rds")]
    pub r#rds: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "recyclebin")]
    pub r#recyclebin: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "redshift")]
    pub r#redshift: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "redshiftdata")]
    pub r#redshiftdata: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "redshiftdataapiservice")]
    pub r#redshiftdataapiservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "redshiftserverless")]
    pub r#redshiftserverless: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "rekognition")]
    pub r#rekognition: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "resiliencehub")]
    pub r#resiliencehub: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "resourceexplorer2")]
    pub r#resourceexplorer_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "resourcegroups")]
    pub r#resourcegroups: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "resourcegroupstagging")]
    pub r#resourcegroupstagging: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "resourcegroupstaggingapi")]
    pub r#resourcegroupstaggingapi: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "rolesanywhere")]
    pub r#rolesanywhere: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "route53")]
    pub r#route_53: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "route53domains")]
    pub r#route_53_domains: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "route53profiles")]
    pub r#route_53_profiles: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "route53recoverycontrolconfig")]
    pub r#route_53_recoverycontrolconfig: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "route53recoveryreadiness")]
    pub r#route_53_recoveryreadiness: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "route53resolver")]
    pub r#route_53_resolver: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "rum")]
    pub r#rum: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "s3api")]
    pub r#s_3_api: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "s3control")]
    pub r#s_3_control: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "s3outposts")]
    pub r#s_3_outposts: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "s3tables")]
    pub r#s_3_tables: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "sagemaker")]
    pub r#sagemaker: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "scheduler")]
    pub r#scheduler: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "schemas")]
    pub r#schemas: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "sdb")]
    pub r#sdb: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "secretsmanager")]
    pub r#secretsmanager: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "securityhub")]
    pub r#securityhub: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "securitylake")]
    pub r#securitylake: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "serverlessapplicationrepository")]
    pub r#serverlessapplicationrepository: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "serverlessapprepo")]
    pub r#serverlessapprepo: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "serverlessrepo")]
    pub r#serverlessrepo: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "servicecatalog")]
    pub r#servicecatalog: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "servicecatalogappregistry")]
    pub r#servicecatalogappregistry: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "servicediscovery")]
    pub r#servicediscovery: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "servicequotas")]
    pub r#servicequotas: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ses")]
    pub r#ses: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "sesv2")]
    pub r#sesv_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "sfn")]
    pub r#sfn: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "shield")]
    pub r#shield: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "signer")]
    pub r#signer: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "simpledb")]
    pub r#simpledb: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "sns")]
    pub r#sns: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "sqs")]
    pub r#sqs: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ssm")]
    pub r#ssm: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ssmcontacts")]
    pub r#ssmcontacts: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ssmincidents")]
    pub r#ssmincidents: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ssmquicksetup")]
    pub r#ssmquicksetup: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ssmsap")]
    pub r#ssmsap: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "sso")]
    pub r#sso: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "ssoadmin")]
    pub r#ssoadmin: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "stepfunctions")]
    pub r#stepfunctions: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "storagegateway")]
    pub r#storagegateway: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "sts")]
    pub r#sts: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "swf")]
    pub r#swf: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "synthetics")]
    pub r#synthetics: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "taxsettings")]
    pub r#taxsettings: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "timestreaminfluxdb")]
    pub r#timestreaminfluxdb: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "timestreamquery")]
    pub r#timestreamquery: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "timestreamwrite")]
    pub r#timestreamwrite: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "transcribe")]
    pub r#transcribe: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "transcribeservice")]
    pub r#transcribeservice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "transfer")]
    pub r#transfer: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "verifiedpermissions")]
    pub r#verifiedpermissions: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "vpclattice")]
    pub r#vpclattice: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "waf")]
    pub r#waf: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "wafregional")]
    pub r#wafregional: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "wafv2")]
    pub r#wafv_2: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "wellarchitected")]
    pub r#wellarchitected: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "worklink")]
    pub r#worklink: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "workspaces")]
    pub r#workspaces: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "workspacesweb")]
    pub r#workspacesweb: Box<Option<String>>,
    /// Use this to override the default service endpoint URL
    #[builder(into, default)]
    #[serde(rename = "xray")]
    pub r#xray: Box<Option<String>>,
}
