/// Resource for managing an AWS CodeCatalyst Project.
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
pub mod project {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// The description of the project. This description will be displayed to all users of the project. We recommend providing a brief description of the project and its intended purpose.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The friendly name of the project that will be displayed to users.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The name of the space.
        #[builder(into)]
        pub space_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// The description of the project. This description will be displayed to all users of the project. We recommend providing a brief description of the project and its intended purpose.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The friendly name of the project that will be displayed to users.
        ///
        /// The following arguments are optional:
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The name of the project in the space.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the space.
        pub space_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProjectArgs) -> ProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let space_name_binding = args.space_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codecatalyst/project:Project".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "spaceName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProjectResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            space_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spaceName").unwrap(),
            ),
        }
    }
}