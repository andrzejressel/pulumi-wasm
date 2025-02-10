#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod graph_ql_api {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GraphQLApiArgs {
        /// One or more additional authentication providers for the GraphQL API. See `additional_authentication_provider` Block for details.
        #[builder(into, default)]
        pub additional_authentication_providers: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::appsync::GraphQlApiAdditionalAuthenticationProvider,
                >,
            >,
        >,
        /// API type. Valid values are `GRAPHQL` or `MERGED`. A `MERGED` type requires `merged_api_execution_role_arn` to be set.
        #[builder(into, default)]
        pub api_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Authentication type. Valid values: `API_KEY`, `AWS_IAM`, `AMAZON_COGNITO_USER_POOLS`, `OPENID_CONNECT`, `AWS_LAMBDA`
        #[builder(into)]
        pub authentication_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Enables and controls the enhanced metrics feature. See `enhanced_metrics_config` Block for details.
        #[builder(into, default)]
        pub enhanced_metrics_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appsync::GraphQlApiEnhancedMetricsConfig>,
        >,
        /// Sets the value of the GraphQL API to enable (`ENABLED`) or disable (`DISABLED`) introspection. If no value is provided, the introspection configuration will be set to ENABLED by default. This field will produce an error if the operation attempts to use the introspection feature while this field is disabled. For more information about introspection, see [GraphQL introspection](https://graphql.org/learn/introspection/).
        #[builder(into, default)]
        pub introspection_config: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Nested argument containing Lambda authorizer configuration. See `lambda_authorizer_config` Block for details.
        #[builder(into, default)]
        pub lambda_authorizer_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appsync::GraphQlApiLambdaAuthorizerConfig>,
        >,
        /// Nested argument containing logging configuration. See `log_config` Block for details.
        #[builder(into, default)]
        pub log_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appsync::GraphQlApiLogConfig>,
        >,
        /// ARN of the execution role when `api_type` is set to `MERGED`.
        #[builder(into, default)]
        pub merged_api_execution_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// User-supplied name for the GraphQL API.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Nested argument containing OpenID Connect configuration. See `openid_connect_config` Block for details.
        #[builder(into, default)]
        pub openid_connect_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appsync::GraphQlApiOpenidConnectConfig>,
        >,
        /// The maximum depth a query can have in a single request. Depth refers to the amount of nested levels allowed in the body of query. The default value is `0` (or unspecified), which indicates there's no depth limit. If you set a limit, it can be between `1` and `75` nested levels. This field will produce a limit error if the operation falls out of bounds.
        ///
        /// Note that fields can still be set to nullable or non-nullable. If a non-nullable field produces an error, the error will be thrown upwards to the first nullable field available.
        #[builder(into, default)]
        pub query_depth_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The maximum number of resolvers that can be invoked in a single request. The default value is `0` (or unspecified), which will set the limit to `10000`. When specified, the limit value can be between `1` and `10000`. This field will produce a limit error if the operation falls out of bounds.
        #[builder(into, default)]
        pub resolver_count_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Schema definition, in GraphQL schema language format. This provider cannot perform drift detection of this configuration.
        #[builder(into, default)]
        pub schema: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Amazon Cognito User Pool configuration. See `user_pool_config` Block for details.
        #[builder(into, default)]
        pub user_pool_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appsync::GraphQlApiUserPoolConfig>,
        >,
        /// Sets the value of the GraphQL API to public (`GLOBAL`) or private (`PRIVATE`). If no value is provided, the visibility will be set to `GLOBAL` by default. This value cannot be changed once the API has been created.
        #[builder(into, default)]
        pub visibility: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether tracing with X-ray is enabled. Defaults to false.
        #[builder(into, default)]
        pub xray_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GraphQLApiResult {
        /// One or more additional authentication providers for the GraphQL API. See `additional_authentication_provider` Block for details.
        pub additional_authentication_providers: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::appsync::GraphQlApiAdditionalAuthenticationProvider,
                >,
            >,
        >,
        /// API type. Valid values are `GRAPHQL` or `MERGED`. A `MERGED` type requires `merged_api_execution_role_arn` to be set.
        pub api_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Authentication type. Valid values: `API_KEY`, `AWS_IAM`, `AMAZON_COGNITO_USER_POOLS`, `OPENID_CONNECT`, `AWS_LAMBDA`
        pub authentication_type: pulumi_gestalt_rust::Output<String>,
        /// Enables and controls the enhanced metrics feature. See `enhanced_metrics_config` Block for details.
        pub enhanced_metrics_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::appsync::GraphQlApiEnhancedMetricsConfig>,
        >,
        /// Sets the value of the GraphQL API to enable (`ENABLED`) or disable (`DISABLED`) introspection. If no value is provided, the introspection configuration will be set to ENABLED by default. This field will produce an error if the operation attempts to use the introspection feature while this field is disabled. For more information about introspection, see [GraphQL introspection](https://graphql.org/learn/introspection/).
        pub introspection_config: pulumi_gestalt_rust::Output<Option<String>>,
        /// Nested argument containing Lambda authorizer configuration. See `lambda_authorizer_config` Block for details.
        pub lambda_authorizer_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::appsync::GraphQlApiLambdaAuthorizerConfig>,
        >,
        /// Nested argument containing logging configuration. See `log_config` Block for details.
        pub log_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::appsync::GraphQlApiLogConfig>,
        >,
        /// ARN of the execution role when `api_type` is set to `MERGED`.
        pub merged_api_execution_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// User-supplied name for the GraphQL API.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Nested argument containing OpenID Connect configuration. See `openid_connect_config` Block for details.
        pub openid_connect_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::appsync::GraphQlApiOpenidConnectConfig>,
        >,
        /// The maximum depth a query can have in a single request. Depth refers to the amount of nested levels allowed in the body of query. The default value is `0` (or unspecified), which indicates there's no depth limit. If you set a limit, it can be between `1` and `75` nested levels. This field will produce a limit error if the operation falls out of bounds.
        ///
        /// Note that fields can still be set to nullable or non-nullable. If a non-nullable field produces an error, the error will be thrown upwards to the first nullable field available.
        pub query_depth_limit: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The maximum number of resolvers that can be invoked in a single request. The default value is `0` (or unspecified), which will set the limit to `10000`. When specified, the limit value can be between `1` and `10000`. This field will produce a limit error if the operation falls out of bounds.
        pub resolver_count_limit: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Schema definition, in GraphQL schema language format. This provider cannot perform drift detection of this configuration.
        pub schema: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Map of URIs associated with the API E.g., `uris["GRAPHQL"] = https://ID.appsync-api.REGION.amazonaws.com/graphql`
        pub uris: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Amazon Cognito User Pool configuration. See `user_pool_config` Block for details.
        pub user_pool_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::appsync::GraphQlApiUserPoolConfig>,
        >,
        /// Sets the value of the GraphQL API to public (`GLOBAL`) or private (`PRIVATE`). If no value is provided, the visibility will be set to `GLOBAL` by default. This value cannot be changed once the API has been created.
        pub visibility: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether tracing with X-ray is enabled. Defaults to false.
        pub xray_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GraphQLApiArgs,
    ) -> GraphQLApiResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_authentication_providers_binding = args
            .additional_authentication_providers
            .get_output(context);
        let api_type_binding = args.api_type.get_output(context);
        let authentication_type_binding = args.authentication_type.get_output(context);
        let enhanced_metrics_config_binding = args
            .enhanced_metrics_config
            .get_output(context);
        let introspection_config_binding = args.introspection_config.get_output(context);
        let lambda_authorizer_config_binding = args
            .lambda_authorizer_config
            .get_output(context);
        let log_config_binding = args.log_config.get_output(context);
        let merged_api_execution_role_arn_binding = args
            .merged_api_execution_role_arn
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let openid_connect_config_binding = args
            .openid_connect_config
            .get_output(context);
        let query_depth_limit_binding = args.query_depth_limit.get_output(context);
        let resolver_count_limit_binding = args.resolver_count_limit.get_output(context);
        let schema_binding = args.schema.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_pool_config_binding = args.user_pool_config.get_output(context);
        let visibility_binding = args.visibility.get_output(context);
        let xray_enabled_binding = args.xray_enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appsync/graphQLApi:GraphQLApi".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalAuthenticationProviders".into(),
                    value: additional_authentication_providers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiType".into(),
                    value: api_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationType".into(),
                    value: authentication_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enhancedMetricsConfig".into(),
                    value: enhanced_metrics_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "introspectionConfig".into(),
                    value: introspection_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lambdaAuthorizerConfig".into(),
                    value: lambda_authorizer_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logConfig".into(),
                    value: log_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mergedApiExecutionRoleArn".into(),
                    value: merged_api_execution_role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "openidConnectConfig".into(),
                    value: openid_connect_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryDepthLimit".into(),
                    value: query_depth_limit_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resolverCountLimit".into(),
                    value: resolver_count_limit_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schema".into(),
                    value: schema_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPoolConfig".into(),
                    value: user_pool_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "visibility".into(),
                    value: visibility_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "xrayEnabled".into(),
                    value: xray_enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GraphQLApiResult {
            additional_authentication_providers: o
                .get_field("additionalAuthenticationProviders"),
            api_type: o.get_field("apiType"),
            arn: o.get_field("arn"),
            authentication_type: o.get_field("authenticationType"),
            enhanced_metrics_config: o.get_field("enhancedMetricsConfig"),
            introspection_config: o.get_field("introspectionConfig"),
            lambda_authorizer_config: o.get_field("lambdaAuthorizerConfig"),
            log_config: o.get_field("logConfig"),
            merged_api_execution_role_arn: o.get_field("mergedApiExecutionRoleArn"),
            name: o.get_field("name"),
            openid_connect_config: o.get_field("openidConnectConfig"),
            query_depth_limit: o.get_field("queryDepthLimit"),
            resolver_count_limit: o.get_field("resolverCountLimit"),
            schema: o.get_field("schema"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            uris: o.get_field("uris"),
            user_pool_config: o.get_field("userPoolConfig"),
            visibility: o.get_field("visibility"),
            xray_enabled: o.get_field("xrayEnabled"),
        }
    }
}
