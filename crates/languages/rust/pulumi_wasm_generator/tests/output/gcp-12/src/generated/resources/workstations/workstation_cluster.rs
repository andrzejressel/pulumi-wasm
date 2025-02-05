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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkstationClusterArgs {
        /// Client-specified annotations. This is distinct from labels.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Human-readable name for this resource.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration options for a custom domain.
        /// Structure is documented below.
        #[builder(into, default)]
        pub domain_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::workstations::WorkstationClusterDomainConfig>,
        >,
        /// Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the workstation cluster should reside.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The relative resource name of the VPC network on which the instance can be accessed.
        /// It is specified in the following form: "projects/{projectNumber}/global/networks/{network_id}".
        #[builder(into)]
        pub network: pulumi_wasm_rust::InputOrOutput<String>,
        /// Configuration for private cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_cluster_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::workstations::WorkstationClusterPrivateClusterConfig,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the Compute Engine subnetwork in which instances associated with this cluster will be created.
        /// Must be part of the subnetwork specified for this cluster.
        #[builder(into)]
        pub subnetwork: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID to use for the workstation cluster.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub workstation_cluster_id: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WorkstationClusterArgs,
    ) -> WorkstationClusterResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let domain_config_binding = args.domain_config.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let private_cluster_config_binding = args
            .private_cluster_config
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let subnetwork_binding = args.subnetwork.get_output(context).get_inner();
        let workstation_cluster_id_binding = args
            .workstation_cluster_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:workstations/workstationCluster:WorkstationCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkstationClusterResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("conditions"),
            ),
            control_plane_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("controlPlaneIp"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            degraded: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("degraded"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            domain_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainConfig"),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            private_cluster_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateClusterConfig"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
            workstation_cluster_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workstationClusterId"),
            ),
        }
    }
}
