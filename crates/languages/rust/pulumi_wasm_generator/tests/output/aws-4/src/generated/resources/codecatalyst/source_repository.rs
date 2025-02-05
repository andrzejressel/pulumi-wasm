/// Resource for managing an AWS CodeCatalyst Source Repository.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = source_repository::create(
///         "example",
///         SourceRepositoryArgs::builder()
///             .name("example-repo")
///             .project_name("example-project")
///             .space_name("example-space")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeCatalyst Source Repository using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:codecatalyst/sourceRepository:SourceRepository example example-repo
/// ```
pub mod source_repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceRepositoryArgs {
        /// The description of the project. This description will be displayed to all users of the project. We recommend providing a brief description of the project and its intended purpose.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the source repository. For more information about name requirements, see [Quotas for source repositories](https://docs.aws.amazon.com/codecatalyst/latest/userguide/source-quotas.html).
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the project in the CodeCatalyst space.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub project_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the CodeCatalyst space.
        #[builder(into)]
        pub space_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SourceRepositoryResult {
        /// The description of the project. This description will be displayed to all users of the project. We recommend providing a brief description of the project and its intended purpose.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the source repository. For more information about name requirements, see [Quotas for source repositories](https://docs.aws.amazon.com/codecatalyst/latest/userguide/source-quotas.html).
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the project in the CodeCatalyst space.
        ///
        /// The following arguments are optional:
        pub project_name: pulumi_wasm_rust::Output<String>,
        /// The name of the CodeCatalyst space.
        pub space_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SourceRepositoryArgs,
    ) -> SourceRepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_name_binding = args.project_name.get_output(context).get_inner();
        let space_name_binding = args.space_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codecatalyst/sourceRepository:SourceRepository".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "projectName".into(),
                    value: &project_name_binding,
                },
                register_interface::ObjectField {
                    name: "spaceName".into(),
                    value: &space_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SourceRepositoryResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("projectName"),
            ),
            space_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("spaceName"),
            ),
        }
    }
}
