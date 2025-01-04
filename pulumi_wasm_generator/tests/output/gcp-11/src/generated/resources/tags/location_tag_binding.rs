/// A LocationTagBinding represents a connection between a TagValue and a non-global target such as a Cloud Run Service or Compute Instance. Once a LocationTagBinding is created, the TagValue is applied to all the descendants of the cloud resource.
///
/// To get more information about LocationTagBinding, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/reference/rest/v3/tagBindings)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/resource-manager/docs/tags/tags-creating-and-managing)
///
/// ## Example Usage
///
/// ### Cloud Run Service
///
/// To bind a tag to a Cloud Run service:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = location_tag_binding::create(
///         "binding",
///         LocationTagBindingArgs::builder()
///             .location("us-central1")
///             .parent(
///                 "//run.googleapis.com/projects/${projectGoogleProject.number}/locations/${default.location}/services/${default.name}",
///             )
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
/// ### Compute Instance
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = location_tag_binding::create(
///         "binding",
///         LocationTagBindingArgs::builder()
///             .location("us-central1-a")
///             .parent(
///                 "//compute.googleapis.com/projects/${project.number}/zones/us-central1-a/instances/${instance.instanceId}",
///             )
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
/// LocationTagBinding can be imported using any of these accepted formats:
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, TagBinding can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:tags/locationTagBinding:LocationTagBinding default {{location}}/{{name}}
/// ```
///
pub mod location_tag_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationTagBindingArgs {
        /// Location of the target resource.
        ///
        /// - - -
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The full resource name of the resource the TagValue is bound to. E.g. //cloudresourcemanager.googleapis.com/projects/123
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The TagValue of the TagBinding. Must be of the form tagValues/456.
        #[builder(into)]
        pub tag_value: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LocationTagBindingResult {
        /// Location of the target resource.
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The generated id for the TagBinding. This is a string of the form: `tagBindings/{parent}/{tag-value-name}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The full resource name of the resource the TagValue is bound to. E.g. //cloudresourcemanager.googleapis.com/projects/123
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The TagValue of the TagBinding. Must be of the form tagValues/456.
        pub tag_value: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LocationTagBindingArgs) -> LocationTagBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let parent_binding = args.parent.get_inner();
        let tag_value_binding = args.tag_value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:tags/locationTagBinding:LocationTagBinding".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
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
                    name: "location".into(),
                },
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
        LocationTagBindingResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
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
