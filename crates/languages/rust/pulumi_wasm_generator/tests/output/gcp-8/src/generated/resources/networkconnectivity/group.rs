/// The NetworkConnectivity Group resource
///
///
/// To get more information about Group, see:
///
/// * [API documentation](https://cloud.google.com/network-connectivity/docs/reference/networkconnectivity/rest/v1beta/projects.locations.global.hubs.groups)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/network-connectivity/docs/network-connectivity-center/concepts/overview)
///
/// ## Example Usage
///
/// ### Network Connectivity Group Basic
///
///
/// ```yaml
/// resources:
///   basicHub:
///     type: gcp:networkconnectivity:Hub
///     name: basic_hub
///     properties:
///       name: network-connectivity-hub1
///       description: A sample hub
///       labels:
///         label-one: value-one
///   primary:
///     type: gcp:networkconnectivity:Group
///     properties:
///       hub: ${basicHub.id}
///       name: default
///       labels:
///         label-one: value-one
///       description: A sample hub group
///       autoAccept:
///         autoAcceptProjects:
///           - foo
///           - bar
/// ```
///
/// ## Import
///
/// Group can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/hubs/{{hub}}/groups/{{name}}`
///
/// * `{{project}}/{{hub}}/{{name}}`
///
/// * `{{hub}}/{{name}}`
///
/// When using the `pulumi import` command, Group can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/group:Group default projects/{{project}}/locations/global/hubs/{{hub}}/groups/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/group:Group default {{project}}/{{hub}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkconnectivity/group:Group default {{hub}}/{{name}}
/// ```
///
pub mod group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// Optional. The auto-accept setting for this group.
        /// Structure is documented below.
        #[builder(into, default)]
        pub auto_accept: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::networkconnectivity::GroupAutoAccept>,
        >,
        /// An optional description of the group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the hub. Hub names must be unique. They use the following form: projects/{projectNumber}/locations/global/hubs/{hubId}
        ///
        ///
        /// - - -
        #[builder(into)]
        pub hub: pulumi_wasm_rust::InputOrOutput<String>,
        /// Optional labels in key:value format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the group. Group names must be unique.
        /// Possible values are: `default`, `center`, `edge`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// Optional. The auto-accept setting for this group.
        /// Structure is documented below.
        pub auto_accept: pulumi_wasm_rust::Output<
            Option<super::super::types::networkconnectivity::GroupAutoAccept>,
        >,
        /// Output only. The time the hub was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// An optional description of the group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the hub. Hub names must be unique. They use the following form: projects/{projectNumber}/locations/global/hubs/{hubId}
        ///
        ///
        /// - - -
        pub hub: pulumi_wasm_rust::Output<String>,
        /// Optional labels in key:value format. For more information about labels, see [Requirements for labels](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements).
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the group. Group names must be unique.
        /// Possible values are: `default`, `center`, `edge`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The name of the route table that corresponds to this group. They use the following form: `projects/{projectNumber}/locations/global/hubs/{hubId}/routeTables/{route_table_id}`
        pub route_table: pulumi_wasm_rust::Output<String>,
        /// Output only. The current lifecycle state of this hub.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. The Google-generated UUID for the group. This value is unique across all group resources. If a group is deleted and another with the same name is created, the new route table is assigned a different uniqueId.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. The time the hub was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GroupArgs,
    ) -> GroupResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_accept_binding = args.auto_accept.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let hub_binding = args.hub.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkconnectivity/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoAccept".into(),
                    value: &auto_accept_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "hub".into(),
                    value: &hub_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupResult {
            auto_accept: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoAccept"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            hub: pulumi_wasm_rust::__private::into_domain(o.extract_field("hub")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            route_table: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("routeTable"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
