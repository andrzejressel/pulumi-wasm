#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_broker_nodes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBrokerNodesArgs {
        /// ARN of the cluster the nodes belong to.
        #[builder(into)]
        pub cluster_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBrokerNodesResult {
        pub cluster_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub node_info_lists: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::msk::GetBrokerNodesNodeInfoList>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBrokerNodesArgs,
    ) -> GetBrokerNodesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_arn_binding = args.cluster_arn.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:msk/getBrokerNodes:getBrokerNodes".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterArn".into(),
                    value: &cluster_arn_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBrokerNodesResult {
            cluster_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterArn"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            node_info_lists: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeInfoLists"),
            ),
        }
    }
}
