/// Provides an AppSync Resolver.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let mutationPipelineTest = resolver::create(
///         "mutationPipelineTest",
///         ResolverArgs::builder()
///             .api_id("${test.id}")
///             .field("pipelineTest")
///             .kind("PIPELINE")
///             .pipeline_config(
///                 ResolverPipelineConfig::builder()
///                     .functions(
///                         vec![
///                             "${test1.functionId}", "${test2.functionId}",
///                             "${test3.functionId}",
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .request_template("{}")
///             .response_template("$util.toJson($ctx.result)")
///             .type_("Mutation")
///             .build_struct(),
///     );
///     let test = graph_ql_api::create(
///         "test",
///         GraphQlApiArgs::builder()
///             .authentication_type("API_KEY")
///             .name("tf-example")
///             .schema(
///                 "type Mutation {\n\tputPost(id: ID!, title: String!): Post\n}\n\ntype Post {\n\tid: ID!\n\ttitle: String!\n}\n\ntype Query {\n\tsinglePost(id: ID!): Post\n}\n\nschema {\n\tquery: Query\n\tmutation: Mutation\n}\n",
///             )
///             .build_struct(),
///     );
///     let testDataSource = data_source::create(
///         "testDataSource",
///         DataSourceArgs::builder()
///             .api_id("${test.id}")
///             .http_config(
///                 DataSourceHttpConfig::builder()
///                     .endpoint("http://example.com")
///                     .build_struct(),
///             )
///             .name("my_example")
///             .type_("HTTP")
///             .build_struct(),
///     );
///     let testResolver = resolver::create(
///         "testResolver",
///         ResolverArgs::builder()
///             .api_id("${test.id}")
///             .caching_config(
///                 ResolverCachingConfig::builder()
///                     .cachingKeys(vec!["$context.identity.sub", "$context.arguments.id",])
///                     .ttl(60)
///                     .build_struct(),
///             )
///             .data_source("${testDataSource.name}")
///             .field("singlePost")
///             .request_template(
///                 "{\n    \"version\": \"2018-05-29\",\n    \"method\": \"GET\",\n    \"resourcePath\": \"/\",\n    \"params\":{\n        \"headers\": $utils.http.copyheaders($ctx.request.headers)\n    }\n}\n",
///             )
///             .response_template(
///                 "#if($ctx.result.statusCode == 200)\n    $ctx.result.body\n#else\n    $utils.appendError($ctx.result.body, $ctx.result.statusCode)\n#end\n",
///             )
///             .type_("Query")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### JS
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appsync:Resolver
///     properties:
///       type: Query
///       apiId: ${testAwsAppsyncGraphqlApi.id}
///       field: pipelineTest
///       kind: PIPELINE
///       code:
///         fn::invoke:
///           Function: std:file
///           Arguments:
///             input: some-code-dir
///           Return: result
///       runtime:
///         name: APPSYNC_JS
///         runtimeVersion: 1.0.0
///       pipelineConfig:
///         functions:
///           - ${test.functionId}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appsync_resolver` using the `api_id`, a hyphen, `type`, a hypen and `field`. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/resolver:Resolver example abcdef123456-exampleType-exampleField
/// ```
pub mod resolver {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverArgs {
        /// API ID for the GraphQL API.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The Caching Config. See Caching Config.
        #[builder(into, default)]
        pub caching_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::ResolverCachingConfig>,
        >,
        /// The function code that contains the request and response functions. When code is used, the runtime is required. The runtime value must be APPSYNC_JS.
        #[builder(into, default)]
        pub code: pulumi_wasm_rust::Output<Option<String>>,
        /// Data source name.
        #[builder(into, default)]
        pub data_source: pulumi_wasm_rust::Output<Option<String>>,
        /// Field name from the schema defined in the GraphQL API.
        #[builder(into)]
        pub field: pulumi_wasm_rust::Output<String>,
        /// Resolver type. Valid values are `UNIT` and `PIPELINE`.
        #[builder(into, default)]
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum batching size for a resolver. Valid values are between `0` and `2000`.
        #[builder(into, default)]
        pub max_batch_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The caching configuration for the resolver. See Pipeline Config.
        #[builder(into, default)]
        pub pipeline_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::ResolverPipelineConfig>,
        >,
        /// Request mapping template for UNIT resolver or 'before mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
        #[builder(into, default)]
        pub request_template: pulumi_wasm_rust::Output<Option<String>>,
        /// Response mapping template for UNIT resolver or 'after mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
        #[builder(into, default)]
        pub response_template: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must also be specified. See Runtime.
        #[builder(into, default)]
        pub runtime: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::ResolverRuntime>,
        >,
        /// Describes a Sync configuration for a resolver. See Sync Config.
        #[builder(into, default)]
        pub sync_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::ResolverSyncConfig>,
        >,
        /// Type name from the schema defined in the GraphQL API.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverResult {
        /// API ID for the GraphQL API.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// ARN
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Caching Config. See Caching Config.
        pub caching_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::ResolverCachingConfig>,
        >,
        /// The function code that contains the request and response functions. When code is used, the runtime is required. The runtime value must be APPSYNC_JS.
        pub code: pulumi_wasm_rust::Output<Option<String>>,
        /// Data source name.
        pub data_source: pulumi_wasm_rust::Output<Option<String>>,
        /// Field name from the schema defined in the GraphQL API.
        pub field: pulumi_wasm_rust::Output<String>,
        /// Resolver type. Valid values are `UNIT` and `PIPELINE`.
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum batching size for a resolver. Valid values are between `0` and `2000`.
        pub max_batch_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// The caching configuration for the resolver. See Pipeline Config.
        pub pipeline_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::ResolverPipelineConfig>,
        >,
        /// Request mapping template for UNIT resolver or 'before mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
        pub request_template: pulumi_wasm_rust::Output<Option<String>>,
        /// Response mapping template for UNIT resolver or 'after mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
        pub response_template: pulumi_wasm_rust::Output<Option<String>>,
        /// Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must also be specified. See Runtime.
        pub runtime: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::ResolverRuntime>,
        >,
        /// Describes a Sync configuration for a resolver. See Sync Config.
        pub sync_config: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::ResolverSyncConfig>,
        >,
        /// Type name from the schema defined in the GraphQL API.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResolverArgs) -> ResolverResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let caching_config_binding = args.caching_config.get_inner();
        let code_binding = args.code.get_inner();
        let data_source_binding = args.data_source.get_inner();
        let field_binding = args.field.get_inner();
        let kind_binding = args.kind.get_inner();
        let max_batch_size_binding = args.max_batch_size.get_inner();
        let pipeline_config_binding = args.pipeline_config.get_inner();
        let request_template_binding = args.request_template.get_inner();
        let response_template_binding = args.response_template.get_inner();
        let runtime_binding = args.runtime.get_inner();
        let sync_config_binding = args.sync_config.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appsync/resolver:Resolver".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "cachingConfig".into(),
                    value: &caching_config_binding,
                },
                register_interface::ObjectField {
                    name: "code".into(),
                    value: &code_binding,
                },
                register_interface::ObjectField {
                    name: "dataSource".into(),
                    value: &data_source_binding,
                },
                register_interface::ObjectField {
                    name: "field".into(),
                    value: &field_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "maxBatchSize".into(),
                    value: &max_batch_size_binding,
                },
                register_interface::ObjectField {
                    name: "pipelineConfig".into(),
                    value: &pipeline_config_binding,
                },
                register_interface::ObjectField {
                    name: "requestTemplate".into(),
                    value: &request_template_binding,
                },
                register_interface::ObjectField {
                    name: "responseTemplate".into(),
                    value: &response_template_binding,
                },
                register_interface::ObjectField {
                    name: "runtime".into(),
                    value: &runtime_binding,
                },
                register_interface::ObjectField {
                    name: "syncConfig".into(),
                    value: &sync_config_binding,
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
                    name: "cachingConfig".into(),
                },
                register_interface::ResultField {
                    name: "code".into(),
                },
                register_interface::ResultField {
                    name: "dataSource".into(),
                },
                register_interface::ResultField {
                    name: "field".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "maxBatchSize".into(),
                },
                register_interface::ResultField {
                    name: "pipelineConfig".into(),
                },
                register_interface::ResultField {
                    name: "requestTemplate".into(),
                },
                register_interface::ResultField {
                    name: "responseTemplate".into(),
                },
                register_interface::ResultField {
                    name: "runtime".into(),
                },
                register_interface::ResultField {
                    name: "syncConfig".into(),
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
        ResolverResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            caching_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cachingConfig").unwrap(),
            ),
            code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("code").unwrap(),
            ),
            data_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSource").unwrap(),
            ),
            field: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("field").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            max_batch_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxBatchSize").unwrap(),
            ),
            pipeline_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pipelineConfig").unwrap(),
            ),
            request_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestTemplate").unwrap(),
            ),
            response_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseTemplate").unwrap(),
            ),
            runtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtime").unwrap(),
            ),
            sync_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("syncConfig").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
