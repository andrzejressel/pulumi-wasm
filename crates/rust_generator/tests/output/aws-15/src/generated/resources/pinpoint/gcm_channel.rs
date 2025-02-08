/// Provides a Pinpoint GCM Channel resource.
///
/// > **Note:** Credentials (Service Account JSON and API Key) will be stored in the raw state as plain-text.
/// ## Import
///
/// Using `pulumi import`, import Pinpoint GCM Channel using the `application-id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/gcmChannel:GcmChannel gcm application-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gcm_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GcmChannelArgs {
        /// Platform credential API key from Google.
        #[builder(into, default)]
        pub api_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub default_authentication_method: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub service_json: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GcmChannelResult {
        /// Platform credential API key from Google.
        pub api_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The application ID.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        pub default_authentication_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub service_json: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GcmChannelArgs,
    ) -> GcmChannelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_key_binding = args.api_key.get_output(context).get_inner();
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let default_authentication_method_binding = args
            .default_authentication_method
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let service_json_binding = args.service_json.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/gcmChannel:GcmChannel".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiKey".into(),
                    value: &api_key_binding,
                },
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "defaultAuthenticationMethod".into(),
                    value: &default_authentication_method_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "serviceJson".into(),
                    value: &service_json_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GcmChannelResult {
            api_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiKey"),
            ),
            application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            default_authentication_method: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultAuthenticationMethod"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            service_json: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceJson"),
            ),
        }
    }
}
