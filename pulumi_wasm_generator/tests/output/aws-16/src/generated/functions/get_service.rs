pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// DNS name of the service (_e.g.,_ `rds.us-east-1.amazonaws.com`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.
        #[builder(into, default)]
        pub dns_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region of the service (_e.g.,_ `us-west-2`, `ap-northeast-1`).
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Reverse DNS name of the service (_e.g.,_ `com.amazonaws.us-west-2.s3`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.
        #[builder(into, default)]
        pub reverse_dns_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Prefix of the service (_e.g.,_ `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
        #[builder(into, default)]
        pub reverse_dns_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Service endpoint ID (_e.g.,_ `s3`, `rds`, `ec2`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required. A service's endpoint ID can be found in the [_AWS General Reference_](https://docs.aws.amazon.com/general/latest/gr/aws-service-information.html).
        #[builder(into, default)]
        pub service_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        pub dns_name: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        pub partition: pulumi_wasm_rust::Output<String>,
        pub region: pulumi_wasm_rust::Output<String>,
        pub reverse_dns_name: pulumi_wasm_rust::Output<String>,
        pub reverse_dns_prefix: pulumi_wasm_rust::Output<String>,
        pub service_id: pulumi_wasm_rust::Output<String>,
        /// Whether the service is supported in the region's partition. New services may not be listed immediately as supported.
        pub supported: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dns_name_binding = args.dns_name.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let reverse_dns_name_binding = args
            .reverse_dns_name
            .get_output(context)
            .get_inner();
        let reverse_dns_prefix_binding = args
            .reverse_dns_prefix
            .get_output(context)
            .get_inner();
        let service_id_binding = args.service_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:index/getService:getService".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsName".into(),
                    value: &dns_name_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "reverseDnsName".into(),
                    value: &reverse_dns_name_binding,
                },
                register_interface::ObjectField {
                    name: "reverseDnsPrefix".into(),
                    value: &reverse_dns_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "partition".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "reverseDnsName".into(),
                },
                register_interface::ResultField {
                    name: "reverseDnsPrefix".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
                register_interface::ResultField {
                    name: "supported".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceResult {
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            partition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partition").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            reverse_dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reverseDnsName").unwrap(),
            ),
            reverse_dns_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reverseDnsPrefix").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
            supported: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supported").unwrap(),
            ),
        }
    }
}
