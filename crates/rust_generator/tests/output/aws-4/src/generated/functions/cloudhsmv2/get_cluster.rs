#[allow(clippy::doc_lazy_continuation)]
pub mod get_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// ID of Cloud HSM v2 cluster.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// State of the cluster to be found.
        #[builder(into, default)]
        pub cluster_state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// The list of cluster certificates.
        pub cluster_certificates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudhsmv2::GetClusterClusterCertificate>,
        >,
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        pub cluster_state: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ID of the security group associated with the CloudHSM cluster.
        pub security_group_id: pulumi_gestalt_rust::Output<String>,
        /// IDs of subnets in which cluster operates.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ID of the VPC that the CloudHSM cluster resides in.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_output(context).get_inner();
        let cluster_state_binding = args.cluster_state.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudhsmv2/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "clusterState".into(),
                    value: &cluster_state_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterResult {
            cluster_certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterCertificates"),
            ),
            cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterId"),
            ),
            cluster_state: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterState"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            security_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupId"),
            ),
            subnet_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
