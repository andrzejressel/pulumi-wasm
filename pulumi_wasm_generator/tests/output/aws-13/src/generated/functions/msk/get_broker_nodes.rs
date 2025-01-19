pub mod get_broker_nodes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBrokerNodesArgs {
        /// ARN of the cluster the nodes belong to.
        #[builder(into)]
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetBrokerNodesResult {
        pub cluster_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub node_info_lists: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::msk::GetBrokerNodesNodeInfoList>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBrokerNodesArgs) -> GetBrokerNodesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_arn_binding = args.cluster_arn.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:msk/getBrokerNodes:getBrokerNodes".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "nodeInfoLists".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBrokerNodesResult {
            cluster_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            node_info_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeInfoLists").unwrap(),
            ),
        }
    }
}
