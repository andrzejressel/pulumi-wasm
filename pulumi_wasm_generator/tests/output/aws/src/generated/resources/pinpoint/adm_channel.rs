/// Provides a Pinpoint ADM (Amazon Device Messaging) Channel resource.
///
/// > **Note:** All arguments including the Client ID and Client Secret will be stored in the raw state as plain-text.
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod adm_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AdmChannelArgs {
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Client ID (part of OAuth Credentials) obtained via Amazon Developer Account.
        #[builder(into)]
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// Client Secret (part of OAuth Credentials) obtained via Amazon Developer Account.
        #[builder(into)]
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// Specifies whether to enable the channel. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct AdmChannelResult {
        /// The application ID.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Client ID (part of OAuth Credentials) obtained via Amazon Developer Account.
        pub client_id: pulumi_wasm_rust::Output<String>,
        /// Client Secret (part of OAuth Credentials) obtained via Amazon Developer Account.
        pub client_secret: pulumi_wasm_rust::Output<String>,
        /// Specifies whether to enable the channel. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AdmChannelArgs) -> AdmChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let client_id_binding = args.client_id.get_inner();
        let client_secret_binding = args.client_secret.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/admChannel:AdmChannel".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
                register_interface::ObjectField {
                    name: "clientSecret".into(),
                    value: &client_secret_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "clientId".into(),
                },
                register_interface::ResultField {
                    name: "clientSecret".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AdmChannelResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            client_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientId").unwrap(),
            ),
            client_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientSecret").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
        }
    }
}
