/// Resource for managing an AWS CodeCatalyst Dev Environment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dev_environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevEnvironmentArgs {
        #[builder(into, default)]
        pub alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Information about the integrated development environment (IDE) configured for a Dev Environment.
        #[builder(into)]
        pub ides: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::codecatalyst::DevEnvironmentIdes,
        >,
        /// The amount of time the Dev Environment will run without any activity detected before stopping, in minutes. Only whole integers are allowed. Dev Environments consume compute minutes when running.
        #[builder(into, default)]
        pub inactivity_timeout_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The Amazon EC2 instace type to use for the Dev Environment. Valid values include dev.standard1.small,dev.standard1.medium,dev.standard1.large,dev.standard1.xlarge
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Information about the amount of storage allocated to the Dev Environment.
        #[builder(into)]
        pub persistent_storage: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::codecatalyst::DevEnvironmentPersistentStorage,
        >,
        /// The name of the project in the space.
        #[builder(into)]
        pub project_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The source repository that contains the branch to clone into the Dev Environment.
        #[builder(into, default)]
        pub repositories: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::codecatalyst::DevEnvironmentRepository>>,
        >,
        /// The name of the space.
        #[builder(into)]
        pub space_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DevEnvironmentResult {
        pub alias: pulumi_gestalt_rust::Output<Option<String>>,
        /// Information about the integrated development environment (IDE) configured for a Dev Environment.
        pub ides: pulumi_gestalt_rust::Output<
            super::super::types::codecatalyst::DevEnvironmentIdes,
        >,
        /// The amount of time the Dev Environment will run without any activity detected before stopping, in minutes. Only whole integers are allowed. Dev Environments consume compute minutes when running.
        pub inactivity_timeout_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The Amazon EC2 instace type to use for the Dev Environment. Valid values include dev.standard1.small,dev.standard1.medium,dev.standard1.large,dev.standard1.xlarge
        ///
        /// The following arguments are optional:
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// Information about the amount of storage allocated to the Dev Environment.
        pub persistent_storage: pulumi_gestalt_rust::Output<
            super::super::types::codecatalyst::DevEnvironmentPersistentStorage,
        >,
        /// The name of the project in the space.
        pub project_name: pulumi_gestalt_rust::Output<String>,
        /// The source repository that contains the branch to clone into the Dev Environment.
        pub repositories: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::codecatalyst::DevEnvironmentRepository>>,
        >,
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
        args: DevEnvironmentArgs,
    ) -> DevEnvironmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alias_binding = args.alias.get_output(context);
        let ides_binding = args.ides.get_output(context);
        let inactivity_timeout_minutes_binding = args
            .inactivity_timeout_minutes
            .get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let persistent_storage_binding = args.persistent_storage.get_output(context);
        let project_name_binding = args.project_name.get_output(context);
        let repositories_binding = args.repositories.get_output(context);
        let space_name_binding = args.space_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codecatalyst/devEnvironment:DevEnvironment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alias".into(),
                    value: alias_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ides".into(),
                    value: ides_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inactivityTimeoutMinutes".into(),
                    value: inactivity_timeout_minutes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: instance_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "persistentStorage".into(),
                    value: persistent_storage_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectName".into(),
                    value: project_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositories".into(),
                    value: repositories_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spaceName".into(),
                    value: space_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DevEnvironmentResult {
            alias: o.get_field("alias"),
            ides: o.get_field("ides"),
            inactivity_timeout_minutes: o.get_field("inactivityTimeoutMinutes"),
            instance_type: o.get_field("instanceType"),
            persistent_storage: o.get_field("persistentStorage"),
            project_name: o.get_field("projectName"),
            repositories: o.get_field("repositories"),
            space_name: o.get_field("spaceName"),
        }
    }
}
