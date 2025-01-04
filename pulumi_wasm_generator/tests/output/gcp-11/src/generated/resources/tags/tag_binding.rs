/// A TagBinding represents a connection between a TagValue and a cloud resource (currently project, folder, or organization). Once a TagBinding is created, the TagValue is applied to all the descendants of the cloud resource.
///
///
/// To get more information about TagBinding, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/reference/rest/v3/tagBindings)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing)
///
/// ## Example Usage
///
/// ### Tag Binding Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = tag_binding::create(
///         "binding",
///         TagBindingArgs::builder()
///             .parent("//cloudresourcemanager.googleapis.com/projects/${project.number}")
///             .tag_value("${value.id}")
///             .build_struct(),
///     );
///     let key = tag_key::create(
///         "key",
///         TagKeyArgs::builder()
///             .description("For keyname resources.")
///             .parent("organizations/123456789")
///             .short_name("keyname")
///             .build_struct(),
///     );
///     let project = project::create(
///         "project",
///         ProjectArgs::builder()
///             .deletion_policy("DELETE")
///             .name("project_id")
///             .org_id("123456789")
///             .project_id("project_id")
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
/// TagBinding can be imported using any of these accepted formats:
///
/// * `tagBindings/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TagBinding can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:tags/tagBinding:TagBinding default tagBindings/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tags/tagBinding:TagBinding default {{name}}
/// ```
///
pub mod tag_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagBindingArgs {
        /// The full resource name of the resource the TagValue is bound to. E.g. //cloudresourcemanager.googleapis.com/projects/123
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The TagValue of the TagBinding. Must be of the form tagValues/456.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub tag_value: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TagBindingResult {
        /// The generated id for the TagBinding. This is a string of the form: `tagBindings/{full-resource-name}/{tag-value-name}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The full resource name of the resource the TagValue is bound to. E.g. //cloudresourcemanager.googleapis.com/projects/123
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The TagValue of the TagBinding. Must be of the form tagValues/456.
        ///
        ///
        /// - - -
        pub tag_value: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TagBindingArgs) -> TagBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_inner();
        let tag_value_binding = args.tag_value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:tags/tagBinding:TagBinding".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "tagValue".into(),
                    value: &tag_value_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "tagValue".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TagBindingResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            tag_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagValue").unwrap(),
            ),
        }
    }
}
