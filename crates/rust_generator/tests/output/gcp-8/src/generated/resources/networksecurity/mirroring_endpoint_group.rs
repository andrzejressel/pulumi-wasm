/// ## Example Usage
///
/// ### Network Security Mirroring Endpoint Group Basic
///
///
/// ```yaml
/// resources:
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: example-network
///       autoCreateSubnetworks: false
///   deploymentGroup:
///     type: gcp:networksecurity:MirroringDeploymentGroup
///     name: deployment_group
///     properties:
///       mirroringDeploymentGroupId: example-dg
///       location: global
///       network: ${network.id}
///   default:
///     type: gcp:networksecurity:MirroringEndpointGroup
///     properties:
///       mirroringEndpointGroupId: example-eg
///       location: global
///       mirroringDeploymentGroup: ${deploymentGroup.id}
///       labels:
///         foo: bar
/// ```
///
/// ## Import
///
/// MirroringEndpointGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/mirroringEndpointGroups/{{mirroring_endpoint_group_id}}`
///
/// * `{{project}}/{{location}}/{{mirroring_endpoint_group_id}}`
///
/// * `{{location}}/{{mirroring_endpoint_group_id}}`
///
/// When using the `pulumi import` command, MirroringEndpointGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/mirroringEndpointGroup:MirroringEndpointGroup default projects/{{project}}/locations/{{location}}/mirroringEndpointGroups/{{mirroring_endpoint_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/mirroringEndpointGroup:MirroringEndpointGroup default {{project}}/{{location}}/{{mirroring_endpoint_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/mirroringEndpointGroup:MirroringEndpointGroup default {{location}}/{{mirroring_endpoint_group_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mirroring_endpoint_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MirroringEndpointGroupArgs {
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `networksecurity.googleapis.com/MirroringEndpointGroup`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. Immutable. The Mirroring Deployment Group that this resource is connected to. Format
        /// is:
        /// `projects/{project}/locations/global/mirroringDeploymentGroups/{mirroringDeploymentGroup}`
        #[builder(into)]
        pub mirroring_deployment_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. Id of the requesting object
        /// If auto-generating Id server-side, remove this field and
        /// mirroring_endpoint_group_id from the method_signature of Create RPC
        ///
        ///
        /// - - -
        #[builder(into)]
        pub mirroring_endpoint_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MirroringEndpointGroupResult {
        /// Output only. [Output only] Create time stamp
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `networksecurity.googleapis.com/MirroringEndpointGroup`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Required. Immutable. The Mirroring Deployment Group that this resource is connected to. Format
        /// is:
        /// `projects/{project}/locations/global/mirroringDeploymentGroups/{mirroringDeploymentGroup}`
        pub mirroring_deployment_group: pulumi_gestalt_rust::Output<String>,
        /// Required. Id of the requesting object
        /// If auto-generating Id server-side, remove this field and
        /// mirroring_endpoint_group_id from the method_signature of Create RPC
        ///
        ///
        /// - - -
        pub mirroring_endpoint_group_id: pulumi_gestalt_rust::Output<String>,
        /// Immutable. Identifier. The name of the MirroringEndpointGroup.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Whether reconciling is in progress, recommended per
        /// https://google.aip.dev/128.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Output only. Current state of the endpoint group.
        /// Possible values:
        /// STATE_UNSPECIFIED
        /// ACTIVE
        /// CLOSED
        /// CREATING
        /// DELETING
        /// OUT_OF_SYNC
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. [Output only] Update time stamp
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MirroringEndpointGroupArgs,
    ) -> MirroringEndpointGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let mirroring_deployment_group_binding = args
            .mirroring_deployment_group
            .get_output(context);
        let mirroring_endpoint_group_id_binding = args
            .mirroring_endpoint_group_id
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/mirroringEndpointGroup:MirroringEndpointGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mirroringDeploymentGroup".into(),
                    value: mirroring_deployment_group_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mirroringEndpointGroupId".into(),
                    value: mirroring_endpoint_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MirroringEndpointGroupResult {
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            mirroring_deployment_group: o.get_field("mirroringDeploymentGroup"),
            mirroring_endpoint_group_id: o.get_field("mirroringEndpointGroupId"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
        }
    }
}
