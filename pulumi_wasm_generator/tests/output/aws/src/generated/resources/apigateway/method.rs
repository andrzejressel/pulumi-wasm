/// Provides a HTTP Method for an API Gateway Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
///       Function: aws:cognito:getUserPools
///       Arguments:
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
pub mod method {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MethodArgs {
        /// Specify if the method requires an API key
        #[builder(into, default)]
        pub api_key_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// Type of authorization used for the method (`NONE`, `CUSTOM`, `AWS_IAM`, `COGNITO_USER_POOLS`)
        #[builder(into)]
        pub authorization: pulumi_wasm_rust::Output<String>,
        /// Authorization scopes used when the authorization is `COGNITO_USER_POOLS`
        #[builder(into, default)]
        pub authorization_scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Authorizer id to be used when the authorization is `CUSTOM` or `COGNITO_USER_POOLS`
        #[builder(into, default)]
        pub authorizer_id: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP Method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `ANY`)
        #[builder(into)]
        pub http_method: pulumi_wasm_rust::Output<String>,
        /// Function name that will be given to the method when generating an SDK through API Gateway. If omitted, API Gateway will generate a function name based on the resource path and HTTP verb.
        #[builder(into, default)]
        pub operation_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of the API models used for the request's content type
        /// where key is the content type (e.g., `application/json`)
        /// and value is either `Error`, `Empty` (built-in models) or `aws.apigateway.Model`'s `name`.
        #[builder(into, default)]
        pub request_models: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of request parameters (from the path, query string and headers) that should be passed to the integration. The boolean value indicates whether the parameter is required (`true`) or optional (`false`).
        /// For example: `request_parameters = {"method.request.header.X-Some-Header" = true "method.request.querystring.some-query-param" = true}` would define that the header `X-Some-Header` and the query string `some-query-param` must be provided in the request.
        #[builder(into, default)]
        pub request_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, bool>>,
        >,
        /// ID of a `aws.apigateway.RequestValidator`
        #[builder(into, default)]
        pub request_validator_id: pulumi_wasm_rust::Output<Option<String>>,
        /// API resource ID
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// ID of the associated REST API
        #[builder(into)]
        pub rest_api: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MethodResult {
        /// Specify if the method requires an API key
        pub api_key_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// Type of authorization used for the method (`NONE`, `CUSTOM`, `AWS_IAM`, `COGNITO_USER_POOLS`)
        pub authorization: pulumi_wasm_rust::Output<String>,
        /// Authorization scopes used when the authorization is `COGNITO_USER_POOLS`
        pub authorization_scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Authorizer id to be used when the authorization is `CUSTOM` or `COGNITO_USER_POOLS`
        pub authorizer_id: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP Method (`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `ANY`)
        pub http_method: pulumi_wasm_rust::Output<String>,
        /// Function name that will be given to the method when generating an SDK through API Gateway. If omitted, API Gateway will generate a function name based on the resource path and HTTP verb.
        pub operation_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of the API models used for the request's content type
        /// where key is the content type (e.g., `application/json`)
        /// and value is either `Error`, `Empty` (built-in models) or `aws.apigateway.Model`'s `name`.
        pub request_models: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of request parameters (from the path, query string and headers) that should be passed to the integration. The boolean value indicates whether the parameter is required (`true`) or optional (`false`).
        /// For example: `request_parameters = {"method.request.header.X-Some-Header" = true "method.request.querystring.some-query-param" = true}` would define that the header `X-Some-Header` and the query string `some-query-param` must be provided in the request.
        pub request_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, bool>>,
        >,
        /// ID of a `aws.apigateway.RequestValidator`
        pub request_validator_id: pulumi_wasm_rust::Output<Option<String>>,
        /// API resource ID
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// ID of the associated REST API
        pub rest_api: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MethodArgs) -> MethodResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_key_required_binding = args.api_key_required.get_inner();
        let authorization_binding = args.authorization.get_inner();
        let authorization_scopes_binding = args.authorization_scopes.get_inner();
        let authorizer_id_binding = args.authorizer_id.get_inner();
        let http_method_binding = args.http_method.get_inner();
        let operation_name_binding = args.operation_name.get_inner();
        let request_models_binding = args.request_models.get_inner();
        let request_parameters_binding = args.request_parameters.get_inner();
        let request_validator_id_binding = args.request_validator_id.get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let rest_api_binding = args.rest_api.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apigateway/method:Method".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiKeyRequired".into(),
                    value: &api_key_required_binding,
                },
                register_interface::ObjectField {
                    name: "authorization".into(),
                    value: &authorization_binding,
                },
                register_interface::ObjectField {
                    name: "authorizationScopes".into(),
                    value: &authorization_scopes_binding,
                },
                register_interface::ObjectField {
                    name: "authorizerId".into(),
                    value: &authorizer_id_binding,
                },
                register_interface::ObjectField {
                    name: "httpMethod".into(),
                    value: &http_method_binding,
                },
                register_interface::ObjectField {
                    name: "operationName".into(),
                    value: &operation_name_binding,
                },
                register_interface::ObjectField {
                    name: "requestModels".into(),
                    value: &request_models_binding,
                },
                register_interface::ObjectField {
                    name: "requestParameters".into(),
                    value: &request_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "requestValidatorId".into(),
                    value: &request_validator_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiKeyRequired".into(),
                },
                register_interface::ResultField {
                    name: "authorization".into(),
                },
                register_interface::ResultField {
                    name: "authorizationScopes".into(),
                },
                register_interface::ResultField {
                    name: "authorizerId".into(),
                },
                register_interface::ResultField {
                    name: "httpMethod".into(),
                },
                register_interface::ResultField {
                    name: "operationName".into(),
                },
                register_interface::ResultField {
                    name: "requestModels".into(),
                },
                register_interface::ResultField {
                    name: "requestParameters".into(),
                },
                register_interface::ResultField {
                    name: "requestValidatorId".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "restApi".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MethodResult {
            api_key_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKeyRequired").unwrap(),
            ),
            authorization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorization").unwrap(),
            ),
            authorization_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizationScopes").unwrap(),
            ),
            authorizer_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerId").unwrap(),
            ),
            http_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpMethod").unwrap(),
            ),
            operation_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operationName").unwrap(),
            ),
            request_models: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestModels").unwrap(),
            ),
            request_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestParameters").unwrap(),
            ),
            request_validator_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestValidatorId").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            rest_api: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restApi").unwrap(),
            ),
        }
    }
}