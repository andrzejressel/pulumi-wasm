/// Provides a Pinpoint ADM (Amazon Device Messaging) Channel resource.
///
/// > **Note:** All arguments including the Client ID and Client Secret will be stored in the raw state as plain-text.
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let app = app::create("app", AppArgs::builder().build_struct());
///     let channel = adm_channel::create(
///         "channel",
///         AdmChannelArgs::builder()
///             .application_id("${app.applicationId}")
///             .client_id("")
///             .client_secret("")
///             .enabled(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Pinpoint ADM Channel using the `application-id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/admChannel:AdmChannel channel application-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod adm_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AdmChannelArgs {
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Client ID (part of OAuth Credentials) obtained via Amazon Developer Account.
        #[builder(into)]
        pub client_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Client Secret (part of OAuth Credentials) obtained via Amazon Developer Account.
        #[builder(into)]
        pub client_secret: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether to enable the channel. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct AdmChannelResult {
        /// The application ID.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// Client ID (part of OAuth Credentials) obtained via Amazon Developer Account.
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// Client Secret (part of OAuth Credentials) obtained via Amazon Developer Account.
        pub client_secret: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether to enable the channel. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AdmChannelArgs,
    ) -> AdmChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let client_id_binding = args.client_id.get_output(context);
        let client_secret_binding = args.client_secret.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pinpoint/admChannel:AdmChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientId".into(),
                    value: client_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientSecret".into(),
                    value: client_secret_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AdmChannelResult {
            application_id: o.get_field("applicationId"),
            client_id: o.get_field("clientId"),
            client_secret: o.get_field("clientSecret"),
            enabled: o.get_field("enabled"),
        }
    }
}
