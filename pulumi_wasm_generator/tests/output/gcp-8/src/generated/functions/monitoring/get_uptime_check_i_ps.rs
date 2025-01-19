pub mod get_uptime_check_i_ps {
    #[allow(dead_code)]
    pub struct GetUptimeCheckIPsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of uptime check IPs used by Stackdriver Monitoring. Each `uptime_check_ip` contains:
        pub uptime_check_ips: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetUptimeCheckIPsUptimeCheckIp>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke() -> GetUptimeCheckIPsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:monitoring/getUptimeCheckIPs:getUptimeCheckIPs".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "uptimeCheckIps".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetUptimeCheckIPsResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            uptime_check_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uptimeCheckIps").unwrap(),
            ),
        }
    }
}
