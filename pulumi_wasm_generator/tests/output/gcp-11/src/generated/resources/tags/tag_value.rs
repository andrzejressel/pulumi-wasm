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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod tag_value {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagValueArgs {
        /// User-assigned description of the TagValue. Must not exceed 256 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Input only. The resource name of the new TagValue's parent. Must be of the form tagKeys/{tag_key_id}.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
        /// Input only. User-assigned short name for TagValue. The short name should be unique for TagValues within the same parent TagKey.
        /// The short name can have a maximum length of 256 characters. The permitted character set for the shortName includes all UTF-8 encoded Unicode characters except single quotes ('), double quotes ("), backslashes (\\), and forward slashes (/).
        ///
        ///
        /// - - -
        #[builder(into)]
        pub short_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagValueResult {
        /// Output only. Creation time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// User-assigned description of the TagValue. Must not exceed 256 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The generated numeric id for the TagValue.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Output only. Namespaced name of the TagValue. Will be in the format {parentNamespace}/{tagKeyShortName}/{shortName}.
        pub namespaced_name: pulumi_wasm_rust::Output<String>,
        /// Input only. The resource name of the new TagValue's parent. Must be of the form tagKeys/{tag_key_id}.
        pub parent: pulumi_wasm_rust::Output<String>,
        /// Input only. User-assigned short name for TagValue. The short name should be unique for TagValues within the same parent TagKey.
        /// The short name can have a maximum length of 256 characters. The permitted character set for the shortName includes all UTF-8 encoded Unicode characters except single quotes ('), double quotes ("), backslashes (\\), and forward slashes (/).
        ///
        ///
        /// - - -
        pub short_name: pulumi_wasm_rust::Output<String>,
        /// Output only. Update time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TagValueArgs,
    ) -> TagValueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let short_name_binding = args.short_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:tags/tagValue:TagValue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "shortName".into(),
                    value: &short_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TagValueResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            namespaced_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespacedName"),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            short_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shortName"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
