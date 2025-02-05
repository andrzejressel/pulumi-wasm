/// The PrivateConnection resource is used to establish private connectivity between Datastream and a customer's network.
///
///
/// To get more information about PrivateConnection, see:
///
/// * [API documentation](https://cloud.google.com/datastream/docs/reference/rest/v1/projects.locations.privateConnections)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/datastream/docs/create-a-private-connectivity-configuration)
///
/// ## Example Usage
///
/// ### Datastream Private Connection Full
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:datastream:PrivateConnection
///     properties:
///       displayName: Connection profile
///       location: us-central1
///       privateConnectionId: my-connection
///       labels:
///         key: value
///       vpcPeeringConfig:
///         vpc: ${defaultNetwork.id}
///         subnet: 10.0.0.0/29
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: my-network
/// ```
///
/// ## Import
///
/// PrivateConnection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/privateConnections/{{private_connection_id}}`
///
/// * `{{project}}/{{location}}/{{private_connection_id}}`
///
/// * `{{location}}/{{private_connection_id}}`
///
/// When using the `pulumi import` command, PrivateConnection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datastream/privateConnection:PrivateConnection default projects/{{project}}/locations/{{location}}/privateConnections/{{private_connection_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datastream/privateConnection:PrivateConnection default {{project}}/{{location}}/{{private_connection_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datastream/privateConnection:PrivateConnection default {{location}}/{{private_connection_id}}
/// ```
///
pub mod private_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrivateConnectionArgs {
        /// If set to true, will skip validations.
        #[builder(into, default)]
        pub create_without_validation: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Display name.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Labels. **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please
        /// refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location this private connection is located in.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The private connectivity identifier.
        #[builder(into)]
        pub private_connection_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The VPC Peering configuration is used to create VPC peering
        /// between Datastream and the consumer's VPC.
        /// Structure is documented below.
        #[builder(into)]
        pub vpc_peering_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::datastream::PrivateConnectionVpcPeeringConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct PrivateConnectionResult {
        /// If set to true, will skip validations.
        pub create_without_validation: pulumi_wasm_rust::Output<Option<bool>>,
        /// Display name.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The PrivateConnection error in case of failure.
        /// Structure is documented below.
        pub errors: pulumi_wasm_rust::Output<
            Vec<super::super::types::datastream::PrivateConnectionError>,
        >,
        /// Labels. **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please
        /// refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location this private connection is located in.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource's name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The private connectivity identifier.
        pub private_connection_id: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// State of the PrivateConnection.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The VPC Peering configuration is used to create VPC peering
        /// between Datastream and the consumer's VPC.
        /// Structure is documented below.
        pub vpc_peering_config: pulumi_wasm_rust::Output<
            super::super::types::datastream::PrivateConnectionVpcPeeringConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PrivateConnectionArgs,
    ) -> PrivateConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let create_without_validation_binding = args
            .create_without_validation
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let private_connection_id_binding = args
            .private_connection_id
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let vpc_peering_config_binding = args
            .vpc_peering_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:datastream/privateConnection:PrivateConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "createWithoutValidation".into(),
                    value: &create_without_validation_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
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
                    name: "privateConnectionId".into(),
                    value: &private_connection_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "vpcPeeringConfig".into(),
                    value: &vpc_peering_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PrivateConnectionResult {
            create_without_validation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createWithoutValidation"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            errors: pulumi_wasm_rust::__private::into_domain(o.extract_field("errors")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            private_connection_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateConnectionId"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            vpc_peering_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcPeeringConfig"),
            ),
        }
    }
}
