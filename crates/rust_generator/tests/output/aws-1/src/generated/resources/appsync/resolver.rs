/// Provides an AppSync Resolver.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
///           function: std:file
///           arguments:
///             input: some-code-dir
///           return: result
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverArgs {
        /// API ID for the GraphQL API.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Caching Config. See Caching Config.
        #[builder(into, default)]
        pub caching_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appsync::ResolverCachingConfig>,
        >,
        /// The function code that contains the request and response functions. When code is used, the runtime is required. The runtime value must be APPSYNC_JS.
        #[builder(into, default)]
        pub code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Data source name.
        #[builder(into, default)]
        pub data_source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Field name from the schema defined in the GraphQL API.
        #[builder(into)]
        pub field: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resolver type. Valid values are `UNIT` and `PIPELINE`.
        #[builder(into, default)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Maximum batching size for a resolver. Valid values are between `0` and `2000`.
        #[builder(into, default)]
        pub max_batch_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The caching configuration for the resolver. See Pipeline Config.
        #[builder(into, default)]
        pub pipeline_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appsync::ResolverPipelineConfig>,
        >,
        /// Request mapping template for UNIT resolver or 'before mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
        #[builder(into, default)]
        pub request_template: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Response mapping template for UNIT resolver or 'after mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
        #[builder(into, default)]
        pub response_template: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must also be specified. See Runtime.
        #[builder(into, default)]
        pub runtime: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appsync::ResolverRuntime>,
        >,
        /// Describes a Sync configuration for a resolver. See Sync Config.
        #[builder(into, default)]
        pub sync_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appsync::ResolverSyncConfig>,
        >,
        /// Type name from the schema defined in the GraphQL API.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResolverResult {
        /// API ID for the GraphQL API.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// ARN
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Caching Config. See Caching Config.
        pub caching_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::appsync::ResolverCachingConfig>,
        >,
        /// The function code that contains the request and response functions. When code is used, the runtime is required. The runtime value must be APPSYNC_JS.
        pub code: pulumi_gestalt_rust::Output<Option<String>>,
        /// Data source name.
        pub data_source: pulumi_gestalt_rust::Output<Option<String>>,
        /// Field name from the schema defined in the GraphQL API.
        pub field: pulumi_gestalt_rust::Output<String>,
        /// Resolver type. Valid values are `UNIT` and `PIPELINE`.
        pub kind: pulumi_gestalt_rust::Output<Option<String>>,
        /// Maximum batching size for a resolver. Valid values are between `0` and `2000`.
        pub max_batch_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The caching configuration for the resolver. See Pipeline Config.
        pub pipeline_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::appsync::ResolverPipelineConfig>,
        >,
        /// Request mapping template for UNIT resolver or 'before mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
        pub request_template: pulumi_gestalt_rust::Output<Option<String>>,
        /// Response mapping template for UNIT resolver or 'after mapping template' for PIPELINE resolver. Required for non-Lambda resolvers.
        pub response_template: pulumi_gestalt_rust::Output<Option<String>>,
        /// Describes a runtime used by an AWS AppSync pipeline resolver or AWS AppSync function. Specifies the name and version of the runtime to use. Note that if a runtime is specified, code must also be specified. See Runtime.
        pub runtime: pulumi_gestalt_rust::Output<
            Option<super::super::types::appsync::ResolverRuntime>,
        >,
        /// Describes a Sync configuration for a resolver. See Sync Config.
        pub sync_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::appsync::ResolverSyncConfig>,
        >,
        /// Type name from the schema defined in the GraphQL API.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResolverArgs,
    ) -> ResolverResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let caching_config_binding = args.caching_config.get_output(context);
        let code_binding = args.code.get_output(context);
        let data_source_binding = args.data_source.get_output(context);
        let field_binding = args.field.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let max_batch_size_binding = args.max_batch_size.get_output(context);
        let pipeline_config_binding = args.pipeline_config.get_output(context);
        let request_template_binding = args.request_template.get_output(context);
        let response_template_binding = args.response_template.get_output(context);
        let runtime_binding = args.runtime.get_output(context);
        let sync_config_binding = args.sync_config.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appsync/resolver:Resolver".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cachingConfig".into(),
                    value: &caching_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "code".into(),
                    value: &code_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSource".into(),
                    value: &data_source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "field".into(),
                    value: &field_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: &kind_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxBatchSize".into(),
                    value: &max_batch_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pipelineConfig".into(),
                    value: &pipeline_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestTemplate".into(),
                    value: &request_template_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "responseTemplate".into(),
                    value: &response_template_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtime".into(),
                    value: &runtime_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "syncConfig".into(),
                    value: &sync_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResolverResult {
            api_id: o.get_field("apiId"),
            arn: o.get_field("arn"),
            caching_config: o.get_field("cachingConfig"),
            code: o.get_field("code"),
            data_source: o.get_field("dataSource"),
            field: o.get_field("field"),
            kind: o.get_field("kind"),
            max_batch_size: o.get_field("maxBatchSize"),
            pipeline_config: o.get_field("pipelineConfig"),
            request_template: o.get_field("requestTemplate"),
            response_template: o.get_field("responseTemplate"),
            runtime: o.get_field("runtime"),
            sync_config: o.get_field("syncConfig"),
            type_: o.get_field("type"),
        }
    }
}
