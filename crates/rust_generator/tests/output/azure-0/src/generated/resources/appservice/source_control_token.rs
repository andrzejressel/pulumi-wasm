/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = source_control_token::create(
///         "example",
///         SourceControlTokenArgs::builder()
///             .token("ghp_sometokenvaluesecretsauce")
///             .type_("GitHub")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// App Service Source GitHub Tokens can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/sourceControlToken:SourceControlToken example /providers/Microsoft.Web/sourceControls/GitHub
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod source_control_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceControlTokenArgs {
        /// The Access Token.
        #[builder(into)]
        pub token: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Access Token Secret.
        ///
        /// > **NOTE:** The token used for deploying App Service needs the following permissions: `repo` and `workflow`.
        #[builder(into, default)]
        pub token_secret: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Token type. Possible values include `Bitbucket`, `Dropbox`, `Github`, and `OneDrive`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SourceControlTokenResult {
        /// The Access Token.
        pub token: pulumi_gestalt_rust::Output<String>,
        /// The Access Token Secret.
        ///
        /// > **NOTE:** The token used for deploying App Service needs the following permissions: `repo` and `workflow`.
        pub token_secret: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Token type. Possible values include `Bitbucket`, `Dropbox`, `Github`, and `OneDrive`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SourceControlTokenArgs,
    ) -> SourceControlTokenResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let token_binding = args.token.get_output(context);
        let token_secret_binding = args.token_secret.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/sourceControlToken:SourceControlToken".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "token".into(),
                    value: token_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tokenSecret".into(),
                    value: token_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SourceControlTokenResult {
            token: o.get_field("token"),
            token_secret: o.get_field("tokenSecret"),
            type_: o.get_field("type"),
        }
    }
}
