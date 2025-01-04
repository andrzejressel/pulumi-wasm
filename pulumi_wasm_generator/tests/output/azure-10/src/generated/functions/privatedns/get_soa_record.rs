pub mod get_soa_record {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSoaRecordArgs {
        /// The name of the Private DNS SOA Record.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the resource group where the Private DNS Zone (parent resource) exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Private DNS Zone where the resource exists.
        #[builder(into)]
        pub zone_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSoaRecordResult {
        /// The email contact for the SOA record.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The expire time for the SOA record.
        pub expire_time: pulumi_wasm_rust::Output<i32>,
        /// The FQDN of the Private DNS SOA Record.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The domain name of the authoritative name server for the SOA record.
        pub host_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The minimum Time To Live for the SOA record. By convention, it is used to determine the negative caching duration.
        pub minimum_ttl: pulumi_wasm_rust::Output<i32>,
        /// The name of the Private DNS SOA Record.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The refresh time for the SOA record.
        pub refresh_time: pulumi_wasm_rust::Output<i32>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The retry time for the SOA record.
        pub retry_time: pulumi_wasm_rust::Output<i32>,
        /// The serial number for the SOA record.
        pub serial_number: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The Time To Live (TTL) of the Private DNS record in seconds.
        pub ttl: pulumi_wasm_rust::Output<i32>,
        pub zone_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSoaRecordArgs) -> GetSoaRecordResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let zone_name_binding = args.zone_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:privatedns/getSoaRecord:getSoaRecord".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "zoneName".into(),
                    value: &zone_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "expireTime".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "hostName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "minimumTtl".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "refreshTime".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "retryTime".into(),
                },
                register_interface::ResultField {
                    name: "serialNumber".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
                register_interface::ResultField {
                    name: "zoneName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSoaRecordResult {
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expireTime").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            host_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            minimum_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumTtl").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            refresh_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("refreshTime").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            retry_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retryTime").unwrap(),
            ),
            serial_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serialNumber").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ttl").unwrap(),
            ),
            zone_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneName").unwrap(),
            ),
        }
    }
}
