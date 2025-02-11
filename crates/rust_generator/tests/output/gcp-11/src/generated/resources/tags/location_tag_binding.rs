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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod location_tag_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationTagBindingArgs {
        /// Location of the target resource.
        ///
        /// - - -
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The full resource name of the resource the TagValue is bound to. E.g. //cloudresourcemanager.googleapis.com/projects/123
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The TagValue of the TagBinding. Must be of the form tagValues/456.
        #[builder(into)]
        pub tag_value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LocationTagBindingResult {
        /// Location of the target resource.
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The generated id for the TagBinding. This is a string of the form: `tagBindings/{parent}/{tag-value-name}`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The full resource name of the resource the TagValue is bound to. E.g. //cloudresourcemanager.googleapis.com/projects/123
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// The TagValue of the TagBinding. Must be of the form tagValues/456.
        pub tag_value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocationTagBindingArgs,
    ) -> LocationTagBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let tag_value_binding = args.tag_value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:tags/locationTagBinding:LocationTagBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagValue".into(),
                    value: &tag_value_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocationTagBindingResult {
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            tag_value: o.get_field("tagValue"),
        }
    }
}
