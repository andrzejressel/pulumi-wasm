/// Runs a Container Registry Task Schedule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleRegistry = registry::create(
///         "exampleRegistry",
///         RegistryArgs::builder()
///             .location("${example.location}")
///             .name("example-acr")
///             .resource_group_name("${example.name}")
///             .sku("Basic")
///             .build_struct(),
///     );
///     let exampleRegistryTask = registry_task::create(
///         "exampleRegistryTask",
///         RegistryTaskArgs::builder()
///             .container_registry_id("${exampleRegistry.id}")
///             .docker_step(
///                 RegistryTaskDockerStep::builder()
///                     .contextAccessToken("<github personal access token>")
///                     .contextPath(
///                         "https://github.com/<user name>/acr-build-helloworld-node#main",
///                     )
///                     .dockerfilePath("Dockerfile")
///                     .imageNames(vec!["helloworld:{{.Run.ID}}",])
///                     .build_struct(),
///             )
///             .name("example-task")
///             .platform(RegistryTaskPlatform::builder().os("Linux").build_struct())
///             .build_struct(),
///     );
///     let exampleRegistryTaskScheduleRunNow = registry_task_schedule_run_now::create(
///         "exampleRegistryTaskScheduleRunNow",
///         RegistryTaskScheduleRunNowArgs::builder()
///             .container_registry_task_id("${exampleRegistryTask.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod registry_task_schedule_run_now {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryTaskScheduleRunNowArgs {
        /// The ID of the Container Registry Task that to be scheduled. Changing this forces a new Container Registry Task Schedule to be created.
        #[builder(into)]
        pub container_registry_task_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegistryTaskScheduleRunNowResult {
        /// The ID of the Container Registry Task that to be scheduled. Changing this forces a new Container Registry Task Schedule to be created.
        pub container_registry_task_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegistryTaskScheduleRunNowArgs,
    ) -> RegistryTaskScheduleRunNowResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_registry_task_id_binding = args
            .container_registry_task_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/registryTaskScheduleRunNow:RegistryTaskScheduleRunNow"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistryTaskId".into(),
                    value: container_registry_task_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegistryTaskScheduleRunNowResult {
            container_registry_task_id: o.get_field("containerRegistryTaskId"),
        }
    }
}
