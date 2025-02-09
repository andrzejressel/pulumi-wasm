/// A Distributed Cloud Edge network, which provides L3 isolation within a zone.
///
///
/// To get more information about Network, see:
///
/// * [API documentation](https://cloud.google.com/distributed-cloud/edge/latest/docs/reference/network/rest/v1/projects.locations.zones.networks)
/// * How-to Guides
///     * [Create and manage networks](https://cloud.google.com/distributed-cloud/edge/latest/docs/networks#api)
///
/// ## Example Usage
///
/// ### Edgenetwork Network
///
///
/// ```yaml
/// resources:
///   exampleNetwork:
///     type: gcp:edgenetwork:Network
///     name: example_network
///     properties:
///       networkId: example-network
///       location: us-west1
///       zone: ""
///       description: Example network.
///       mtu: 9000
///       labels:
///         environment: dev
/// ```
///
/// ## Import
///
/// Network can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/zones/{{zone}}/networks/{{network_id}}`
///
/// * `{{project}}/{{location}}/{{zone}}/{{network_id}}`
///
/// * `{{location}}/{{zone}}/{{network_id}}`
///
/// * `{{location}}/{{network_id}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Network can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:edgenetwork/network:Network default projects/{{project}}/locations/{{location}}/zones/{{zone}}/networks/{{network_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgenetwork/network:Network default {{project}}/{{location}}/{{zone}}/{{network_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgenetwork/network:Network default {{location}}/{{zone}}/{{network_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgenetwork/network:Network default {{location}}/{{network_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:edgenetwork/network:Network default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels associated with this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Google Cloud region to which the target Distributed Cloud Edge zone belongs.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// IP (L3) MTU value of the network. Default value is `1500`. Possible values are: `1500`, `9000`.
        #[builder(into, default)]
        pub mtu: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A unique ID that identifies this network.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the target Distributed Cloud Edge zone.
        #[builder(into)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NetworkResult {
        /// The time when the subnet was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: `2014-10-02T15:01:23Z` and `2014-10-02T15:01:23.045123456Z`.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Labels associated with this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Google Cloud region to which the target Distributed Cloud Edge zone belongs.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// IP (L3) MTU value of the network. Default value is `1500`. Possible values are: `1500`, `9000`.
        pub mtu: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The canonical name of this resource, with format
        /// `projects/{{project}}/locations/{{location}}/zones/{{zone}}/networks/{{network_id}}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A unique ID that identifies this network.
        ///
        ///
        /// - - -
        pub network_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The time when the subnet was last updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: `2014-10-02T15:01:23Z` and `2014-10-02T15:01:23.045123456Z`.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the target Distributed Cloud Edge zone.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkArgs,
    ) -> NetworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let mtu_binding = args.mtu.get_output(context);
        let network_id_binding = args.network_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:edgenetwork/network:Network".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
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
                    name: "mtu".into(),
                    value: mtu_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkId".into(),
                    value: network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            mtu: o.get_field("mtu"),
            name: o.get_field("name"),
            network_id: o.get_field("networkId"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            update_time: o.get_field("updateTime"),
            zone: o.get_field("zone"),
        }
    }
}
