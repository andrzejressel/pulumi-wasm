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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod authorizer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorizerArgs {
        /// The ARN of the authorizer's Lambda function.
        #[builder(into)]
        pub authorizer_function_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether the HTTP caching is enabled or not. Default: `false`.
        #[builder(into, default)]
        pub enable_caching_for_http: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the authorizer.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether AWS IoT validates the token signature in an authorization request. Default: `false`.
        #[builder(into, default)]
        pub signing_disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The status of Authorizer request at creation. Valid values: `ACTIVE`, `INACTIVE`. Default: `ACTIVE`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the token key used to extract the token from the HTTP headers. This value is required if signing is enabled in your authorizer.
        #[builder(into, default)]
        pub token_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The public keys used to verify the digital signature returned by your custom authentication service. This value is required if signing is enabled in your authorizer.
        #[builder(into, default)]
        pub token_signing_public_keys: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AuthorizerResult {
        /// The ARN of the authorizer.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the authorizer's Lambda function.
        pub authorizer_function_arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the HTTP caching is enabled or not. Default: `false`.
        pub enable_caching_for_http: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the authorizer.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether AWS IoT validates the token signature in an authorization request. Default: `false`.
        pub signing_disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The status of Authorizer request at creation. Valid values: `ACTIVE`, `INACTIVE`. Default: `ACTIVE`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the token key used to extract the token from the HTTP headers. This value is required if signing is enabled in your authorizer.
        pub token_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The public keys used to verify the digital signature returned by your custom authentication service. This value is required if signing is enabled in your authorizer.
        pub token_signing_public_keys: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthorizerArgs,
    ) -> AuthorizerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorizer_function_arn_binding = args
            .authorizer_function_arn
            .get_output(context);
        let enable_caching_for_http_binding = args
            .enable_caching_for_http
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let signing_disabled_binding = args.signing_disabled.get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let token_key_name_binding = args.token_key_name.get_output(context);
        let token_signing_public_keys_binding = args
            .token_signing_public_keys
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/authorizer:Authorizer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizerFunctionArn".into(),
                    value: &authorizer_function_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableCachingForHttp".into(),
                    value: &enable_caching_for_http_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signingDisabled".into(),
                    value: &signing_disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenKeyName".into(),
                    value: &token_key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenSigningPublicKeys".into(),
                    value: &token_signing_public_keys_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AuthorizerResult {
            arn: o.get_field("arn"),
            authorizer_function_arn: o.get_field("authorizerFunctionArn"),
            enable_caching_for_http: o.get_field("enableCachingForHttp"),
            name: o.get_field("name"),
            signing_disabled: o.get_field("signingDisabled"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            token_key_name: o.get_field("tokenKeyName"),
            token_signing_public_keys: o.get_field("tokenSigningPublicKeys"),
        }
    }
}
