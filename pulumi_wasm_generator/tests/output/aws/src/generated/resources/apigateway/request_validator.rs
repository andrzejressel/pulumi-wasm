/// Manages an API Gateway Request Validator.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = request_validator::create(
///         "example",
///         RequestValidatorArgs::builder()
///             .name("example")
///             .rest_api("${exampleAwsApiGatewayRestApi.id}")
///             .validate_request_body(true)
///             .validate_request_parameters(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_request_validator` using `REST-API-ID/REQUEST-VALIDATOR-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/requestValidator:RequestValidator example 12345abcde/67890fghij
/// ```
pub mod request_validator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RequestValidatorArgs {
        /// Name of the request validator
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the associated Rest API
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Boolean whether to validate request body. Defaults to `false`.
        #[builder(into, default)]
        pub validate_request_body: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean whether to validate request parameters. Defaults to `false`.
        #[builder(into, default)]
        pub validate_request_parameters: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct RequestValidatorResult {
        /// Name of the request validator
        pub name: pulumi_wasm_rust::Output<String>,
        /// ID of the associated Rest API
        pub rest_api: pulumi_wasm_rust::Output<String>,
        /// Boolean whether to validate request body. Defaults to `false`.
        pub validate_request_body: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean whether to validate request parameters. Defaults to `false`.
        pub validate_request_parameters: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RequestValidatorArgs) -> RequestValidatorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let validate_request_body_binding = args.validate_request_body.get_inner();
        let validate_request_parameters_binding = args
            .validate_request_parameters
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/requestValidator:RequestValidator".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
                register_interface::ObjectField {
                    name: "validateRequestBody".into(),
                    value: &validate_request_body_binding,
                },
                register_interface::ObjectField {
                    name: "validateRequestParameters".into(),
                    value: &validate_request_parameters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "restApi".into(),
                },
                register_interface::ResultField {
                    name: "validateRequestBody".into(),
                },
                register_interface::ResultField {
                    name: "validateRequestParameters".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RequestValidatorResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApi").unwrap(),
            ),
            validate_request_body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validateRequestBody").unwrap(),
            ),
            validate_request_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validateRequestParameters").unwrap(),
            ),
        }
    }
}
