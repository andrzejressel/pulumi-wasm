/// AddressGroup is a resource that specifies how a collection of IP/DNS used in Firewall Policy.
///
///
/// To get more information about AddressGroup, see:
///
/// * [API documentation](https://cloud.google.com/traffic-director/docs/reference/network-security/rest/v1beta1/organizations.locations.addressGroups)
/// * How-to Guides
///     * [Use AddressGroups](https://cloud.google.com/vpc/docs/use-address-groups-firewall-policies)
///
/// ## Example Usage
///
/// ### Network Security Address Groups Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:AddressGroup
///     properties:
///       name: my-address-groups
///       parent: projects/my-project-name
///       location: us-central1
///       type: IPV4
///       capacity: '100'
///       items:
///         - 208.80.154.224/32
/// ```
/// ### Network Security Address Groups Organization Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:AddressGroup
///     properties:
///       name: my-address-groups
///       parent: organizations/123456789
///       location: us-central1
///       type: IPV4
///       capacity: '100'
///       items:
///         - 208.80.154.224/32
/// ```
/// ### Network Security Address Groups Advanced
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:AddressGroup
///     properties:
///       name: my-address-groups
///       parent: projects/my-project-name
///       location: us-central1
///       description: my description
///       type: IPV4
///       capacity: '100'
///       items:
///         - 208.80.154.224/32
/// ```
/// ### Network Security Address Groups Cloud Armor
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:AddressGroup
///     properties:
///       name: my-address-groups
///       parent: projects/my-project-name
///       location: global
///       type: IPV4
///       capacity: '100'
///       purposes:
///         - CLOUD_ARMOR
///       items:
///         - 208.80.154.224/32
/// ```
///
/// ## Import
///
/// AddressGroup can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/addressGroups/{{name}}`
///
/// When using the `pulumi import` command, AddressGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/addressGroup:AddressGroup default {{parent}}/locations/{{location}}/addressGroups/{{name}}
/// ```
///
pub mod address_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AddressGroupArgs {
        /// Capacity of the Address Group.
        #[builder(into)]
        pub capacity: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Free-text description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of items.
        #[builder(into, default)]
        pub items: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Set of label tags associated with the AddressGroup resource.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the gateway security policy.
        /// The default value is `global`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the AddressGroup resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the parent this address group belongs to. Format: organizations/{organization_id} or projects/{project_id}.
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of supported purposes of the Address Group.
        /// Each value may be one of: `DEFAULT`, `CLOUD_ARMOR`.
        #[builder(into, default)]
        pub purposes: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The type of the Address Group. Possible values are "IPV4" or "IPV6".
        /// Possible values are: `IPV4`, `IPV6`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AddressGroupResult {
        /// Capacity of the Address Group.
        pub capacity: pulumi_wasm_rust::Output<i32>,
        /// The timestamp when the resource was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z"
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Free-text description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of items.
        pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Set of label tags associated with the AddressGroup resource.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the gateway security policy.
        /// The default value is `global`.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the AddressGroup resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the parent this address group belongs to. Format: organizations/{organization_id} or projects/{project_id}.
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of supported purposes of the Address Group.
        /// Each value may be one of: `DEFAULT`, `CLOUD_ARMOR`.
        pub purposes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The type of the Address Group. Possible values are "IPV4" or "IPV6".
        /// Possible values are: `IPV4`, `IPV6`.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the resource was updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AddressGroupArgs,
    ) -> AddressGroupResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_binding = args.capacity.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let items_binding = args.items.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let purposes_binding = args.purposes.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/addressGroup:AddressGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "items".into(),
                    value: &items_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "purposes".into(),
                    value: &purposes_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AddressGroupResult {
            capacity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacity"),
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
            items: pulumi_wasm_rust::__private::into_domain(o.extract_field("items")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            purposes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("purposes"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
