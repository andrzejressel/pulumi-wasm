pub mod get_dedicated_ip_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDedicatedIpPoolArgs {
        /// Name of the dedicated IP pool.
        #[builder(into)]
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags attached to the pool.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDedicatedIpPoolResult {
        /// ARN of the Dedicated IP Pool.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A list of objects describing the pool's dedicated IP's. See `dedicated_ips`.
        pub dedicated_ips: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sesv2::GetDedicatedIpPoolDedicatedIp>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub pool_name: pulumi_wasm_rust::Output<String>,
        /// (Optional) IP pool scaling mode. Valid values: `STANDARD`, `MANAGED`.
        pub scaling_mode: pulumi_wasm_rust::Output<String>,
        /// A map of tags attached to the pool.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDedicatedIpPoolArgs) -> GetDedicatedIpPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let pool_name_binding = args.pool_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sesv2/getDedicatedIpPool:getDedicatedIpPool".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "poolName".into(),
                    value: &pool_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "dedicatedIps".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "poolName".into(),
                },
                register_interface::ResultField {
                    name: "scalingMode".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDedicatedIpPoolResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            dedicated_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dedicatedIps").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            pool_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("poolName").unwrap(),
            ),
            scaling_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalingMode").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
