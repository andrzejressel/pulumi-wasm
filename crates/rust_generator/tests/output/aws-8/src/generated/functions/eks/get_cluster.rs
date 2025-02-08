#[allow(clippy::doc_lazy_continuation)]
pub mod get_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// Name of the cluster.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        /// Configuration block for access config.
        pub access_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetClusterAccessConfig>,
        >,
        /// ARN of the cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Nested attribute containing `certificate-authority-data` for your cluster.
        pub certificate_authorities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetClusterCertificateAuthority>,
        >,
        /// The ID of your local Amazon EKS cluster on the AWS Outpost. This attribute isn't available for an AWS EKS cluster on AWS cloud.
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Nested attribute containing compute capability configuration for EKS Auto Mode enabled cluster.
        pub compute_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetClusterComputeConfig>,
        >,
        /// Unix epoch time stamp in seconds for when the cluster was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The enabled control plane logs.
        pub enabled_cluster_log_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Endpoint for your Kubernetes API server.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Nested attribute containing identity provider information for your cluster. Only available on Kubernetes version 1.13 and 1.14 clusters created or upgraded on or after September 3, 2019. For an example using this information to enable IAM Roles for Service Accounts, see the `aws.eks.Cluster` resource documentation.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetClusterIdentity>,
        >,
        /// Nested list containing Kubernetes Network Configuration.
        pub kubernetes_network_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetClusterKubernetesNetworkConfig>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Contains Outpost Configuration.
        pub outpost_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetClusterOutpostConfig>,
        >,
        /// Platform version for the cluster.
        pub platform_version: pulumi_gestalt_rust::Output<String>,
        /// Contains remote network configuration for EKS Hybrid Nodes.
        pub remote_network_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetClusterRemoteNetworkConfig>,
        >,
        /// ARN of the IAM role that provides permissions for the Kubernetes control plane to make calls to AWS API operations on your behalf.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Status of the EKS cluster. One of `CREATING`, `ACTIVE`, `DELETING`, `FAILED`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Contains storage configuration for EKS Auto Mode enabled cluster.
        pub storage_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetClusterStorageConfig>,
        >,
        /// Key-value map of resource tags.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Configuration block for the support policy to use for the cluster.
        pub upgrade_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetClusterUpgradePolicy>,
        >,
        /// Kubernetes server version for the cluster.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// Nested list containing VPC configuration for the cluster.
        pub vpc_config: pulumi_gestalt_rust::Output<
            super::super::super::types::eks::GetClusterVpcConfig,
        >,
        /// Contains Zonal Shift Configuration.
        pub zonal_shift_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetClusterZonalShiftConfig>,
        >,
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
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:eks/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterResult {
            access_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessConfigs"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            certificate_authorities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateAuthorities"),
            ),
            cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterId"),
            ),
            compute_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("computeConfigs"),
            ),
            created_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            enabled_cluster_log_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabledClusterLogTypes"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            kubernetes_network_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kubernetesNetworkConfigs"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            outpost_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outpostConfigs"),
            ),
            platform_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformVersion"),
            ),
            remote_network_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("remoteNetworkConfigs"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            storage_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageConfigs"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            upgrade_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("upgradePolicies"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            vpc_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcConfig"),
            ),
            zonal_shift_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zonalShiftConfigs"),
            ),
        }
    }
}
