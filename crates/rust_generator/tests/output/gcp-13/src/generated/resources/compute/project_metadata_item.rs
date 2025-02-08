/// Manages a single key/value pair on metadata common to all instances for
/// a project in GCE. Using `gcp.compute.ProjectMetadataItem` lets you
/// manage a single key/value setting in the provider rather than the entire
/// project metadata map.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = project_metadata_item::create(
///         "default",
///         ProjectMetadataItemArgs::builder()
///             .key("my_metadata")
///             .value("my_value")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Project metadata items can be imported using the `key`, e.g.
///
/// * `{{key}}`
///
/// When using the `pulumi import` command, project metadata items can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/projectMetadataItem:ProjectMetadataItem default {{key}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod project_metadata_item {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectMetadataItemArgs {
        /// The metadata key to set.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The value to set for the given metadata key.
        ///
        /// - - -
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProjectMetadataItemResult {
        /// The metadata key to set.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The value to set for the given metadata key.
        ///
        /// - - -
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProjectMetadataItemArgs,
    ) -> ProjectMetadataItemResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_binding = args.key.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/projectMetadataItem:ProjectMetadataItem".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProjectMetadataItemResult {
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
