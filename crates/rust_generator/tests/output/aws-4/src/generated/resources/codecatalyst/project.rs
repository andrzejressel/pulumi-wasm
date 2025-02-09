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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProjectArgs,
    ) -> ProjectResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let space_name_binding_1 = args.space_name.get_output(context);
        let space_name_binding = space_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codecatalyst/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "spaceName".into(),
                    value: &space_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProjectResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            space_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("spaceName"),
            ),
        }
    }
}
