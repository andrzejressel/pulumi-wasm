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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workstation_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkstationClusterArgs {
        /// Client-specified annotations. This is distinct from labels.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Human-readable name for this resource.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration options for a custom domain.
        /// Structure is documented below.
        #[builder(into, default)]
        pub domain_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::workstations::WorkstationClusterDomainConfig>,
        >,
        /// Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the workstation cluster should reside.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The relative resource name of the VPC network on which the instance can be accessed.
        /// It is specified in the following form: "projects/{projectNumber}/global/networks/{network_id}".
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for private cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_cluster_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::workstations::WorkstationClusterPrivateClusterConfig,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the Compute Engine subnetwork in which instances associated with this cluster will be created.
        /// Must be part of the subnetwork specified for this cluster.
        #[builder(into)]
        pub subnetwork: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID to use for the workstation cluster.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub workstation_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkstationClusterResult {
        /// Client-specified annotations. This is distinct from labels.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Status conditions describing the current resource state.
        /// Structure is documented below.
        pub conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::workstations::WorkstationClusterCondition>,
        >,
        /// The private IP address of the control plane for this workstation cluster.
        /// Workstation VMs need access to this IP address to work with the service, so make sure that your firewall rules allow egress from the workstation VMs to this address.
        pub control_plane_ip: pulumi_gestalt_rust::Output<String>,
        /// Time when this resource was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Whether this resource is in degraded mode, in which case it may require user action to restore full functionality.
        /// Details can be found in the conditions field.
        pub degraded: pulumi_gestalt_rust::Output<bool>,
        /// Human-readable name for this resource.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration options for a custom domain.
        /// Structure is documented below.
        pub domain_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::workstations::WorkstationClusterDomainConfig>,
        >,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Checksum computed by the server.
        /// May be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the workstation cluster should reside.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the cluster resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The relative resource name of the VPC network on which the instance can be accessed.
        /// It is specified in the following form: "projects/{projectNumber}/global/networks/{network_id}".
        pub network: pulumi_gestalt_rust::Output<String>,
        /// Configuration for private cluster.
        /// Structure is documented below.
        pub private_cluster_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::workstations::WorkstationClusterPrivateClusterConfig,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name of the Compute Engine subnetwork in which instances associated with this cluster will be created.
        /// Must be part of the subnetwork specified for this cluster.
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
        /// The system-generated UID of the resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// ID to use for the workstation cluster.
        ///
        ///
        /// - - -
        pub workstation_cluster_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkstationClusterArgs,
    ) -> WorkstationClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let domain_config_binding = args.domain_config.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let network_binding = args.network.get_output(context);
        let private_cluster_config_binding = args
            .private_cluster_config
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let subnetwork_binding = args.subnetwork.get_output(context);
        let workstation_cluster_id_binding = args
            .workstation_cluster_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:workstations/workstationCluster:WorkstationCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainConfig".into(),
                    value: domain_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateClusterConfig".into(),
                    value: private_cluster_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetwork".into(),
                    value: subnetwork_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workstationClusterId".into(),
                    value: workstation_cluster_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkstationClusterResult {
            annotations: o.get_field("annotations"),
            conditions: o.get_field("conditions"),
            control_plane_ip: o.get_field("controlPlaneIp"),
            create_time: o.get_field("createTime"),
            degraded: o.get_field("degraded"),
            display_name: o.get_field("displayName"),
            domain_config: o.get_field("domainConfig"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            private_cluster_config: o.get_field("privateClusterConfig"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            subnetwork: o.get_field("subnetwork"),
            uid: o.get_field("uid"),
            workstation_cluster_id: o.get_field("workstationClusterId"),
        }
    }
}
