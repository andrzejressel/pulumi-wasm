/// ## Example Usage
///
/// ### Workstation Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
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
///       network: ${default.name}
///   defaultWorkstationCluster:
///     type: gcp:workstations:WorkstationCluster
///     name: default
///     properties:
///       workstationClusterId: workstation-cluster
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultWorkstationConfig:
///     type: gcp:workstations:WorkstationConfig
///     name: default
///     properties:
///       workstationConfigId: workstation-config
///       workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
///       location: us-central1
///       host:
///         gceInstance:
///           machineType: e2-standard-4
///           bootDiskSizeGb: 35
///           disablePublicIpAddresses: true
///   defaultWorkstation:
///     type: gcp:workstations:Workstation
///     name: default
///     properties:
///       workstationId: work-station
///       workstationConfigId: ${defaultWorkstationConfig.workstationConfigId}
///       workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
///       location: us-central1
///       labels:
///         label: key
///       env:
///         name: foo
///       annotations:
///         label-one: value-one
/// ```
///
/// ## Import
///
/// Workstation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}}`
///
/// * `{{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}`
///
/// * `{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}`
///
/// When using the `pulumi import` command, Workstation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:workstations/workstation:Workstation default projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}/workstations/{{workstation_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:workstations/workstation:Workstation default {{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:workstations/workstation:Workstation default {{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}/{{workstation_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workstation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkstationArgs {
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
        /// 'Client-specified environment variables passed to the workstation container's entrypoint.'
        #[builder(into, default)]
        pub env: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the workstation parent resources reside.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the parent workstation cluster.
        #[builder(into)]
        pub workstation_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the parent workstation cluster config.
        #[builder(into)]
        pub workstation_config_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID to use for the workstation.
        #[builder(into)]
        pub workstation_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkstationResult {
        /// Client-specified annotations. This is distinct from labels.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Time when this resource was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Human-readable name for this resource.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// 'Client-specified environment variables passed to the workstation container's entrypoint.'
        pub env: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Host to which clients can send HTTPS traffic that will be received by the workstation.
        /// Authorized traffic will be received to the workstation as HTTP on port 80.
        /// To send traffic to a different port, clients may prefix the host with the destination port in the format "{port}-{host}".
        pub host: pulumi_gestalt_rust::Output<String>,
        /// Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the workstation parent resources reside.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Full name of this resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Current state of the workstation.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A system-assigned unique identified for this resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The ID of the parent workstation cluster.
        pub workstation_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the parent workstation cluster config.
        pub workstation_config_id: pulumi_gestalt_rust::Output<String>,
        /// ID to use for the workstation.
        pub workstation_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkstationArgs,
    ) -> WorkstationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let env_binding = args.env.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let workstation_cluster_id_binding = args
            .workstation_cluster_id
            .get_output(context);
        let workstation_config_id_binding = args
            .workstation_config_id
            .get_output(context);
        let workstation_id_binding = args.workstation_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:workstations/workstation:Workstation".into(),
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
                    name: "env".into(),
                    value: env_binding.get_id(),
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
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workstationClusterId".into(),
                    value: workstation_cluster_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workstationConfigId".into(),
                    value: workstation_config_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workstationId".into(),
                    value: workstation_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkstationResult {
            annotations: o.get_field("annotations"),
            create_time: o.get_field("createTime"),
            display_name: o.get_field("displayName"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            env: o.get_field("env"),
            host: o.get_field("host"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            state: o.get_field("state"),
            uid: o.get_field("uid"),
            workstation_cluster_id: o.get_field("workstationClusterId"),
            workstation_config_id: o.get_field("workstationConfigId"),
            workstation_id: o.get_field("workstationId"),
        }
    }
}
