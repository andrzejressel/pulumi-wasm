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
pub mod v_mware_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VMwareClusterArgs {
        /// The admin cluster this VMware User Cluster belongs to.
        /// This is the full resource name of the admin cluster's hub membership.
        /// In the future, references to other resource types might be allowed if
        /// admin clusters are modeled as their own resources.
        #[builder(into)]
        pub admin_cluster_membership: pulumi_wasm_rust::Output<String>,
        /// Annotations on the VMware User Cluster. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// 'effective_annotations' for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// AAGConfig specifies whether to spread VMware User Cluster nodes across at least three physical hosts in the datacenter.
        #[builder(into, default)]
        pub anti_affinity_groups: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterAntiAffinityGroups>,
        >,
        /// RBAC policy that will be applied and managed by GKE On-Prem.
        #[builder(into, default)]
        pub authorization: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterAuthorization>,
        >,
        /// Configuration for auto repairing.
        #[builder(into, default)]
        pub auto_repair_config: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterAutoRepairConfig>,
        >,
        /// VMware User Cluster control plane nodes must have either 1 or 3 replicas.
        /// Structure is documented below.
        #[builder(into)]
        pub control_plane_node: pulumi_wasm_rust::Output<
            super::super::types::gkeonprem::VMwareClusterControlPlaneNode,
        >,
        /// VmwareDataplaneV2Config specifies configuration for Dataplane V2.
        #[builder(into, default)]
        pub dataplane_v2: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterDataplaneV2>,
        >,
        /// (Output)
        /// The description of the validation check.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Disable bundled ingress.
        #[builder(into, default)]
        pub disable_bundled_ingress: pulumi_wasm_rust::Output<Option<bool>>,
        /// Enable control plane V2. Default to false.
        #[builder(into, default)]
        pub enable_control_plane_v2: pulumi_wasm_rust::Output<Option<bool>>,
        /// Load Balancer configuration.
        #[builder(into, default)]
        pub load_balancer: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterLoadBalancer>,
        >,
        /// The location of the resource.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The VMware cluster name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The VMware User Cluster network configuration.
        #[builder(into, default)]
        pub network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterNetworkConfig>,
        >,
        /// The Anthos clusters on the VMware version for your user cluster.
        #[builder(into)]
        pub on_prem_version: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Storage configuration.
        #[builder(into, default)]
        pub storage: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterStorage>,
        >,
        /// Specifies upgrade policy for the cluster.
        #[builder(into, default)]
        pub upgrade_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterUpgradePolicy>,
        >,
        /// VmwareVCenterConfig specifies vCenter config for the user cluster. Inherited from the admin cluster.
        #[builder(into, default)]
        pub vcenter: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterVcenter>,
        >,
        /// Enable VM tracking.
        #[builder(into, default)]
        pub vm_tracking_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct VMwareClusterResult {
        /// The admin cluster this VMware User Cluster belongs to.
        /// This is the full resource name of the admin cluster's hub membership.
        /// In the future, references to other resource types might be allowed if
        /// admin clusters are modeled as their own resources.
        pub admin_cluster_membership: pulumi_wasm_rust::Output<String>,
        /// Annotations on the VMware User Cluster. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// 'effective_annotations' for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// AAGConfig specifies whether to spread VMware User Cluster nodes across at least three physical hosts in the datacenter.
        pub anti_affinity_groups: pulumi_wasm_rust::Output<
            super::super::types::gkeonprem::VMwareClusterAntiAffinityGroups,
        >,
        /// RBAC policy that will be applied and managed by GKE On-Prem.
        pub authorization: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterAuthorization>,
        >,
        /// Configuration for auto repairing.
        pub auto_repair_config: pulumi_wasm_rust::Output<
            super::super::types::gkeonprem::VMwareClusterAutoRepairConfig,
        >,
        /// VMware User Cluster control plane nodes must have either 1 or 3 replicas.
        /// Structure is documented below.
        pub control_plane_node: pulumi_wasm_rust::Output<
            super::super::types::gkeonprem::VMwareClusterControlPlaneNode,
        >,
        /// The time at which VMware User Cluster was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// VmwareDataplaneV2Config specifies configuration for Dataplane V2.
        pub dataplane_v2: pulumi_wasm_rust::Output<
            super::super::types::gkeonprem::VMwareClusterDataplaneV2,
        >,
        /// The time at which VMware User Cluster was deleted.
        pub delete_time: pulumi_wasm_rust::Output<String>,
        /// (Output)
        /// The description of the validation check.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Disable bundled ingress.
        pub disable_bundled_ingress: pulumi_wasm_rust::Output<Option<bool>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable control plane V2. Default to false.
        pub enable_control_plane_v2: pulumi_wasm_rust::Output<Option<bool>>,
        /// The DNS name of VMware User Cluster's API server.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// This checksum is computed by the server based on the value of other
        /// fields, and may be sent on update and delete requests to ensure the
        /// client has an up-to-date value before proceeding.
        /// Allows clients to perform consistent read-modify-writes
        /// through optimistic concurrency control.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Fleet configuration for the cluster.
        /// Structure is documented below.
        pub fleets: pulumi_wasm_rust::Output<
            Vec<super::super::types::gkeonprem::VMwareClusterFleet>,
        >,
        /// Load Balancer configuration.
        pub load_balancer: pulumi_wasm_rust::Output<
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
        pub local_name: pulumi_wasm_rust::Output<String>,
        /// The location of the resource.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The VMware cluster name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The VMware User Cluster network configuration.
        pub network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterNetworkConfig>,
        >,
        /// The Anthos clusters on the VMware version for your user cluster.
        pub on_prem_version: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// If set, there are currently changes in flight to the VMware User Cluster.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// (Output)
        /// The lifecycle state of the condition.
        pub state: pulumi_wasm_rust::Output<String>,
        /// (Output)
        /// Specifies the detailed validation check status
        /// Structure is documented below.
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::gkeonprem::VMwareClusterStatus>,
        >,
        /// Storage configuration.
        pub storage: pulumi_wasm_rust::Output<
            super::super::types::gkeonprem::VMwareClusterStorage,
        >,
        /// The unique identifier of the VMware User Cluster.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// The time at which VMware User Cluster was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Specifies upgrade policy for the cluster.
        pub upgrade_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterUpgradePolicy>,
        >,
        /// ValidationCheck represents the result of the preflight check job.
        /// Structure is documented below.
        pub validation_checks: pulumi_wasm_rust::Output<
            Vec<super::super::types::gkeonprem::VMwareClusterValidationCheck>,
        >,
        /// VmwareVCenterConfig specifies vCenter config for the user cluster. Inherited from the admin cluster.
        pub vcenter: pulumi_wasm_rust::Output<
            Option<super::super::types::gkeonprem::VMwareClusterVcenter>,
        >,
        /// Enable VM tracking.
        pub vm_tracking_enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VMwareClusterArgs) -> VMwareClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_cluster_membership_binding = args.admin_cluster_membership.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let anti_affinity_groups_binding = args.anti_affinity_groups.get_inner();
        let authorization_binding = args.authorization.get_inner();
        let auto_repair_config_binding = args.auto_repair_config.get_inner();
        let control_plane_node_binding = args.control_plane_node.get_inner();
        let dataplane_v2_binding = args.dataplane_v2.get_inner();
        let description_binding = args.description.get_inner();
        let disable_bundled_ingress_binding = args.disable_bundled_ingress.get_inner();
        let enable_control_plane_v2_binding = args.enable_control_plane_v2.get_inner();
        let load_balancer_binding = args.load_balancer.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_config_binding = args.network_config.get_inner();
        let on_prem_version_binding = args.on_prem_version.get_inner();
        let project_binding = args.project.get_inner();
        let storage_binding = args.storage.get_inner();
        let upgrade_policy_binding = args.upgrade_policy.get_inner();
        let vcenter_binding = args.vcenter.get_inner();
        let vm_tracking_enabled_binding = args.vm_tracking_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkeonprem/vMwareCluster:VMwareCluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminClusterMembership".into(),
                    value: &admin_cluster_membership_binding,
                },
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "antiAffinityGroups".into(),
                    value: &anti_affinity_groups_binding,
                },
                register_interface::ObjectField {
                    name: "authorization".into(),
                    value: &authorization_binding,
                },
                register_interface::ObjectField {
                    name: "autoRepairConfig".into(),
                    value: &auto_repair_config_binding,
                },
                register_interface::ObjectField {
                    name: "controlPlaneNode".into(),
                    value: &control_plane_node_binding,
                },
                register_interface::ObjectField {
                    name: "dataplaneV2".into(),
                    value: &dataplane_v2_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disableBundledIngress".into(),
                    value: &disable_bundled_ingress_binding,
                },
                register_interface::ObjectField {
                    name: "enableControlPlaneV2".into(),
                    value: &enable_control_plane_v2_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancer".into(),
                    value: &load_balancer_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfig".into(),
                    value: &network_config_binding,
                },
                register_interface::ObjectField {
                    name: "onPremVersion".into(),
                    value: &on_prem_version_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "storage".into(),
                    value: &storage_binding,
                },
                register_interface::ObjectField {
                    name: "upgradePolicy".into(),
                    value: &upgrade_policy_binding,
                },
                register_interface::ObjectField {
                    name: "vcenter".into(),
                    value: &vcenter_binding,
                },
                register_interface::ObjectField {
                    name: "vmTrackingEnabled".into(),
                    value: &vm_tracking_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adminClusterMembership".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "antiAffinityGroups".into(),
                },
                register_interface::ResultField {
                    name: "authorization".into(),
                },
                register_interface::ResultField {
                    name: "autoRepairConfig".into(),
                },
                register_interface::ResultField {
                    name: "controlPlaneNode".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dataplaneV2".into(),
                },
                register_interface::ResultField {
                    name: "deleteTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disableBundledIngress".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "enableControlPlaneV2".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "fleets".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancer".into(),
                },
                register_interface::ResultField {
                    name: "localName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkConfig".into(),
                },
                register_interface::ResultField {
                    name: "onPremVersion".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "statuses".into(),
                },
                register_interface::ResultField {
                    name: "storage".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "upgradePolicy".into(),
                },
                register_interface::ResultField {
                    name: "validationChecks".into(),
                },
                register_interface::ResultField {
                    name: "vcenter".into(),
                },
                register_interface::ResultField {
                    name: "vmTrackingEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VMwareClusterResult {
            admin_cluster_membership: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminClusterMembership").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            anti_affinity_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("antiAffinityGroups").unwrap(),
            ),
            authorization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorization").unwrap(),
            ),
            auto_repair_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoRepairConfig").unwrap(),
            ),
            control_plane_node: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlPlaneNode").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dataplane_v2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataplaneV2").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disable_bundled_ingress: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableBundledIngress").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            enable_control_plane_v2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableControlPlaneV2").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            fleets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleets").unwrap(),
            ),
            load_balancer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancer").unwrap(),
            ),
            local_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfig").unwrap(),
            ),
            on_prem_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onPremVersion").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statuses").unwrap(),
            ),
            storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storage").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            upgrade_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("upgradePolicy").unwrap(),
            ),
            validation_checks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationChecks").unwrap(),
            ),
            vcenter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vcenter").unwrap(),
            ),
            vm_tracking_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmTrackingEnabled").unwrap(),
            ),
        }
    }
}
