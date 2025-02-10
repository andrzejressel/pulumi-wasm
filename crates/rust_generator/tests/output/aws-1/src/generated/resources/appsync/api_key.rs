/// Provides an AppSync API Key.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = graph_ql_api::create(
///         "example",
///         GraphQlApiArgs::builder()
///             .authentication_type("API_KEY")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleApiKey = api_key::create(
///         "exampleApiKey",
///         ApiKeyArgs::builder()
///             .api_id("${example.id}")
///             .expires("2018-05-03T04:00:00Z")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appsync_api_key` using the AppSync API ID and key separated by `:`. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/apiKey:ApiKey example xxxxx:yyyyy
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiKeyArgs {
        /// ID of the associated AppSync API
        #[builder(into)]
        pub api_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// API key description. Defaults to "Managed by Pulumi".
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// RFC3339 string representation of the expiry date. Rounded down to nearest hour. By default, it is 7 days from the date of creation.
        #[builder(into, default)]
        pub expires: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiKeyResult {
        /// ID of the associated AppSync API
        pub api_id: pulumi_gestalt_rust::Output<String>,
        pub api_key_id: pulumi_gestalt_rust::Output<String>,
        /// API key description. Defaults to "Managed by Pulumi".
        pub description: pulumi_gestalt_rust::Output<String>,
        /// RFC3339 string representation of the expiry date. Rounded down to nearest hour. By default, it is 7 days from the date of creation.
        pub expires: pulumi_gestalt_rust::Output<Option<String>>,
        /// API key
        pub key: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApiKeyArgs,
    ) -> ApiKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_id_binding = args.api_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let expires_binding = args.expires.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appsync/apiKey:ApiKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiId".into(),
                    value: api_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expires".into(),
                    value: expires_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApiKeyResult {
            api_id: o.get_field("apiId"),
            api_key_id: o.get_field("apiKeyId"),
            description: o.get_field("description"),
            expires: o.get_field("expires"),
            key: o.get_field("key"),
        }
    }
}
