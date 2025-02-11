/// Provides a HTTP Method for an API Gateway Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myDemoAPI = rest_api::create(
///         "myDemoAPI",
///         RestApiArgs::builder()
///             .description("This is my API for demonstration purposes")
///             .name("MyDemoAPI")
///             .build_struct(),
///     );
///     let myDemoMethod = method::create(
///         "myDemoMethod",
///         MethodArgs::builder()
///             .authorization("NONE")
///             .http_method("GET")
///             .resource_id("${myDemoResource.id}")
///             .rest_api("${myDemoAPI.id}")
///             .build_struct(),
///     );
///     let myDemoResource = resource::create(
///         "myDemoResource",
///         ResourceArgs::builder()
///             .parent_id("${myDemoAPI.rootResourceId}")
///             .path_part("mydemoresource")
///             .rest_api("${myDemoAPI.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Usage with Cognito User Pool Authorizer
///
/// ```yaml
/// configuration:
///   cognitoUserPoolName:
///     type: dynamic
/// resources:
///   thisRestApi:
///     type: aws:apigateway:RestApi
///     name: this
///     properties:
///       name: with-authorizer
///   thisResource:
///     type: aws:apigateway:Resource
///     name: this
///     properties:
///       restApi: ${thisRestApi.id}
///       parentId: ${thisRestApi.rootResourceId}
///       pathPart: '{proxy+}'
///   thisAuthorizer:
///     type: aws:apigateway:Authorizer
///     name: this
///     properties:
///       name: CognitoUserPoolAuthorizer
///       type: COGNITO_USER_POOLS
///       restApi: ${thisRestApi.id}
///       providerArns: ${this.arns}
///   any:
///     type: aws:apigateway:Method
///     properties:
///       restApi: ${thisRestApi.id}
///       resourceId: ${thisResource.id}
///       httpMethod: ANY
///       authorization: COGNITO_USER_POOLS
///       authorizerId: ${thisAuthorizer.id}
///       requestParameters:
///         method.request.path.proxy: true
/// variables:
///   this:
///     fn::invoke:
///       function: aws:cognito:getUserPools
///       arguments:
///         name: ${cognitoUserPoolName}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_method` using `REST-API-ID/RESOURCE-ID/HTTP-METHOD`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/method:Method example 12345abcde/67890fghij/GET
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod method {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MethodArgs {
        /// Specify if the method requires an API key
        #[builder(into, default)]
        pub api_key_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Type of authorization used for the method (`NONE`, `CUSTOM`, `AWS_IAM`, `COGNITO_USER_POOLS`)
        #[builder(into)]
        pub authorization: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Authorization scopes used when the authorization is `COGNITO_USER_POOLS`
        #[builder(into, default)]
        pub authorization_scopes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Authorizer id to be used when the authorization is `CUSTOM` or `COGNITO_USER_POOLS`
        #[builder(into, default)]
        pub authorizer_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// HTTP Method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `ANY`)
        #[builder(into)]
        pub http_method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Function name that will be given to the method when generating an SDK through API Gateway. If omitted, API Gateway will generate a function name based on the resource path and HTTP verb.
        #[builder(into, default)]
        pub operation_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of the API models used for the request's content type
        /// where key is the content type (e.g., `application/json`)
        /// and value is either `Error`, `Empty` (built-in models) or `aws.apigateway.Model`'s `name`.
        #[builder(into, default)]
        pub request_models: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of request parameters (from the path, query string and headers) that should be passed to the integration. The boolean value indicates whether the parameter is required (`true`) or optional (`false`).
        /// For example: `request_parameters = {"method.request.header.X-Some-Header" = true "method.request.querystring.some-query-param" = true}` would define that the header `X-Some-Header` and the query string `some-query-param` must be provided in the request.
        #[builder(into, default)]
        pub request_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, bool>>,
        >,
        /// ID of a `aws.apigateway.RequestValidator`
        #[builder(into, default)]
        pub request_validator_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// API resource ID
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MethodResult {
        /// Specify if the method requires an API key
        pub api_key_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Type of authorization used for the method (`NONE`, `CUSTOM`, `AWS_IAM`, `COGNITO_USER_POOLS`)
        pub authorization: pulumi_gestalt_rust::Output<String>,
        /// Authorization scopes used when the authorization is `COGNITO_USER_POOLS`
        pub authorization_scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Authorizer id to be used when the authorization is `CUSTOM` or `COGNITO_USER_POOLS`
        pub authorizer_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// HTTP Method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `ANY`)
        pub http_method: pulumi_gestalt_rust::Output<String>,
        /// Function name that will be given to the method when generating an SDK through API Gateway. If omitted, API Gateway will generate a function name based on the resource path and HTTP verb.
        pub operation_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of the API models used for the request's content type
        /// where key is the content type (e.g., `application/json`)
        /// and value is either `Error`, `Empty` (built-in models) or `aws.apigateway.Model`'s `name`.
        pub request_models: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of request parameters (from the path, query string and headers) that should be passed to the integration. The boolean value indicates whether the parameter is required (`true`) or optional (`false`).
        /// For example: `request_parameters = {"method.request.header.X-Some-Header" = true "method.request.querystring.some-query-param" = true}` would define that the header `X-Some-Header` and the query string `some-query-param` must be provided in the request.
        pub request_parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, bool>>,
        >,
        /// ID of a `aws.apigateway.RequestValidator`
        pub request_validator_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// API resource ID
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the associated REST API
        pub rest_api: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MethodArgs,
    ) -> MethodResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_key_required_binding = args.api_key_required.get_output(context);
        let authorization_binding = args.authorization.get_output(context);
        let authorization_scopes_binding = args.authorization_scopes.get_output(context);
        let authorizer_id_binding = args.authorizer_id.get_output(context);
        let http_method_binding = args.http_method.get_output(context);
        let operation_name_binding = args.operation_name.get_output(context);
        let request_models_binding = args.request_models.get_output(context);
        let request_parameters_binding = args.request_parameters.get_output(context);
        let request_validator_id_binding = args.request_validator_id.get_output(context);
        let resource_id_binding = args.resource_id.get_output(context);
        let rest_api_binding = args.rest_api.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/method:Method".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiKeyRequired".into(),
                    value: &api_key_required_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorization".into(),
                    value: &authorization_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizationScopes".into(),
                    value: &authorization_scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizerId".into(),
                    value: &authorizer_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpMethod".into(),
                    value: &http_method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operationName".into(),
                    value: &operation_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestModels".into(),
                    value: &request_models_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestParameters".into(),
                    value: &request_parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestValidatorId".into(),
                    value: &request_validator_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MethodResult {
            api_key_required: o.get_field("apiKeyRequired"),
            authorization: o.get_field("authorization"),
            authorization_scopes: o.get_field("authorizationScopes"),
            authorizer_id: o.get_field("authorizerId"),
            http_method: o.get_field("httpMethod"),
            operation_name: o.get_field("operationName"),
            request_models: o.get_field("requestModels"),
            request_parameters: o.get_field("requestParameters"),
            request_validator_id: o.get_field("requestValidatorId"),
            resource_id: o.get_field("resourceId"),
            rest_api: o.get_field("restApi"),
        }
    }
}
