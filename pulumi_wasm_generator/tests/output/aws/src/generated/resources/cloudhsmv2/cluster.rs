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
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// The type of HSM module in the cluster. Currently, `hsm1.medium` and `hsm2m.medium` are supported.
        #[builder(into)]
        pub hsm_type: pulumi_wasm_rust::Output<String>,
        /// The mode to use in the cluster. The allowed values are `FIPS` and `NON_FIPS`. This field is required if `hsm_type` is `hsm2m.medium`.
        #[builder(into, default)]
        pub mode: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of Cloud HSM v2 cluster backup to be restored.
        #[builder(into, default)]
        pub source_backup_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The IDs of subnets in which cluster will operate.
        #[builder(into)]
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// The list of cluster certificates.
        pub cluster_certificates: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudhsmv2::ClusterClusterCertificate>,
        >,
        /// The id of the CloudHSM cluster.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The state of the CloudHSM cluster.
        pub cluster_state: pulumi_wasm_rust::Output<String>,
        /// The type of HSM module in the cluster. Currently, `hsm1.medium` and `hsm2m.medium` are supported.
        pub hsm_type: pulumi_wasm_rust::Output<String>,
        /// The mode to use in the cluster. The allowed values are `FIPS` and `NON_FIPS`. This field is required if `hsm_type` is `hsm2m.medium`.
        pub mode: pulumi_wasm_rust::Output<String>,
        /// The ID of the security group associated with the CloudHSM cluster.
        pub security_group_id: pulumi_wasm_rust::Output<String>,
        /// ID of Cloud HSM v2 cluster backup to be restored.
        pub source_backup_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// The IDs of subnets in which cluster will operate.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The id of the VPC that the CloudHSM cluster resides in.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterArgs) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hsm_type_binding = args.hsm_type.get_inner();
        let mode_binding = args.mode.get_inner();
        let source_backup_identifier_binding = args.source_backup_identifier.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudhsmv2/cluster:Cluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hsmType".into(),
                    value: &hsm_type_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "sourceBackupIdentifier".into(),
                    value: &source_backup_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterCertificates".into(),
                },
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "clusterState".into(),
                },
                register_interface::ResultField {
                    name: "hsmType".into(),
                },
                register_interface::ResultField {
                    name: "mode".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "sourceBackupIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterResult {
            cluster_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterCertificates").unwrap(),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            cluster_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterState").unwrap(),
            ),
            hsm_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hsmType").unwrap(),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mode").unwrap(),
            ),
            security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupId").unwrap(),
            ),
            source_backup_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceBackupIdentifier").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}