pub mod get_devices {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDevicesArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDevicesResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub devices: pulumi_wasm_rust::Output<
            Vec<super::super::types::GetDevicesDevice>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDevicesArgs) -> GetDevicesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getDevices:getDevices".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "devices".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDevicesResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("devices").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}