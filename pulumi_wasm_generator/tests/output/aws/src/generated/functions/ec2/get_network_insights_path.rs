pub mod get_network_insights_path {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkInsightsPathArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetNetworkInsightsPathFilter>>,
        >,
        /// ID of the Network Insights Path to select.
        #[builder(into, default)]
        pub network_insights_path_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetNetworkInsightsPathResult {
        /// ARN of the selected Network Insights Path.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// AWS resource that is the destination of the path.
        pub destination: pulumi_wasm_rust::Output<String>,
        /// ARN of the destination.
        pub destination_arn: pulumi_wasm_rust::Output<String>,
        /// IP address of the AWS resource that is the destination of the path.
        pub destination_ip: pulumi_wasm_rust::Output<String>,
        /// Destination port.
        pub destination_port: pulumi_wasm_rust::Output<i32>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetNetworkInsightsPathFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub network_insights_path_id: pulumi_wasm_rust::Output<String>,
        /// Protocol.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// AWS resource that is the source of the path.
        pub source: pulumi_wasm_rust::Output<String>,
        /// ARN of the source.
        pub source_arn: pulumi_wasm_rust::Output<String>,
        /// IP address of the AWS resource that is the source of the path.
        pub source_ip: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNetworkInsightsPathArgs) -> GetNetworkInsightsPathResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let network_insights_path_id_binding = args.network_insights_path_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getNetworkInsightsPath:getNetworkInsightsPath".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "networkInsightsPathId".into(),
                    value: &network_insights_path_id_binding,
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
                    name: "destination".into(),
                },
                register_interface::ResultField {
                    name: "destinationArn".into(),
                },
                register_interface::ResultField {
                    name: "destinationIp".into(),
                },
                register_interface::ResultField {
                    name: "destinationPort".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "networkInsightsPathId".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "sourceArn".into(),
                },
                register_interface::ResultField {
                    name: "sourceIp".into(),
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
        GetNetworkInsightsPathResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destination").unwrap(),
            ),
            destination_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationArn").unwrap(),
            ),
            destination_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationIp").unwrap(),
            ),
            destination_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationPort").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            network_insights_path_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInsightsPathId").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            source_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceArn").unwrap(),
            ),
            source_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceIp").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
