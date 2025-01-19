pub mod get_accelerator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAcceleratorArgs {
        /// Full ARN of the Global Accelerator.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique name of the Global Accelerator.
        ///
        /// > **NOTE:** When both `arn` and `name` are specified, `arn` takes precedence.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAcceleratorResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub attributes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::globalaccelerator::GetAcceleratorAttribute>,
        >,
        pub dns_name: pulumi_wasm_rust::Output<String>,
        pub dual_stack_dns_name: pulumi_wasm_rust::Output<String>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        pub ip_address_type: pulumi_wasm_rust::Output<String>,
        pub ip_sets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::globalaccelerator::GetAcceleratorIpSet>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAcceleratorArgs) -> GetAcceleratorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let id_binding = args.id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:globalaccelerator/getAccelerator:getAccelerator".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "dualStackDnsName".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "hostedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipAddressType".into(),
                },
                register_interface::ResultField {
                    name: "ipSets".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        GetAcceleratorResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            dual_stack_dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dualStackDnsName").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZoneId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddressType").unwrap(),
            ),
            ip_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipSets").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
