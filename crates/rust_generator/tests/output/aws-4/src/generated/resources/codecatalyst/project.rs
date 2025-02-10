/// Resource for managing an AWS CodeCatalyst Project.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = project::create(
///         "test",
///         ProjectArgs::builder()
///             .description("My CodeCatalyst Project created using Pulumi")
///             .display_name("MyProject")
///             .space_name("myproject")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeCatalyst Project using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:codecatalyst/project:Project example project-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// The description of the project. This description will be displayed to all users of the project. We recommend providing a brief description of the project and its intended purpose.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The friendly name of the project that will be displayed to users.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the space.
        #[builder(into)]
        pub space_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// The description of the project. This description will be displayed to all users of the project. We recommend providing a brief description of the project and its intended purpose.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The friendly name of the project that will be displayed to users.
        ///
        /// The following arguments are optional:
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the project in the space.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the space.
        pub space_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let space_name_binding = args.space_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codecatalyst/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spaceName".into(),
                    value: space_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            space_name: o.get_field("spaceName"),
        }
    }
}
