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
pub mod gcm_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GcmChannelArgs {
        /// Platform credential API key from Google.
        #[builder(into, default)]
        pub api_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub default_authentication_method: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub service_json: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GcmChannelResult {
        /// Platform credential API key from Google.
        pub api_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The application ID.
        pub application_id: pulumi_wasm_rust::Output<String>,
        pub default_authentication_method: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the channel is enabled or disabled. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub service_json: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GcmChannelArgs) -> GcmChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_key_binding = args.api_key.get_inner();
        let application_id_binding = args.application_id.get_inner();
        let default_authentication_method_binding = args
            .default_authentication_method
            .get_inner();
        let enabled_binding = args.enabled.get_inner();
        let service_json_binding = args.service_json.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/gcmChannel:GcmChannel".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiKey".into(),
                },
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "defaultAuthenticationMethod".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "serviceJson".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GcmChannelResult {
            api_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKey").unwrap(),
            ),
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            default_authentication_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultAuthenticationMethod").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            service_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceJson").unwrap(),
            ),
        }
    }
}
