/// Resource for managing an AWS CodeCatalyst Dev Environment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = dev_environment::create(
///         "test",
///         DevEnvironmentArgs::builder()
///             .alias("devenv")
///             .ides(
///                 DevEnvironmentIdes::builder()
///                     .name("PyCharm")
///                     .runtime("public.ecr.aws/jetbrains/py")
///                     .build_struct(),
///             )
///             .inactivity_timeout_minutes(40)
///             .instance_type("dev.standard1.small")
///             .persistent_storage(
///                 DevEnvironmentPersistentStorage::builder().size(16).build_struct(),
///             )
///             .project_name("myproject")
///             .repositories(
///                 vec![
///                     DevEnvironmentRepository::builder().branchName("main")
///                     .repositoryName("pulumi-provider-aws").build_struct(),
///                 ],
///             )
///             .space_name("myspace")
///             .build_struct(),
///     );
/// }
/// ```
pub mod dev_environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevEnvironmentArgs {
        #[builder(into, default)]
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// Information about the integrated development environment (IDE) configured for a Dev Environment.
        #[builder(into)]
        pub ides: pulumi_wasm_rust::Output<
            super::super::types::codecatalyst::DevEnvironmentIdes,
        >,
        /// The amount of time the Dev Environment will run without any activity detected before stopping, in minutes. Only whole integers are allowed. Dev Environments consume compute minutes when running.
        #[builder(into, default)]
        pub inactivity_timeout_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Amazon EC2 instace type to use for the Dev Environment. Valid values include dev.standard1.small,dev.standard1.medium,dev.standard1.large,dev.standard1.xlarge
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// Information about the amount of storage allocated to the Dev Environment.
        #[builder(into)]
        pub persistent_storage: pulumi_wasm_rust::Output<
            super::super::types::codecatalyst::DevEnvironmentPersistentStorage,
        >,
        /// The name of the project in the space.
        #[builder(into)]
        pub project_name: pulumi_wasm_rust::Output<String>,
        /// The source repository that contains the branch to clone into the Dev Environment.
        #[builder(into, default)]
        pub repositories: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codecatalyst::DevEnvironmentRepository>>,
        >,
        /// The name of the space.
        #[builder(into)]
        pub space_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DevEnvironmentResult {
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// Information about the integrated development environment (IDE) configured for a Dev Environment.
        pub ides: pulumi_wasm_rust::Output<
            super::super::types::codecatalyst::DevEnvironmentIdes,
        >,
        /// The amount of time the Dev Environment will run without any activity detected before stopping, in minutes. Only whole integers are allowed. Dev Environments consume compute minutes when running.
        pub inactivity_timeout_minutes: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Amazon EC2 instace type to use for the Dev Environment. Valid values include dev.standard1.small,dev.standard1.medium,dev.standard1.large,dev.standard1.xlarge
        ///
        /// The following arguments are optional:
        pub instance_type: pulumi_wasm_rust::Output<String>,
        /// Information about the amount of storage allocated to the Dev Environment.
        pub persistent_storage: pulumi_wasm_rust::Output<
            super::super::types::codecatalyst::DevEnvironmentPersistentStorage,
        >,
        /// The name of the project in the space.
        pub project_name: pulumi_wasm_rust::Output<String>,
        /// The source repository that contains the branch to clone into the Dev Environment.
        pub repositories: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::codecatalyst::DevEnvironmentRepository>>,
        >,
        /// The name of the space.
        pub space_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DevEnvironmentArgs) -> DevEnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_inner();
        let ides_binding = args.ides.get_inner();
        let inactivity_timeout_minutes_binding = args
            .inactivity_timeout_minutes
            .get_inner();
        let instance_type_binding = args.instance_type.get_inner();
        let persistent_storage_binding = args.persistent_storage.get_inner();
        let project_name_binding = args.project_name.get_inner();
        let repositories_binding = args.repositories.get_inner();
        let space_name_binding = args.space_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codecatalyst/devEnvironment:DevEnvironment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "ides".into(),
                    value: &ides_binding,
                },
                register_interface::ObjectField {
                    name: "inactivityTimeoutMinutes".into(),
                    value: &inactivity_timeout_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "persistentStorage".into(),
                    value: &persistent_storage_binding,
                },
                register_interface::ObjectField {
                    name: "projectName".into(),
                    value: &project_name_binding,
                },
                register_interface::ObjectField {
                    name: "repositories".into(),
                    value: &repositories_binding,
                },
                register_interface::ObjectField {
                    name: "spaceName".into(),
                    value: &space_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "ides".into(),
                },
                register_interface::ResultField {
                    name: "inactivityTimeoutMinutes".into(),
                },
                register_interface::ResultField {
                    name: "instanceType".into(),
                },
                register_interface::ResultField {
                    name: "persistentStorage".into(),
                },
                register_interface::ResultField {
                    name: "projectName".into(),
                },
                register_interface::ResultField {
                    name: "repositories".into(),
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
        DevEnvironmentResult {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            ides: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ides").unwrap(),
            ),
            inactivity_timeout_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inactivityTimeoutMinutes").unwrap(),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceType").unwrap(),
            ),
            persistent_storage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("persistentStorage").unwrap(),
            ),
            project_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectName").unwrap(),
            ),
            repositories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositories").unwrap(),
            ),
            space_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spaceName").unwrap(),
            ),
        }
    }
}
