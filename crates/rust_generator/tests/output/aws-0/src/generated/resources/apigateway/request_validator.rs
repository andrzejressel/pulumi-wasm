/// Manages an API Gateway Request Validator.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod request_validator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RequestValidatorArgs {
        /// Name of the request validator
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the associated Rest API
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Boolean whether to validate request body. Defaults to `false`.
        #[builder(into, default)]
        pub validate_request_body: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Boolean whether to validate request parameters. Defaults to `false`.
        #[builder(into, default)]
        pub validate_request_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct RequestValidatorResult {
        /// Name of the request validator
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the associated Rest API
        pub rest_api: pulumi_gestalt_rust::Output<String>,
        /// Boolean whether to validate request body. Defaults to `false`.
        pub validate_request_body: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Boolean whether to validate request parameters. Defaults to `false`.
        pub validate_request_parameters: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RequestValidatorArgs,
    ) -> RequestValidatorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let rest_api_binding = args.rest_api.get_output(context);
        let validate_request_body_binding = args
            .validate_request_body
            .get_output(context);
        let validate_request_parameters_binding = args
            .validate_request_parameters
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/requestValidator:RequestValidator".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validateRequestBody".into(),
                    value: &validate_request_body_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validateRequestParameters".into(),
                    value: &validate_request_parameters_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RequestValidatorResult {
            name: o.get_field("name"),
            rest_api: o.get_field("restApi"),
            validate_request_body: o.get_field("validateRequestBody"),
            validate_request_parameters: o.get_field("validateRequestParameters"),
        }
    }
}
