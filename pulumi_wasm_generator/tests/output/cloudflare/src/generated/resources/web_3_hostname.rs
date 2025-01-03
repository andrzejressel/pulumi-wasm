/// Manages Web3 hostnames for IPFS and Ethereum gateways.
pub mod web_3_hostname {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Web3HostnameArgs {
        /// An optional description of the hostname.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// DNSLink value used if the target is ipfs.
        #[builder(into, default)]
        pub dnslink: pulumi_wasm_rust::Output<Option<String>>,
        /// The hostname that will point to the target gateway via CNAME.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Target gateway of the hostname.
        #[builder(into)]
        pub target: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct Web3HostnameResult {
        /// Creation time.
        pub created_on: pulumi_wasm_rust::Output<String>,
        /// An optional description of the hostname.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// DNSLink value used if the target is ipfs.
        pub dnslink: pulumi_wasm_rust::Output<Option<String>>,
        /// Last modification time.
        pub modified_on: pulumi_wasm_rust::Output<String>,
        /// The hostname that will point to the target gateway via CNAME.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Status of the hostname's activation.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Target gateway of the hostname.
        pub target: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: Web3HostnameArgs) -> Web3HostnameResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let dnslink_binding = args.dnslink.get_inner();
        let name_binding = args.name.get_inner();
        let target_binding = args.target.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/web3Hostname:Web3Hostname".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dnslink".into(),
                    value: &dnslink_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdOn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dnslink".into(),
                },
                register_interface::ResultField {
                    name: "modifiedOn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "target".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        Web3HostnameResult {
            created_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdOn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dnslink: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnslink").unwrap(),
            ),
            modified_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modifiedOn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
