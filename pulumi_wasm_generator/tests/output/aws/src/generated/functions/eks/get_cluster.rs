pub mod get_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Name of the cluster.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// Configuration block for access config.
        pub access_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetClusterAccessConfig>,
        >,
        /// ARN of the cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Nested attribute containing `certificate-authority-data` for your cluster.
        pub certificate_authorities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetClusterCertificateAuthority>,
        >,
        /// The ID of your local Amazon EKS cluster on the AWS Outpost. This attribute isn't available for an AWS EKS cluster on AWS cloud.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// Nested attribute containing compute capability configuration for EKS Auto Mode enabled cluster.
        pub compute_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetClusterComputeConfig>,
        >,
        /// Unix epoch time stamp in seconds for when the cluster was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The enabled control plane logs.
        pub enabled_cluster_log_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// Endpoint for your Kubernetes API server.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Nested attribute containing identity provider information for your cluster. Only available on Kubernetes version 1.13 and 1.14 clusters created or upgraded on or after September 3, 2019. For an example using this information to enable IAM Roles for Service Accounts, see the `aws.eks.Cluster` resource documentation.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetClusterIdentity>,
        >,
        /// Nested list containing Kubernetes Network Configuration.
        pub kubernetes_network_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetClusterKubernetesNetworkConfig>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Contains Outpost Configuration.
        pub outpost_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetClusterOutpostConfig>,
        >,
        /// Platform version for the cluster.
        pub platform_version: pulumi_wasm_rust::Output<String>,
        /// Contains remote network configuration for EKS Hybrid Nodes.
        pub remote_network_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetClusterRemoteNetworkConfig>,
        >,
        /// ARN of the IAM role that provides permissions for the Kubernetes control plane to make calls to AWS API operations on your behalf.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Status of the EKS cluster. One of `CREATING`, `ACTIVE`, `DELETING`, `FAILED`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Contains storage configuration for EKS Auto Mode enabled cluster.
        pub storage_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetClusterStorageConfig>,
        >,
        /// Key-value map of resource tags.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Configuration block for the support policy to use for the cluster.
        pub upgrade_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetClusterUpgradePolicy>,
        >,
        /// Kubernetes server version for the cluster.
        pub version: pulumi_wasm_rust::Output<String>,
        /// Nested list containing VPC configuration for the cluster.
        pub vpc_config: pulumi_wasm_rust::Output<
            super::super::super::types::eks::GetClusterVpcConfig,
        >,
        /// Contains Zonal Shift Configuration.
        pub zonal_shift_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eks::GetClusterZonalShiftConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetClusterArgs) -> GetClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:eks/getCluster:getCluster".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessConfigs".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "certificateAuthorities".into(),
                },
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "computeConfigs".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "enabledClusterLogTypes".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "kubernetesNetworkConfigs".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outpostConfigs".into(),
                },
                register_interface::ResultField {
                    name: "platformVersion".into(),
                },
                register_interface::ResultField {
                    name: "remoteNetworkConfigs".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "storageConfigs".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "upgradePolicies".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfig".into(),
                },
                register_interface::ResultField {
                    name: "zonalShiftConfigs".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterResult {
            access_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessConfigs").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate_authorities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateAuthorities").unwrap(),
            ),
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            compute_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeConfigs").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            enabled_cluster_log_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabledClusterLogTypes").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            kubernetes_network_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesNetworkConfigs").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outpost_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outpostConfigs").unwrap(),
            ),
            platform_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformVersion").unwrap(),
            ),
            remote_network_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteNetworkConfigs").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            storage_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageConfigs").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            upgrade_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upgradePolicies").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            vpc_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfig").unwrap(),
            ),
            zonal_shift_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zonalShiftConfigs").unwrap(),
            ),
        }
    }
}
