/// A TagKey, used to group a set of TagValues.
///
///
/// To get more information about TagKey, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/reference/rest/v3/tagKeys)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing)
///
/// ## Example Usage
///
/// ### Tag Key Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let key = tag_key::create(
///         "key",
///         TagKeyArgs::builder()
///             .description("For keyname resources.")
///             .parent("organizations/123456789")
///             .short_name("keyname")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TagKey can be imported using any of these accepted formats:
///
/// * `tagKeys/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TagKey can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:tags/tagKey:TagKey default tagKeys/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tags/tagKey:TagKey default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagKeyArgs {
        /// User-assigned description of the TagKey. Must not exceed 256 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Input only. The resource name of the new TagKey's parent. Must be of the form organizations/{org_id} or projects/{project_id_or_number}.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. A purpose cannot be changed once set.
        /// A purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag.
        /// Possible values are: `GCE_FIREWALL`.
        #[builder(into, default)]
        pub purpose: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Purpose data cannot be changed once set.
        /// Purpose data corresponds to the policy system that the tag is intended for. For example, the GCE_FIREWALL purpose expects data in the following format: `network = "<project-name>/<vpc-name>"`.
        #[builder(into, default)]
        pub purpose_data: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Input only. The user friendly name for a TagKey. The short name should be unique for TagKeys within the same tag namespace.
        /// The short name can have a maximum length of 256 characters. The permitted character set for the shortName includes all UTF-8 encoded Unicode characters except single quotes ('), double quotes ("), backslashes (\\), and forward slashes (/).
        ///
        ///
        /// - - -
        #[builder(into)]
        pub short_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagKeyResult {
        /// Output only. Creation time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// User-assigned description of the TagKey. Must not exceed 256 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The generated numeric id for the TagKey.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Output only. Namespaced name of the TagKey.
        pub namespaced_name: pulumi_gestalt_rust::Output<String>,
        /// Input only. The resource name of the new TagKey's parent. Must be of the form organizations/{org_id} or projects/{project_id_or_number}.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Optional. A purpose cannot be changed once set.
        /// A purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag.
        /// Possible values are: `GCE_FIREWALL`.
        pub purpose: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Purpose data cannot be changed once set.
        /// Purpose data corresponds to the policy system that the tag is intended for. For example, the GCE_FIREWALL purpose expects data in the following format: `network = "<project-name>/<vpc-name>"`.
        pub purpose_data: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Input only. The user friendly name for a TagKey. The short name should be unique for TagKeys within the same tag namespace.
        /// The short name can have a maximum length of 256 characters. The permitted character set for the shortName includes all UTF-8 encoded Unicode characters except single quotes ('), double quotes ("), backslashes (\\), and forward slashes (/).
        ///
        ///
        /// - - -
        pub short_name: pulumi_gestalt_rust::Output<String>,
        /// Output only. Update time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TagKeyArgs,
    ) -> TagKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let purpose_binding = args.purpose.get_output(context);
        let purpose_data_binding = args.purpose_data.get_output(context);
        let short_name_binding = args.short_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:tags/tagKey:TagKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purpose".into(),
                    value: purpose_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purposeData".into(),
                    value: purpose_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shortName".into(),
                    value: short_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TagKeyResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            namespaced_name: o.get_field("namespacedName"),
            parent: o.get_field("parent"),
            purpose: o.get_field("purpose"),
            purpose_data: o.get_field("purposeData"),
            short_name: o.get_field("shortName"),
            update_time: o.get_field("updateTime"),
        }
    }
}
