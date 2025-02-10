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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod address_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AddressGroupArgs {
        /// Capacity of the Address Group.
        #[builder(into)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Free-text description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of items.
        #[builder(into, default)]
        pub items: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Set of label tags associated with the AddressGroup resource.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the gateway security policy.
        /// The default value is `global`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the AddressGroup resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the parent this address group belongs to. Format: organizations/{organization_id} or projects/{project_id}.
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of supported purposes of the Address Group.
        /// Each value may be one of: `DEFAULT`, `CLOUD_ARMOR`.
        #[builder(into, default)]
        pub purposes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The type of the Address Group. Possible values are "IPV4" or "IPV6".
        /// Possible values are: `IPV4`, `IPV6`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AddressGroupResult {
        /// Capacity of the Address Group.
        pub capacity: pulumi_gestalt_rust::Output<i32>,
        /// The timestamp when the resource was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z"
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Free-text description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of items.
        pub items: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Set of label tags associated with the AddressGroup resource.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the gateway security policy.
        /// The default value is `global`.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the AddressGroup resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the parent this address group belongs to. Format: organizations/{organization_id} or projects/{project_id}.
        pub parent: pulumi_gestalt_rust::Output<Option<String>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of supported purposes of the Address Group.
        /// Each value may be one of: `DEFAULT`, `CLOUD_ARMOR`.
        pub purposes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The type of the Address Group. Possible values are "IPV4" or "IPV6".
        /// Possible values are: `IPV4`, `IPV6`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the resource was updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AddressGroupArgs,
    ) -> AddressGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_binding = args.capacity.get_output(context);
        let description_binding = args.description.get_output(context);
        let items_binding = args.items.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let purposes_binding = args.purposes.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/addressGroup:AddressGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacity".into(),
                    value: capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "items".into(),
                    value: items_binding.get_id(),
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
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purposes".into(),
                    value: purposes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AddressGroupResult {
            capacity: o.get_field("capacity"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            items: o.get_field("items"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            pulumi_labels: o.get_field("pulumiLabels"),
            purposes: o.get_field("purposes"),
            type_: o.get_field("type"),
            update_time: o.get_field("updateTime"),
        }
    }
}
