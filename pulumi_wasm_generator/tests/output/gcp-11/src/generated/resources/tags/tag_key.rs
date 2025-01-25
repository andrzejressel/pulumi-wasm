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
pub mod tag_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagKeyArgs {
        /// User-assigned description of the TagKey. Must not exceed 256 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Input only. The resource name of the new TagKey's parent. Must be of the form organizations/{org_id} or projects/{project_id_or_number}.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
        /// Optional. A purpose cannot be changed once set.
        /// A purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag.
        /// Possible values are: `GCE_FIREWALL`.
        #[builder(into, default)]
        pub purpose: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. Purpose data cannot be changed once set.
        /// Purpose data corresponds to the policy system that the tag is intended for. For example, the GCE_FIREWALL purpose expects data in the following format: `network = "<project-name>/<vpc-name>"`.
        #[builder(into, default)]
        pub purpose_data: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Input only. The user friendly name for a TagKey. The short name should be unique for TagKeys within the same tag namespace.
        /// The short name can have a maximum length of 256 characters. The permitted character set for the shortName includes all UTF-8 encoded Unicode characters except single quotes ('), double quotes ("), backslashes (\\), and forward slashes (/).
        ///
        ///
        /// - - -
        #[builder(into)]
        pub short_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagKeyResult {
        /// Output only. Creation time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// User-assigned description of the TagKey. Must not exceed 256 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The generated numeric id for the TagKey.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Output only. Namespaced name of the TagKey.
        pub namespaced_name: pulumi_wasm_rust::Output<String>,
        /// Input only. The resource name of the new TagKey's parent. Must be of the form organizations/{org_id} or projects/{project_id_or_number}.
        pub parent: pulumi_wasm_rust::Output<String>,
        /// Optional. A purpose cannot be changed once set.
        /// A purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag.
        /// Possible values are: `GCE_FIREWALL`.
        pub purpose: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. Purpose data cannot be changed once set.
        /// Purpose data corresponds to the policy system that the tag is intended for. For example, the GCE_FIREWALL purpose expects data in the following format: `network = "<project-name>/<vpc-name>"`.
        pub purpose_data: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Input only. The user friendly name for a TagKey. The short name should be unique for TagKeys within the same tag namespace.
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
        args: TagKeyArgs,
    ) -> TagKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let purpose_binding = args.purpose.get_output(context).get_inner();
        let purpose_data_binding = args.purpose_data.get_output(context).get_inner();
        let short_name_binding = args.short_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:tags/tagKey:TagKey".into(),
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
                    name: "purpose".into(),
                    value: &purpose_binding,
                },
                register_interface::ObjectField {
                    name: "purposeData".into(),
                    value: &purpose_data_binding,
                },
                register_interface::ObjectField {
                    name: "shortName".into(),
                    value: &short_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespacedName".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "purpose".into(),
                },
                register_interface::ResultField {
                    name: "purposeData".into(),
                },
                register_interface::ResultField {
                    name: "shortName".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TagKeyResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespaced_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespacedName").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            purpose: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purpose").unwrap(),
            ),
            purpose_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purposeData").unwrap(),
            ),
            short_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shortName").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
