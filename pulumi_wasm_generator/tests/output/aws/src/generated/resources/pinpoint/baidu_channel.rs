/// Provides a Pinpoint Baidu Channel resource.
///
/// > **Note:** All arguments including the Api Key and Secret Key will be stored in the raw state as plain-text.
/// ## Example Usage
///
/// ```yaml
/// resources:
///   app:
///     type: aws:pinpoint:App
///   channel:
///     type: aws:pinpoint:BaiduChannel
///     properties:
///       applicationId: ${app.applicationId}
///       apiKey:
///       secretKey:
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Pinpoint Baidu Channel using the `application-id`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/baiduChannel:BaiduChannel channel application-id
/// ```
pub mod baidu_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BaiduChannelArgs {
        /// Platform credential API key from Baidu.
        #[builder(into)]
        pub api_key: pulumi_wasm_rust::Output<String>,
        /// The application ID.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Specifies whether to enable the channel. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Platform credential Secret key from Baidu.
        #[builder(into)]
        pub secret_key: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct BaiduChannelResult {
        /// Platform credential API key from Baidu.
        pub api_key: pulumi_wasm_rust::Output<String>,
        /// The application ID.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Specifies whether to enable the channel. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Platform credential Secret key from Baidu.
        pub secret_key: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: BaiduChannelArgs) -> BaiduChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_key_binding = args.api_key.get_inner();
        let application_id_binding = args.application_id.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let secret_key_binding = args.secret_key.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/baiduChannel:BaiduChannel".into(),
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
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "secretKey".into(),
                    value: &secret_key_binding,
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
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "secretKey".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BaiduChannelResult {
            api_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKey").unwrap(),
            ),
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            secret_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretKey").unwrap(),
            ),
        }
    }
}