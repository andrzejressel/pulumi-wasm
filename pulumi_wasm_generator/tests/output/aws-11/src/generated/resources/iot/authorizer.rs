/// Creates and manages an AWS IoT Authorizer.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iot:Authorizer
///     properties:
///       name: example
///       authorizerFunctionArn: ${exampleAwsLambdaFunction.arn}
///       signingDisabled: false
///       status: ACTIVE
///       tokenKeyName: Token-Header
///       tokenSigningPublicKeys:
///         Key1:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: test-fixtures/iot-authorizer-signing-key.pem
///             return: result
///       tags:
///         Name: example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IOT Authorizers using the name. For example:
///
/// ```sh
/// $ pulumi import aws:iot/authorizer:Authorizer example example
/// ```
pub mod authorizer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizerArgs {
        /// The ARN of the authorizer's Lambda function.
        #[builder(into)]
        pub authorizer_function_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies whether the HTTP caching is enabled or not. Default: `false`.
        #[builder(into, default)]
        pub enable_caching_for_http: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the authorizer.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies whether AWS IoT validates the token signature in an authorization request. Default: `false`.
        #[builder(into, default)]
        pub signing_disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The status of Authorizer request at creation. Valid values: `ACTIVE`, `INACTIVE`. Default: `ACTIVE`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the token key used to extract the token from the HTTP headers. This value is required if signing is enabled in your authorizer.
        #[builder(into, default)]
        pub token_key_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The public keys used to verify the digital signature returned by your custom authentication service. This value is required if signing is enabled in your authorizer.
        #[builder(into, default)]
        pub token_signing_public_keys: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AuthorizerResult {
        /// The ARN of the authorizer.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the authorizer's Lambda function.
        pub authorizer_function_arn: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the HTTP caching is enabled or not. Default: `false`.
        pub enable_caching_for_http: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the authorizer.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies whether AWS IoT validates the token signature in an authorization request. Default: `false`.
        pub signing_disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The status of Authorizer request at creation. Valid values: `ACTIVE`, `INACTIVE`. Default: `ACTIVE`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the token key used to extract the token from the HTTP headers. This value is required if signing is enabled in your authorizer.
        pub token_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The public keys used to verify the digital signature returned by your custom authentication service. This value is required if signing is enabled in your authorizer.
        pub token_signing_public_keys: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AuthorizerArgs,
    ) -> AuthorizerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorizer_function_arn_binding = args
            .authorizer_function_arn
            .get_output(context)
            .get_inner();
        let enable_caching_for_http_binding = args
            .enable_caching_for_http
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let signing_disabled_binding = args
            .signing_disabled
            .get_output(context)
            .get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let token_key_name_binding = args.token_key_name.get_output(context).get_inner();
        let token_signing_public_keys_binding = args
            .token_signing_public_keys
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/authorizer:Authorizer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizerFunctionArn".into(),
                    value: &authorizer_function_arn_binding,
                },
                register_interface::ObjectField {
                    name: "enableCachingForHttp".into(),
                    value: &enable_caching_for_http_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "signingDisabled".into(),
                    value: &signing_disabled_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tokenKeyName".into(),
                    value: &token_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "tokenSigningPublicKeys".into(),
                    value: &token_signing_public_keys_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authorizerFunctionArn".into(),
                },
                register_interface::ResultField {
                    name: "enableCachingForHttp".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "signingDisabled".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tokenKeyName".into(),
                },
                register_interface::ResultField {
                    name: "tokenSigningPublicKeys".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthorizerResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authorizer_function_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizerFunctionArn").unwrap(),
            ),
            enable_caching_for_http: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableCachingForHttp").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            signing_disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingDisabled").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            token_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tokenKeyName").unwrap(),
            ),
            token_signing_public_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tokenSigningPublicKeys").unwrap(),
            ),
        }
    }
}
