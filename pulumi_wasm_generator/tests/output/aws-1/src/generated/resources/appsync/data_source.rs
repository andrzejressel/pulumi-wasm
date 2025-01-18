/// Provides an AppSync Data Source.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleTable:
///     type: aws:dynamodb:Table
///     name: example
///     properties:
///       name: example
///       readCapacity: 1
///       writeCapacity: 1
///       hashKey: UserId
///       attributes:
///         - name: UserId
///           type: S
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example
///       assumeRolePolicy: ${assumeRole.json}
///   exampleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: example
///     properties:
///       name: example
///       role: ${exampleRole.id}
///       policy: ${example.json}
///   exampleGraphQLApi:
///     type: aws:appsync:GraphQLApi
///     name: example
///     properties:
///       authenticationType: API_KEY
///       name: my_appsync_example
///   exampleDataSource:
///     type: aws:appsync:DataSource
///     name: example
///     properties:
///       apiId: ${exampleGraphQLApi.id}
///       name: my_appsync_example
///       serviceRoleArn: ${exampleRole.arn}
///       type: AMAZON_DYNAMODB
///       dynamodbConfig:
///         tableName: ${exampleTable.name}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - appsync.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - dynamodb:*
///             resources:
///               - ${exampleTable.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appsync_datasource` using the `api_id`, a hyphen, and `name`. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/dataSource:DataSource example abcdef123456-example
/// ```
pub mod data_source {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataSourceArgs {
        /// API ID for the GraphQL API for the data source.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// Description of the data source.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// DynamoDB settings. See `dynamodb_config` Block for details.
        #[builder(into, default)]
        pub dynamodb_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceDynamodbConfig>,
        >,
        /// Amazon Elasticsearch settings. See `elasticsearch_config` Block for details.
        #[builder(into, default)]
        pub elasticsearch_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceElasticsearchConfig>,
        >,
        /// AWS EventBridge settings. See `event_bridge_config` Block for details.
        #[builder(into, default)]
        pub event_bridge_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceEventBridgeConfig>,
        >,
        /// HTTP settings. See `http_config` Block for details.
        #[builder(into, default)]
        pub http_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceHttpConfig>,
        >,
        /// AWS Lambda settings. See `lambda_config` Block for details.
        #[builder(into, default)]
        pub lambda_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceLambdaConfig>,
        >,
        /// User-supplied name for the data source.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon OpenSearch Service settings. See `opensearchservice_config` Block for details.
        #[builder(into, default)]
        pub opensearchservice_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceOpensearchserviceConfig>,
        >,
        /// AWS RDS settings. See `relational_database_config` Block for details.
        #[builder(into, default)]
        pub relational_database_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceRelationalDatabaseConfig>,
        >,
        /// IAM service role ARN for the data source. Required if `type` is specified as `AWS_LAMBDA`, `AMAZON_DYNAMODB`, `AMAZON_ELASTICSEARCH`, `AMAZON_EVENTBRIDGE`, or `AMAZON_OPENSEARCH_SERVICE`.
        #[builder(into, default)]
        pub service_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of the Data Source. Valid values: `AWS_LAMBDA`, `AMAZON_DYNAMODB`, `AMAZON_ELASTICSEARCH`, `HTTP`, `NONE`, `RELATIONAL_DATABASE`, `AMAZON_EVENTBRIDGE`, `AMAZON_OPENSEARCH_SERVICE`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DataSourceResult {
        /// API ID for the GraphQL API for the data source.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// ARN
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the data source.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// DynamoDB settings. See `dynamodb_config` Block for details.
        pub dynamodb_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceDynamodbConfig>,
        >,
        /// Amazon Elasticsearch settings. See `elasticsearch_config` Block for details.
        pub elasticsearch_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceElasticsearchConfig>,
        >,
        /// AWS EventBridge settings. See `event_bridge_config` Block for details.
        pub event_bridge_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceEventBridgeConfig>,
        >,
        /// HTTP settings. See `http_config` Block for details.
        pub http_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceHttpConfig>,
        >,
        /// AWS Lambda settings. See `lambda_config` Block for details.
        pub lambda_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceLambdaConfig>,
        >,
        /// User-supplied name for the data source.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Amazon OpenSearch Service settings. See `opensearchservice_config` Block for details.
        pub opensearchservice_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceOpensearchserviceConfig>,
        >,
        /// AWS RDS settings. See `relational_database_config` Block for details.
        pub relational_database_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::DataSourceRelationalDatabaseConfig>,
        >,
        /// IAM service role ARN for the data source. Required if `type` is specified as `AWS_LAMBDA`, `AMAZON_DYNAMODB`, `AMAZON_ELASTICSEARCH`, `AMAZON_EVENTBRIDGE`, or `AMAZON_OPENSEARCH_SERVICE`.
        pub service_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of the Data Source. Valid values: `AWS_LAMBDA`, `AMAZON_DYNAMODB`, `AMAZON_ELASTICSEARCH`, `HTTP`, `NONE`, `RELATIONAL_DATABASE`, `AMAZON_EVENTBRIDGE`, `AMAZON_OPENSEARCH_SERVICE`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataSourceArgs) -> DataSourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let description_binding = args.description.get_inner();
        let dynamodb_config_binding = args.dynamodb_config.get_inner();
        let elasticsearch_config_binding = args.elasticsearch_config.get_inner();
        let event_bridge_config_binding = args.event_bridge_config.get_inner();
        let http_config_binding = args.http_config.get_inner();
        let lambda_config_binding = args.lambda_config.get_inner();
        let name_binding = args.name.get_inner();
        let opensearchservice_config_binding = args.opensearchservice_config.get_inner();
        let relational_database_config_binding = args
            .relational_database_config
            .get_inner();
        let service_role_arn_binding = args.service_role_arn.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appsync/dataSource:DataSource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dynamodbConfig".into(),
                    value: &dynamodb_config_binding,
                },
                register_interface::ObjectField {
                    name: "elasticsearchConfig".into(),
                    value: &elasticsearch_config_binding,
                },
                register_interface::ObjectField {
                    name: "eventBridgeConfig".into(),
                    value: &event_bridge_config_binding,
                },
                register_interface::ObjectField {
                    name: "httpConfig".into(),
                    value: &http_config_binding,
                },
                register_interface::ObjectField {
                    name: "lambdaConfig".into(),
                    value: &lambda_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "opensearchserviceConfig".into(),
                    value: &opensearchservice_config_binding,
                },
                register_interface::ObjectField {
                    name: "relationalDatabaseConfig".into(),
                    value: &relational_database_config_binding,
                },
                register_interface::ObjectField {
                    name: "serviceRoleArn".into(),
                    value: &service_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dynamodbConfig".into(),
                },
                register_interface::ResultField {
                    name: "elasticsearchConfig".into(),
                },
                register_interface::ResultField {
                    name: "eventBridgeConfig".into(),
                },
                register_interface::ResultField {
                    name: "httpConfig".into(),
                },
                register_interface::ResultField {
                    name: "lambdaConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "opensearchserviceConfig".into(),
                },
                register_interface::ResultField {
                    name: "relationalDatabaseConfig".into(),
                },
                register_interface::ResultField {
                    name: "serviceRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataSourceResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dynamodb_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dynamodbConfig").unwrap(),
            ),
            elasticsearch_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticsearchConfig").unwrap(),
            ),
            event_bridge_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventBridgeConfig").unwrap(),
            ),
            http_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpConfig").unwrap(),
            ),
            lambda_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lambdaConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            opensearchservice_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("opensearchserviceConfig").unwrap(),
            ),
            relational_database_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("relationalDatabaseConfig").unwrap(),
            ),
            service_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceRoleArn").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
