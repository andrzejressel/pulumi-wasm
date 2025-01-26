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
pub mod mirroring_endpoint_group_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MirroringEndpointGroupAssociationArgs {
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `networksecurity.googleapis.com/MirroringEndpointGroupAssociation`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Required. Immutable. The Mirroring Endpoint Group that this resource is connected to. Format
        /// is:
        /// `projects/{project}/locations/global/mirroringEndpointGroups/{mirroringEndpointGroup}`
        #[builder(into)]
        pub mirroring_endpoint_group: pulumi_wasm_rust::InputOrOutput<String>,
        /// Optional. Id of the requesting object
        /// If auto-generating Id server-side, remove this field and
        /// mirroring_endpoint_group_association_id from the method_signature of Create
        /// RPC
        #[builder(into, default)]
        pub mirroring_endpoint_group_association_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Required. Immutable. The VPC network associated. Format:
        /// projects/{project}/global/networks/{network}.
        #[builder(into)]
        pub network: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MirroringEndpointGroupAssociationResult {
        /// Output only. [Output only] Create time stamp
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `networksecurity.googleapis.com/MirroringEndpointGroupAssociation`.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// Output only. The list of locations that this association is in and its details.
        /// Structure is documented below.
        pub locations_details: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::networksecurity::MirroringEndpointGroupAssociationLocationsDetail,
            >,
        >,
        /// Required. Immutable. The Mirroring Endpoint Group that this resource is connected to. Format
        /// is:
        /// `projects/{project}/locations/global/mirroringEndpointGroups/{mirroringEndpointGroup}`
        pub mirroring_endpoint_group: pulumi_wasm_rust::Output<String>,
        /// Optional. Id of the requesting object
        /// If auto-generating Id server-side, remove this field and
        /// mirroring_endpoint_group_association_id from the method_signature of Create
        /// RPC
        pub mirroring_endpoint_group_association_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Immutable. Identifier. The name of the MirroringEndpointGroupAssociation.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Required. Immutable. The VPC network associated. Format:
        /// projects/{project}/global/networks/{network}.
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Whether reconciling is in progress, recommended per
        /// https://google.aip.dev/128.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// (Output)
        /// Output only. The association state in this location.
        /// Possible values:
        /// STATE_UNSPECIFIED
        /// ACTIVE
        /// OUT_OF_SYNC
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. [Output only] Update time stamp
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MirroringEndpointGroupAssociationArgs,
    ) -> MirroringEndpointGroupAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let mirroring_endpoint_group_binding = args
            .mirroring_endpoint_group
            .get_output(context)
            .get_inner();
        let mirroring_endpoint_group_association_id_binding = args
            .mirroring_endpoint_group_association_id
            .get_output(context)
            .get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/mirroringEndpointGroupAssociation:MirroringEndpointGroupAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mirroringEndpointGroup".into(),
                    value: &mirroring_endpoint_group_binding,
                },
                register_interface::ObjectField {
                    name: "mirroringEndpointGroupAssociationId".into(),
                    value: &mirroring_endpoint_group_association_id_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MirroringEndpointGroupAssociationResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            locations_details: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("locationsDetails"),
            ),
            mirroring_endpoint_group: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mirroringEndpointGroup"),
            ),
            mirroring_endpoint_group_association_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mirroringEndpointGroupAssociationId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
