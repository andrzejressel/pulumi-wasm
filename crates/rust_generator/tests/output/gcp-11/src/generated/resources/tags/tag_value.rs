/// A TagValue is a child of a particular TagKey. TagValues are used to group cloud resources for the purpose of controlling them using policies.
///
///
/// To get more information about TagValue, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/reference/rest/v3/tagValues)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing)
///
/// ## Example Usage
///
/// ### Tag Value Basic
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
///     let value = tag_value::create(
///         "value",
///         TagValueArgs::builder()
///             .description("For valuename resources.")
///             .parent("${key.id}")
///             .short_name("valuename")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TagValue can be imported using any of these accepted formats:
///
/// * `tagValues/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TagValue can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:tags/tagValue:TagValue default tagValues/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tags/tagValue:TagValue default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag_value {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagValueArgs {
        /// User-assigned description of the TagValue. Must not exceed 256 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Input only. The resource name of the new TagValue's parent. Must be of the form tagKeys/{tag_key_id}.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Input only. User-assigned short name for TagValue. The short name should be unique for TagValues within the same parent TagKey.
        /// The short name can have a maximum length of 256 characters. The permitted character set for the shortName includes all UTF-8 encoded Unicode characters except single quotes ('), double quotes ("), backslashes (\\), and forward slashes (/).
        ///
        ///
        /// - - -
        #[builder(into)]
        pub short_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagValueResult {
        /// Output only. Creation time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// User-assigned description of the TagValue. Must not exceed 256 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The generated numeric id for the TagValue.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Output only. Namespaced name of the TagValue. Will be in the format {parentNamespace}/{tagKeyShortName}/{shortName}.
        pub namespaced_name: pulumi_gestalt_rust::Output<String>,
        /// Input only. The resource name of the new TagValue's parent. Must be of the form tagKeys/{tag_key_id}.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Input only. User-assigned short name for TagValue. The short name should be unique for TagValues within the same parent TagKey.
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
        args: TagValueArgs,
    ) -> TagValueResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let short_name_binding = args.short_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:tags/tagValue:TagValue".into(),
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
                    name: "shortName".into(),
                    value: short_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TagValueResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            namespaced_name: o.get_field("namespacedName"),
            parent: o.get_field("parent"),
            short_name: o.get_field("shortName"),
            update_time: o.get_field("updateTime"),
        }
    }
}
