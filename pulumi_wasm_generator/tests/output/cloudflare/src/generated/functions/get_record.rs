pub mod get_record {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRecordArgs {
        /// Content to filter record results on.
        #[builder(into, default)]
        pub content: pulumi_wasm_rust::Output<Option<String>>,
        /// Hostname to filter DNS record results on.
        #[builder(into)]
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// DNS priority to filter record results on.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// DNS record type to filter record results on. Defaults to `A`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetRecordResult {
        /// Content to filter record results on.
        pub content: pulumi_wasm_rust::Output<Option<String>>,
        /// Hostname to filter DNS record results on.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// DNS priority to filter record results on.
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// Proxiable status of the found DNS record.
        pub proxiable: pulumi_wasm_rust::Output<bool>,
        /// Proxied status of the found DNS record.
        pub proxied: pulumi_wasm_rust::Output<bool>,
        /// TTL of the found DNS record.
        pub ttl: pulumi_wasm_rust::Output<i32>,
        /// DNS record type to filter record results on. Defaults to `A`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// Value of the found DNS record.
        pub value: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRecordArgs) -> GetRecordResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_binding = args.content.get_inner();
        let hostname_binding = args.hostname.get_inner();
        let priority_binding = args.priority.get_inner();
        let type__binding = args.type_.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getRecord:getRecord".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "content".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "proxiable".into(),
                },
                register_interface::ResultField {
                    name: "proxied".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRecordResult {
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            proxiable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxiable").unwrap(),
            ),
            proxied: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxied").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ttl").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
