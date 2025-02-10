/// Creates an Amazon CloudHSM v2 cluster.
///
/// For information about CloudHSM v2, see the
/// [AWS CloudHSM User Guide](https://docs.aws.amazon.com/cloudhsm/latest/userguide/introduction.html) and the [Amazon
/// CloudHSM API Reference][2].
///
/// > **NOTE:** A CloudHSM Cluster can take several minutes to set up.
/// Practically no single attribute can be updated, except for `tags`.
/// If you need to delete a cluster, you have to remove its HSM modules first.
/// To initialize cluster, you have to add an HSM instance to the cluster, then sign CSR and upload it.
///
/// ## Import
///
/// Using `pulumi import`, import CloudHSM v2 Clusters using the cluster `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudhsmv2/cluster:Cluster test_cluster cluster-aeb282a201
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// The type of HSM module in the cluster. Currently, `hsm1.medium` and `hsm2m.medium` are supported.
        #[builder(into)]
        pub hsm_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The mode to use in the cluster. The allowed values are `FIPS` and `NON_FIPS`. This field is required if `hsm_type` is `hsm2m.medium`.
        #[builder(into, default)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of Cloud HSM v2 cluster backup to be restored.
        #[builder(into, default)]
        pub source_backup_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IDs of subnets in which cluster will operate.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// The list of cluster certificates.
        pub cluster_certificates: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudhsmv2::ClusterClusterCertificate>,
        >,
        /// The id of the CloudHSM cluster.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The state of the CloudHSM cluster.
        pub cluster_state: pulumi_gestalt_rust::Output<String>,
        /// The type of HSM module in the cluster. Currently, `hsm1.medium` and `hsm2m.medium` are supported.
        pub hsm_type: pulumi_gestalt_rust::Output<String>,
        /// The mode to use in the cluster. The allowed values are `FIPS` and `NON_FIPS`. This field is required if `hsm_type` is `hsm2m.medium`.
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// The ID of the security group associated with the CloudHSM cluster.
        pub security_group_id: pulumi_gestalt_rust::Output<String>,
        /// ID of Cloud HSM v2 cluster backup to be restored.
        pub source_backup_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IDs of subnets in which cluster will operate.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The id of the VPC that the CloudHSM cluster resides in.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hsm_type_binding = args.hsm_type.get_output(context);
        let mode_binding = args.mode.get_output(context);
        let source_backup_identifier_binding = args
            .source_backup_identifier
            .get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudhsmv2/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hsmType".into(),
                    value: hsm_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceBackupIdentifier".into(),
                    value: source_backup_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: subnet_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            cluster_certificates: o.get_field("clusterCertificates"),
            cluster_id: o.get_field("clusterId"),
            cluster_state: o.get_field("clusterState"),
            hsm_type: o.get_field("hsmType"),
            mode: o.get_field("mode"),
            security_group_id: o.get_field("securityGroupId"),
            source_backup_identifier: o.get_field("sourceBackupIdentifier"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
