/// ## Example Usage
///
/// ### Workstation Cluster Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:workstations:WorkstationCluster
///     properties:
///       workstationClusterId: workstation-cluster
///       network: ${defaultNetwork.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: workstation-cluster
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${defaultNetwork.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Workstation Cluster Private
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:workstations:WorkstationCluster
///     properties:
///       workstationClusterId: workstation-cluster-private
///       network: ${defaultNetwork.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       privateClusterConfig:
///         enablePrivateEndpoint: true
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: workstation-cluster-private
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster-private
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${defaultNetwork.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Workstation Cluster Custom Domain
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:workstations:WorkstationCluster
///     properties:
///       workstationClusterId: workstation-cluster-custom-domain
///       network: ${defaultNetwork.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       privateClusterConfig:
///         enablePrivateEndpoint: true
///       domainConfig:
///         domain: workstations.example.com
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: workstation-cluster-custom-domain
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster-custom-domain
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${defaultNetwork.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// WorkstationCluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}`
///
/// * `{{project}}/{{location}}/{{workstation_cluster_id}}`
///
/// * `{{location}}/{{workstation_cluster_id}}`
///
/// When using the `pulumi import` command, WorkstationCluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:workstations/workstationCluster:WorkstationCluster default projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:workstations/workstationCluster:WorkstationCluster default {{project}}/{{location}}/{{workstation_cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:workstations/workstationCluster:WorkstationCluster default {{location}}/{{workstation_cluster_id}}
/// ```
///
pub mod workstation_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkstationClusterArgs {
        /// Client-specified annotations. This is distinct from labels.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Human-readable name for this resource.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration options for a custom domain.
        /// Structure is documented below.
        #[builder(into, default)]
        pub domain_config: pulumi_wasm_rust::Output<
            Option<super::super::types::workstations::WorkstationClusterDomainConfig>,
        >,
        /// Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the workstation cluster should reside.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative resource name of the VPC network on which the instance can be accessed.
        /// It is specified in the following form: "projects/{projectNumber}/global/networks/{network_id}".
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
        /// Configuration for private cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_cluster_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::workstations::WorkstationClusterPrivateClusterConfig,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Compute Engine subnetwork in which instances associated with this cluster will be created.
        /// Must be part of the subnetwork specified for this cluster.
        #[builder(into)]
        pub subnetwork: pulumi_wasm_rust::Output<String>,
        /// ID to use for the workstation cluster.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub workstation_cluster_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkstationClusterResult {
        /// Client-specified annotations. This is distinct from labels.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Status conditions describing the current resource state.
        /// Structure is documented below.
        pub conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::workstations::WorkstationClusterCondition>,
        >,
        /// The private IP address of the control plane for this workstation cluster.
        /// Workstation VMs need access to this IP address to work with the service, so make sure that your firewall rules allow egress from the workstation VMs to this address.
        pub control_plane_ip: pulumi_wasm_rust::Output<String>,
        /// Time when this resource was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Whether this resource is in degraded mode, in which case it may require user action to restore full functionality.
        /// Details can be found in the conditions field.
        pub degraded: pulumi_wasm_rust::Output<bool>,
        /// Human-readable name for this resource.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration options for a custom domain.
        /// Structure is documented below.
        pub domain_config: pulumi_wasm_rust::Output<
            Option<super::super::types::workstations::WorkstationClusterDomainConfig>,
        >,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Checksum computed by the server.
        /// May be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the workstation cluster should reside.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the cluster resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The relative resource name of the VPC network on which the instance can be accessed.
        /// It is specified in the following form: "projects/{projectNumber}/global/networks/{network_id}".
        pub network: pulumi_wasm_rust::Output<String>,
        /// Configuration for private cluster.
        /// Structure is documented below.
        pub private_cluster_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::workstations::WorkstationClusterPrivateClusterConfig,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name of the Compute Engine subnetwork in which instances associated with this cluster will be created.
        /// Must be part of the subnetwork specified for this cluster.
        pub subnetwork: pulumi_wasm_rust::Output<String>,
        /// The system-generated UID of the resource.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// ID to use for the workstation cluster.
        ///
        ///
        /// - - -
        pub workstation_cluster_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkstationClusterArgs) -> WorkstationClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let domain_config_binding = args.domain_config.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let network_binding = args.network.get_inner();
        let private_cluster_config_binding = args.private_cluster_config.get_inner();
        let project_binding = args.project.get_inner();
        let subnetwork_binding = args.subnetwork.get_inner();
        let workstation_cluster_id_binding = args.workstation_cluster_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:workstations/workstationCluster:WorkstationCluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "domainConfig".into(),
                    value: &domain_config_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "privateClusterConfig".into(),
                    value: &private_cluster_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding,
                },
                register_interface::ObjectField {
                    name: "workstationClusterId".into(),
                    value: &workstation_cluster_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "conditions".into(),
                },
                register_interface::ResultField {
                    name: "controlPlaneIp".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "degraded".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "domainConfig".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "privateClusterConfig".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "subnetwork".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "workstationClusterId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkstationClusterResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditions").unwrap(),
            ),
            control_plane_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlPlaneIp").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            degraded: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("degraded").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            domain_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainConfig").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            private_cluster_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateClusterConfig").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetwork").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            workstation_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workstationClusterId").unwrap(),
            ),
        }
    }
}
