/// Manages an Amazon API Gateway Version 2 API mapping.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html).
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = api_mapping::create(
///         "example",
///         ApiMappingArgs::builder()
///             .api_id("${exampleAwsApigatewayv2Api.id}")
///             .domain_name("${exampleAwsApigatewayv2DomainName.id}")
///             .stage("${exampleAwsApigatewayv2Stage.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_apigatewayv2_api_mapping` using the API mapping identifier and domain name. For example:
///
/// ```sh
/// $ pulumi import aws:apigatewayv2/apiMapping:ApiMapping example 1122334/ws-api.example.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_mapping {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiMappingArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The API mapping key. Refer to [REST API](https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-mappings.html), [HTTP API](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-mappings.html) or [WebSocket API](https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-mappings.html).
        #[builder(into, default)]
        pub api_mapping_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Domain name. Use the `aws.apigatewayv2.DomainName` resource to configure a domain name.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// API stage. Use the `aws.apigatewayv2.Stage` resource to configure an API stage.
        #[builder(into)]
        pub stage: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApiMappingResult {
        /// API identifier.
        pub api_id: pulumi_gestalt_rust::Output<String>,
        /// The API mapping key. Refer to [REST API](https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-mappings.html), [HTTP API](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-mappings.html) or [WebSocket API](https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-mappings.html).
        pub api_mapping_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Domain name. Use the `aws.apigatewayv2.DomainName` resource to configure a domain name.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// API stage. Use the `aws.apigatewayv2.Stage` resource to configure an API stage.
        pub stage: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiMappingArgs,
    ) -> ApiMappingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let api_mapping_key_binding = args.api_mapping_key.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let stage_binding = args.stage.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigatewayv2/apiMapping:ApiMapping".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiMappingKey".into(),
                    value: &api_mapping_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stage".into(),
                    value: &stage_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiMappingResult {
            api_id: o.get_field("apiId"),
            api_mapping_key: o.get_field("apiMappingKey"),
            domain_name: o.get_field("domainName"),
            stage: o.get_field("stage"),
        }
    }
}
