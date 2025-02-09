/// Provides an IAM OpenID Connect provider.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = open_id_connect_provider::create(
///         "default",
///         OpenIdConnectProviderArgs::builder()
///             .client_id_lists(
///                 vec!["266362248691-342342xasdasdasda-apps.googleusercontent.com",],
///             )
///             .thumbprint_lists(vec!["cf23df2207d99a74fbe169e3eba035e633b65d94",])
///             .url("https://accounts.google.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Without A Thumbprint
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = open_id_connect_provider::create(
///         "default",
///         OpenIdConnectProviderArgs::builder()
///             .client_id_lists(
///                 vec!["266362248691-342342xasdasdasda-apps.googleusercontent.com",],
///             )
///             .url("https://accounts.google.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM OpenID Connect Providers using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/openIdConnectProvider:OpenIdConnectProvider default arn:aws:iam::123456789012:oidc-provider/accounts.google.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod open_id_connect_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OpenIdConnectProviderArgs {
        /// List of client IDs (audiences) that identify the application registered with the OpenID Connect provider. This is the value sent as the `client_id` parameter in OAuth requests.
        #[builder(into)]
        pub client_id_lists: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Map of resource tags for the IAM OIDC provider. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub thumbprint_lists: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// URL of the identity provider, corresponding to the `iss` claim.
        #[builder(into)]
        pub url: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OpenIdConnectProviderResult {
        /// ARN assigned by AWS for this provider.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of client IDs (audiences) that identify the application registered with the OpenID Connect provider. This is the value sent as the `client_id` parameter in OAuth requests.
        pub client_id_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Map of resource tags for the IAM OIDC provider. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub thumbprint_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// URL of the identity provider, corresponding to the `iss` claim.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OpenIdConnectProviderArgs,
    ) -> OpenIdConnectProviderResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let client_id_lists_binding = args.client_id_lists.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let thumbprint_lists_binding = args.thumbprint_lists.get_output(context);
        let url_binding = args.url.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/openIdConnectProvider:OpenIdConnectProvider".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientIdLists".into(),
                    value: client_id_lists_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thumbprintLists".into(),
                    value: thumbprint_lists_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "url".into(),
                    value: url_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OpenIdConnectProviderResult {
            arn: o.get_field("arn"),
            client_id_lists: o.get_field("clientIdLists"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            thumbprint_lists: o.get_field("thumbprintLists"),
            url: o.get_field("url"),
        }
    }
}
