/// A Google VMware User Cluster.
///
///
///
/// ## Example Usage
///
/// ### Gkeonprem Vmware Cluster Basic
///
///
/// ```yaml
/// resources:
///   cluster-basic:
///     type: gcp:gkeonprem:VMwareCluster
///     properties:
///       name: cluster-basic
///       location: us-west1
///       adminClusterMembership: projects/870316890899/locations/global/memberships/gkeonprem-terraform-test
///       description: test cluster
///       onPremVersion: 1.13.1-gke.35
///       annotations: {}
///       networkConfig:
///         serviceAddressCidrBlocks:
///           - 10.96.0.0/12
///         podAddressCidrBlocks:
///           - 192.168.0.0/16
///         dhcpIpConfig:
///           enabled: true
///       controlPlaneNode:
///         cpus: 4
///         memory: 8192
///         replicas: 1
///       loadBalancer:
///         vipConfig:
///           controlPlaneVip: 10.251.133.5
///           ingressVip: 10.251.135.19
///         metalLbConfig:
///           addressPools:
///             - pool: ingress-ip
///               manualAssign: 'true'
///               addresses:
///                 - 10.251.135.19
///               avoidBuggyIps: true
///             - pool: lb-test-ip
///               manualAssign: 'true'
///               addresses:
///                 - 10.251.135.19
///               avoidBuggyIps: true
/// ```
/// ### Gkeonprem Vmware Cluster F5lb
///
///
/// ```yaml
/// resources:
///   cluster-f5lb:
///     type: gcp:gkeonprem:VMwareCluster
///     properties:
///       name: cluster-f5lb
///       location: us-west1
///       adminClusterMembership: projects/870316890899/locations/global/memberships/gkeonprem-terraform-test
///       description: test cluster
///       onPremVersion: 1.13.1-gke.35
///       annotations: {}
///       networkConfig:
///         serviceAddressCidrBlocks:
///           - 10.96.0.0/12
///         podAddressCidrBlocks:
///           - 192.168.0.0/16
///         dhcpIpConfig:
///           enabled: true
///         controlPlaneV2Config:
///           controlPlaneIpBlock:
///             ips:
///               - hostname: test-hostname
///                 ip: 10.0.0.1
///             netmask: 10.0.0.1/32
///             gateway: test-gateway
///         vcenterNetwork: test-vcenter-network
///       controlPlaneNode:
///         cpus: 4
///         memory: 8192
///         replicas: 1
///         autoResizeConfig:
///           enabled: true
///       loadBalancer:
///         vipConfig:
///           controlPlaneVip: 10.251.133.5
///           ingressVip: 10.251.135.19
///         f5Config:
///           address: 10.0.0.1
///           partition: test-partition
///           snatPool: test-snap-pool
///       dataplaneV2:
///         dataplaneV2Enabled: true
///         windowsDataplaneV2Enabled: true
///         advancedNetworking: true
///       vmTrackingEnabled: true
///       enableControlPlaneV2: true
///       disableBundledIngress: true
///       authorization:
///         adminUsers:
///           - username: testuser@gmail.com
///       antiAffinityGroups:
///         aagConfigDisabled: true
///       autoRepairConfig:
///         enabled: true
///       storage:
///         vsphereCsiDisabled: true
/// ```
/// ### Gkeonprem Vmware Cluster Manuallb
///
///
/// ```yaml
/// resources:
///   cluster-manuallb:
///     type: gcp:gkeonprem:VMwareCluster
///     properties:
///       name: cluster-manuallb
///       location: us-west1
///       adminClusterMembership: projects/870316890899/locations/global/memberships/gkeonprem-terraform-test
///       description: test cluster
///       onPremVersion: 1.13.1-gke.35
///       annotations: {}
///       networkConfig:
///         serviceAddressCidrBlocks:
///           - 10.96.0.0/12
///         podAddressCidrBlocks:
///           - 192.168.0.0/16
///         hostConfig:
///           dnsServers:
///             - 10.254.41.1
///           ntpServers:
///             - 216.239.35.8
///           dnsSearchDomains:
///             - test-domain
///         staticIpConfig:
///           ipBlocks:
///             - netmask: 255.255.252.0
///               gateway: 10.251.31.254
///               ips:
///                 - ip: 10.251.30.153
///                   hostname: test-hostname1
///                 - ip: 10.251.31.206
///                   hostname: test-hostname2
///                 - ip: 10.251.31.193
///                   hostname: test-hostname3
///                 - ip: 10.251.30.230
///                   hostname: test-hostname4
///       controlPlaneNode:
///         cpus: 4
///         memory: 8192
///         replicas: 1
///         autoResizeConfig:
///           enabled: true
///       loadBalancer:
///         vipConfig:
///           controlPlaneVip: 10.251.133.5
///           ingressVip: 10.251.135.19
///         manualLbConfig:
///           ingressHttpNodePort: 30005
///           ingressHttpsNodePort: 30006
///           controlPlaneNodePort: 30007
///           konnectivityServerNodePort: 30008
///       vcenter:
///         resourcePool: test-resource-pool
///         datastore: test-datastore
///         datacenter: test-datacenter
///         cluster: test-cluster
///         folder: test-folder
///         caCertData: test-ca-cert-data
///         storagePolicyName: test-storage-policy-name
///       dataplaneV2:
///         dataplaneV2Enabled: true
///         windowsDataplaneV2Enabled: true
///         advancedNetworking: true
///       vmTrackingEnabled: true
///       enableControlPlaneV2: true
///       upgradePolicy:
///         controlPlaneOnly: true
///       authorization:
///         adminUsers:
///           - username: testuser@gmail.com
///       antiAffinityGroups:
///         aagConfigDisabled: true
///       autoRepairConfig:
///         enabled: true
/// ```
///
/// ## Import
///
/// VmwareCluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/vmwareClusters/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, VmwareCluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/vMwareCluster:VMwareCluster default projects/{{project}}/locations/{{location}}/vmwareClusters/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/vMwareCluster:VMwareCluster default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkeonprem/vMwareCluster:VMwareCluster default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_mware_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VMwareClusterArgs {
        /// The admin cluster this VMware User Cluster belongs to.
        /// This is the full resource name of the admin cluster's hub membership.
        /// In the future, references to other resource types might be allowed if
        /// admin clusters are modeled as their own resources.
        #[builder(into)]
        pub admin_cluster_membership: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Annotations on the VMware User Cluster. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// 'effective_annotations' for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// AAGConfig specifies whether to spread VMware User Cluster nodes across at least three physical hosts in the datacenter.
        #[builder(into, default)]
        pub anti_affinity_groups: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::VMwareClusterAntiAffinityGroups>,
        >,
        /// RBAC policy that will be applied and managed by GKE On-Prem.
        #[builder(into, default)]
        pub authorization: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::VMwareClusterAuthorization>,
        >,
        /// Configuration for auto repairing.
        #[builder(into, default)]
        pub auto_repair_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::VMwareClusterAutoRepairConfig>,
        >,
        /// VMware User Cluster control plane nodes must have either 1 or 3 replicas.
        /// Structure is documented below.
        #[builder(into)]
        pub control_plane_node: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gkeonprem::VMwareClusterControlPlaneNode,
        >,
        /// VmwareDataplaneV2Config specifies configuration for Dataplane V2.
        #[builder(into, default)]
        pub dataplane_v2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::VMwareClusterDataplaneV2>,
        >,
        /// (Output)
        /// The description of the validation check.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Disable bundled ingress.
        #[builder(into, default)]
        pub disable_bundled_ingress: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enable control plane V2. Default to false.
        #[builder(into, default)]
        pub enable_control_plane_v2: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Load Balancer configuration.
        #[builder(into, default)]
        pub load_balancer: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::VMwareClusterLoadBalancer>,
        >,
        /// The location of the resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The VMware cluster name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The VMware User Cluster network configuration.
        #[builder(into, default)]
        pub network_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::VMwareClusterNetworkConfig>,
        >,
        /// The Anthos clusters on the VMware version for your user cluster.
        #[builder(into)]
        pub on_prem_version: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Storage configuration.
        #[builder(into, default)]
        pub storage: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::VMwareClusterStorage>,
        >,
        /// Specifies upgrade policy for the cluster.
        #[builder(into, default)]
        pub upgrade_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::VMwareClusterUpgradePolicy>,
        >,
        /// VmwareVCenterConfig specifies vCenter config for the user cluster. Inherited from the admin cluster.
        #[builder(into, default)]
        pub vcenter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkeonprem::VMwareClusterVcenter>,
        >,
        /// Enable VM tracking.
        #[builder(into, default)]
        pub vm_tracking_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct VMwareClusterResult {
        /// The admin cluster this VMware User Cluster belongs to.
        /// This is the full resource name of the admin cluster's hub membership.
        /// In the future, references to other resource types might be allowed if
        /// admin clusters are modeled as their own resources.
        pub admin_cluster_membership: pulumi_gestalt_rust::Output<String>,
        /// Annotations on the VMware User Cluster. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// 'effective_annotations' for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// AAGConfig specifies whether to spread VMware User Cluster nodes across at least three physical hosts in the datacenter.
        pub anti_affinity_groups: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::VMwareClusterAntiAffinityGroups,
        >,
        /// RBAC policy that will be applied and managed by GKE On-Prem.
        pub authorization: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterAuthorization>,
        >,
        /// Configuration for auto repairing.
        pub auto_repair_config: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::VMwareClusterAutoRepairConfig,
        >,
        /// VMware User Cluster control plane nodes must have either 1 or 3 replicas.
        /// Structure is documented below.
        pub control_plane_node: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::VMwareClusterControlPlaneNode,
        >,
        /// The time at which VMware User Cluster was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// VmwareDataplaneV2Config specifies configuration for Dataplane V2.
        pub dataplane_v2: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::VMwareClusterDataplaneV2,
        >,
        /// The time at which VMware User Cluster was deleted.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// The description of the validation check.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Disable bundled ingress.
        pub disable_bundled_ingress: pulumi_gestalt_rust::Output<Option<bool>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable control plane V2. Default to false.
        pub enable_control_plane_v2: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The DNS name of VMware User Cluster's API server.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// This checksum is computed by the server based on the value of other
        /// fields, and may be sent on update and delete requests to ensure the
        /// client has an up-to-date value before proceeding.
        /// Allows clients to perform consistent read-modify-writes
        /// through optimistic concurrency control.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Fleet configuration for the cluster.
        /// Structure is documented below.
        pub fleets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkeonprem::VMwareClusterFleet>,
        >,
        /// Load Balancer configuration.
        pub load_balancer: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterLoadBalancer>,
        >,
        /// The object name of the VMware OnPremUserCluster custom resource on the
        /// associated admin cluster. This field is used to support conflicting
        /// names when enrolling existing clusters to the API. When used as a part of
        /// cluster enrollment, this field will differ from the ID in the resource
        /// name. For new clusters, this field will match the user provided cluster ID
        /// and be visible in the last component of the resource name. It is not
        /// modifiable.
        /// All users should use this name to access their cluster using gkectl or
        /// kubectl and should expect to see the local name when viewing admin
        /// cluster controller logs.
        pub local_name: pulumi_gestalt_rust::Output<String>,
        /// The location of the resource.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The VMware cluster name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The VMware User Cluster network configuration.
        pub network_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterNetworkConfig>,
        >,
        /// The Anthos clusters on the VMware version for your user cluster.
        pub on_prem_version: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// If set, there are currently changes in flight to the VMware User Cluster.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// (Output)
        /// The lifecycle state of the condition.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// Specifies the detailed validation check status
        /// Structure is documented below.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkeonprem::VMwareClusterStatus>,
        >,
        /// Storage configuration.
        pub storage: pulumi_gestalt_rust::Output<
            super::super::types::gkeonprem::VMwareClusterStorage,
        >,
        /// The unique identifier of the VMware User Cluster.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The time at which VMware User Cluster was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Specifies upgrade policy for the cluster.
        pub upgrade_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterUpgradePolicy>,
        >,
        /// ValidationCheck represents the result of the preflight check job.
        /// Structure is documented below.
        pub validation_checks: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkeonprem::VMwareClusterValidationCheck>,
        >,
        /// VmwareVCenterConfig specifies vCenter config for the user cluster. Inherited from the admin cluster.
        pub vcenter: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterVcenter>,
        >,
        /// Enable VM tracking.
        pub vm_tracking_enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VMwareClusterArgs,
    ) -> VMwareClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_cluster_membership_binding = args
            .admin_cluster_membership
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let anti_affinity_groups_binding = args.anti_affinity_groups.get_output(context);
        let authorization_binding = args.authorization.get_output(context);
        let auto_repair_config_binding = args.auto_repair_config.get_output(context);
        let control_plane_node_binding = args.control_plane_node.get_output(context);
        let dataplane_v2_binding = args.dataplane_v2.get_output(context);
        let description_binding = args.description.get_output(context);
        let disable_bundled_ingress_binding = args
            .disable_bundled_ingress
            .get_output(context);
        let enable_control_plane_v2_binding = args
            .enable_control_plane_v2
            .get_output(context);
        let load_balancer_binding = args.load_balancer.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_config_binding = args.network_config.get_output(context);
        let on_prem_version_binding = args.on_prem_version.get_output(context);
        let project_binding = args.project.get_output(context);
        let storage_binding = args.storage.get_output(context);
        let upgrade_policy_binding = args.upgrade_policy.get_output(context);
        let vcenter_binding = args.vcenter.get_output(context);
        let vm_tracking_enabled_binding = args.vm_tracking_enabled.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gkeonprem/vMwareCluster:VMwareCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminClusterMembership".into(),
                    value: &admin_cluster_membership_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "antiAffinityGroups".into(),
                    value: &anti_affinity_groups_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorization".into(),
                    value: &authorization_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoRepairConfig".into(),
                    value: &auto_repair_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlPlaneNode".into(),
                    value: &control_plane_node_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataplaneV2".into(),
                    value: &dataplane_v2_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableBundledIngress".into(),
                    value: &disable_bundled_ingress_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableControlPlaneV2".into(),
                    value: &enable_control_plane_v2_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancer".into(),
                    value: &load_balancer_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkConfig".into(),
                    value: &network_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onPremVersion".into(),
                    value: &on_prem_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storage".into(),
                    value: &storage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "upgradePolicy".into(),
                    value: &upgrade_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vcenter".into(),
                    value: &vcenter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmTrackingEnabled".into(),
                    value: &vm_tracking_enabled_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VMwareClusterResult {
            admin_cluster_membership: o.get_field("adminClusterMembership"),
            annotations: o.get_field("annotations"),
            anti_affinity_groups: o.get_field("antiAffinityGroups"),
            authorization: o.get_field("authorization"),
            auto_repair_config: o.get_field("autoRepairConfig"),
            control_plane_node: o.get_field("controlPlaneNode"),
            create_time: o.get_field("createTime"),
            dataplane_v2: o.get_field("dataplaneV2"),
            delete_time: o.get_field("deleteTime"),
            description: o.get_field("description"),
            disable_bundled_ingress: o.get_field("disableBundledIngress"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            enable_control_plane_v2: o.get_field("enableControlPlaneV2"),
            endpoint: o.get_field("endpoint"),
            etag: o.get_field("etag"),
            fleets: o.get_field("fleets"),
            load_balancer: o.get_field("loadBalancer"),
            local_name: o.get_field("localName"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_config: o.get_field("networkConfig"),
            on_prem_version: o.get_field("onPremVersion"),
            project: o.get_field("project"),
            reconciling: o.get_field("reconciling"),
            state: o.get_field("state"),
            statuses: o.get_field("statuses"),
            storage: o.get_field("storage"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            upgrade_policy: o.get_field("upgradePolicy"),
            validation_checks: o.get_field("validationChecks"),
            vcenter: o.get_field("vcenter"),
            vm_tracking_enabled: o.get_field("vmTrackingEnabled"),
        }
    }
}
