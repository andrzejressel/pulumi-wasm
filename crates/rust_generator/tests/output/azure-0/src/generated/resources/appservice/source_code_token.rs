/// Manages an App Service source control token.
///
/// !> **NOTE:** This resource has been deprecated in version 5.0 of the provider and will be removed in version 6.0. Please use `azure.appservice.ServicePlan` resource instead.
///
/// > **NOTE:** Source Control Tokens are configured at the subscription level, not on each App Service - as such this can only be configured Subscription-wide
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = source_code_token::create(
///         "example",
///         SourceCodeTokenArgs::builder()
///             .token("7e57735e77e577e57")
///             .type_("GitHub")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// App Service Source Control Token's can be imported using the `type`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/sourceCodeToken:SourceCodeToken example {type}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod source_code_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceCodeTokenArgs {
        /// The OAuth access token.
        #[builder(into)]
        pub token: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The OAuth access token secret.
        #[builder(into, default)]
        pub token_secret: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The source control type. Possible values are `BitBucket`, `Dropbox`, `GitHub` and `OneDrive`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SourceCodeTokenResult {
        /// The OAuth access token.
        pub token: pulumi_gestalt_rust::Output<String>,
        /// The OAuth access token secret.
        pub token_secret: pulumi_gestalt_rust::Output<Option<String>>,
        /// The source control type. Possible values are `BitBucket`, `Dropbox`, `GitHub` and `OneDrive`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SourceCodeTokenArgs,
    ) -> SourceCodeTokenResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let token_binding = args.token.get_output(context);
        let token_secret_binding = args.token_secret.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/sourceCodeToken:SourceCodeToken".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "token".into(),
                    value: &token_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenSecret".into(),
                    value: &token_secret_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SourceCodeTokenResult {
            token: o.get_field("token"),
            token_secret: o.get_field("tokenSecret"),
            type_: o.get_field("type"),
        }
    }
}
