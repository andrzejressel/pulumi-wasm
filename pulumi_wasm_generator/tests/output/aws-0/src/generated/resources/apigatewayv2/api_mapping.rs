/// Manages an Amazon API Gateway Version 2 API mapping.
/// More information can be found in the [Amazon API Gateway Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html).
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod api_mapping {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiMappingArgs {
        /// API identifier.
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The API mapping key. Refer to [REST API](https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-mappings.html), [HTTP API](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-mappings.html) or [WebSocket API](https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-mappings.html).
        #[builder(into, default)]
        pub api_mapping_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Domain name. Use the `aws.apigatewayv2.DomainName` resource to configure a domain name.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// API stage. Use the `aws.apigatewayv2.Stage` resource to configure an API stage.
        #[builder(into)]
        pub stage: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApiMappingResult {
        /// API identifier.
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// The API mapping key. Refer to [REST API](https://docs.aws.amazon.com/apigateway/latest/developerguide/rest-api-mappings.html), [HTTP API](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-mappings.html) or [WebSocket API](https://docs.aws.amazon.com/apigateway/latest/developerguide/websocket-api-mappings.html).
        pub api_mapping_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Domain name. Use the `aws.apigatewayv2.DomainName` resource to configure a domain name.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// API stage. Use the `aws.apigatewayv2.Stage` resource to configure an API stage.
        pub stage: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiMappingArgs) -> ApiMappingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let api_mapping_key_binding = args.api_mapping_key.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let stage_binding = args.stage.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigatewayv2/apiMapping:ApiMapping".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "apiMappingKey".into(),
                    value: &api_mapping_key_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "stage".into(),
                    value: &stage_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "apiMappingKey".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "stage".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiMappingResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            api_mapping_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiMappingKey").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            stage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stage").unwrap(),
            ),
        }
    }
}
