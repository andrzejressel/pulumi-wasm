/// ## Example Usage
///
/// ### Network Security Mirroring Endpoint Group Association Basic
///
///
/// ```yaml
/// resources:
///   producerNetwork:
///     type: gcp:compute:Network
///     name: producer_network
///     properties:
///       name: example-prod-network
///       autoCreateSubnetworks: false
///   consumerNetwork:
///     type: gcp:compute:Network
///     name: consumer_network
///     properties:
///       name: example-cons-network
///       autoCreateSubnetworks: false
///   deploymentGroup:
///     type: gcp:networksecurity:MirroringDeploymentGroup
///     name: deployment_group
///     properties:
///       mirroringDeploymentGroupId: example-dg
///       location: global
///       network: ${producerNetwork.id}
///   endpointGroup:
///     type: gcp:networksecurity:MirroringEndpointGroup
///     name: endpoint_group
///     properties:
///       mirroringEndpointGroupId: example-eg
///       location: global
///       mirroringDeploymentGroup: ${deploymentGroup.id}
///   default:
///     type: gcp:networksecurity:MirroringEndpointGroupAssociation
///     properties:
///       mirroringEndpointGroupAssociationId: example-ega
///       location: global
///       network: ${consumerNetwork.id}
///       mirroringEndpointGroup: ${endpointGroup.id}
///       labels:
///         foo: bar
/// ```
///
/// ## Import
///
/// MirroringEndpointGroupAssociation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/mirroringEndpointGroupAssociations/{{mirroring_endpoint_group_association_id}}`
///
/// * `{{project}}/{{location}}/{{mirroring_endpoint_group_association_id}}`
///
/// * `{{location}}/{{mirroring_endpoint_group_association_id}}`
///
/// When using the `pulumi import` command, MirroringEndpointGroupAssociation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/mirroringEndpointGroupAssociation:MirroringEndpointGroupAssociation default projects/{{project}}/locations/{{location}}/mirroringEndpointGroupAssociations/{{mirroring_endpoint_group_association_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/mirroringEndpointGroupAssociation:MirroringEndpointGroupAssociation default {{project}}/{{location}}/{{mirroring_endpoint_group_association_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/mirroringEndpointGroupAssociation:MirroringEndpointGroupAssociation default {{location}}/{{mirroring_endpoint_group_association_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mirroring_endpoint_group_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MirroringEndpointGroupAssociationArgs {
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `networksecurity.googleapis.com/MirroringEndpointGroupAssociation`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. Immutable. The Mirroring Endpoint Group that this resource is connected to. Format
        /// is:
        /// `projects/{project}/locations/global/mirroringEndpointGroups/{mirroringEndpointGroup}`
        #[builder(into)]
        pub mirroring_endpoint_group: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Id of the requesting object
        /// If auto-generating Id server-side, remove this field and
        /// mirroring_endpoint_group_association_id from the method_signature of Create
        /// RPC
        #[builder(into, default)]
        pub mirroring_endpoint_group_association_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Required. Immutable. The VPC network associated. Format:
        /// projects/{project}/global/networks/{network}.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MirroringEndpointGroupAssociationResult {
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
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `networksecurity.googleapis.com/MirroringEndpointGroupAssociation`.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Output only. The list of locations that this association is in and its details.
        /// Structure is documented below.
        pub locations_details: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::networksecurity::MirroringEndpointGroupAssociationLocationsDetail,
            >,
        >,
        /// Required. Immutable. The Mirroring Endpoint Group that this resource is connected to. Format
        /// is:
        /// `projects/{project}/locations/global/mirroringEndpointGroups/{mirroringEndpointGroup}`
        pub mirroring_endpoint_group: pulumi_gestalt_rust::Output<String>,
        /// Optional. Id of the requesting object
        /// If auto-generating Id server-side, remove this field and
        /// mirroring_endpoint_group_association_id from the method_signature of Create
        /// RPC
        pub mirroring_endpoint_group_association_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Immutable. Identifier. The name of the MirroringEndpointGroupAssociation.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Required. Immutable. The VPC network associated. Format:
        /// projects/{project}/global/networks/{network}.
        pub network: pulumi_gestalt_rust::Output<String>,
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
        /// (Output)
        /// Output only. The association state in this location.
        /// Possible values:
        /// STATE_UNSPECIFIED
        /// ACTIVE
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
        args: MirroringEndpointGroupAssociationArgs,
    ) -> MirroringEndpointGroupAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let mirroring_endpoint_group_binding = args
            .mirroring_endpoint_group
            .get_output(context);
        let mirroring_endpoint_group_association_id_binding = args
            .mirroring_endpoint_group_association_id
            .get_output(context);
        let network_binding = args.network.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/mirroringEndpointGroupAssociation:MirroringEndpointGroupAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mirroringEndpointGroup".into(),
                    value: &mirroring_endpoint_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mirroringEndpointGroupAssociationId".into(),
                    value: &mirroring_endpoint_group_association_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MirroringEndpointGroupAssociationResult {
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            locations_details: o.get_field("locationsDetails"),
            mirroring_endpoint_group: o.get_field("mirroringEndpointGroup"),
            mirroring_endpoint_group_association_id: o
                .get_field("mirroringEndpointGroupAssociationId"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
        }
    }
}
