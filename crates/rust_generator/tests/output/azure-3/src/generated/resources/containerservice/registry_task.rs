/// Manages a Container Registry Task.
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
///             .name("example")
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
///                         "https://github.com/<username>/<repository>#<branch>:<folder>",
///                     )
///                     .dockerfilePath("Dockerfile")
///                     .imageNames(vec!["helloworld:{{.Run.ID}}",])
///                     .build_struct(),
///             )
///             .name("example-task")
///             .platform(RegistryTaskPlatform::builder().os("Linux").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Container Registry Tasks can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/registryTask:RegistryTask example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.ContainerRegistry/registries/registry1/tasks/task1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod registry_task {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryTaskArgs {
        /// The name of the dedicated Container Registry Agent Pool for this Container Registry Task.
        #[builder(into, default)]
        pub agent_pool_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `agent_setting` block as defined below.
        ///
        /// > **NOTE:** Only one of `agent_pool_name` and `agent_setting` can be specified.
        #[builder(into, default)]
        pub agent_setting: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskAgentSetting>,
        >,
        /// A `base_image_trigger` block as defined below.
        #[builder(into, default)]
        pub base_image_trigger: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskBaseImageTrigger>,
        >,
        /// The ID of the Container Registry that this Container Registry Task resides in. Changing this forces a new Container Registry Task to be created.
        #[builder(into)]
        pub container_registry_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `docker_step` block as defined below.
        #[builder(into, default)]
        pub docker_step: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskDockerStep>,
        >,
        /// Should this Container Registry Task be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `encoded_step` block as defined below.
        #[builder(into, default)]
        pub encoded_step: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskEncodedStep>,
        >,
        /// A `file_step` block as defined below.
        ///
        /// > **NOTE:** For non-system task (when `is_system_task` is set to `false`), one and only one of the `docker_step`, `encoded_step` and `file_step` should be specified.
        #[builder(into, default)]
        pub file_step: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskFileStep>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskIdentity>,
        >,
        /// Whether this Container Registry Task is a system task. Changing this forces a new Container Registry Task to be created. Defaults to `false`.
        #[builder(into, default)]
        pub is_system_task: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub log_template: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Container Registry Task. Changing this forces a new Container Registry Task to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `platform` block as defined below.
        ///
        /// > **NOTE:** The `platform` is required for non-system task (when `is_system_task` is set to `false`).
        #[builder(into, default)]
        pub platform: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskPlatform>,
        >,
        #[builder(into, default)]
        pub registry_credential: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskRegistryCredential>,
        >,
        /// One or more `source_trigger` blocks as defined below.
        #[builder(into, default)]
        pub source_triggers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::containerservice::RegistryTaskSourceTrigger>>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeout_in_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// One or more `timer_trigger` blocks as defined below.
        #[builder(into, default)]
        pub timer_triggers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::containerservice::RegistryTaskTimerTrigger>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegistryTaskResult {
        /// The name of the dedicated Container Registry Agent Pool for this Container Registry Task.
        pub agent_pool_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `agent_setting` block as defined below.
        ///
        /// > **NOTE:** Only one of `agent_pool_name` and `agent_setting` can be specified.
        pub agent_setting: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskAgentSetting>,
        >,
        /// A `base_image_trigger` block as defined below.
        pub base_image_trigger: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskBaseImageTrigger>,
        >,
        /// The ID of the Container Registry that this Container Registry Task resides in. Changing this forces a new Container Registry Task to be created.
        pub container_registry_id: pulumi_gestalt_rust::Output<String>,
        /// A `docker_step` block as defined below.
        pub docker_step: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskDockerStep>,
        >,
        /// Should this Container Registry Task be enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `encoded_step` block as defined below.
        pub encoded_step: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskEncodedStep>,
        >,
        /// A `file_step` block as defined below.
        ///
        /// > **NOTE:** For non-system task (when `is_system_task` is set to `false`), one and only one of the `docker_step`, `encoded_step` and `file_step` should be specified.
        pub file_step: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskFileStep>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskIdentity>,
        >,
        /// Whether this Container Registry Task is a system task. Changing this forces a new Container Registry Task to be created. Defaults to `false`.
        pub is_system_task: pulumi_gestalt_rust::Output<Option<bool>>,
        pub log_template: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Container Registry Task. Changing this forces a new Container Registry Task to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `platform` block as defined below.
        ///
        /// > **NOTE:** The `platform` is required for non-system task (when `is_system_task` is set to `false`).
        pub platform: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskPlatform>,
        >,
        pub registry_credential: pulumi_gestalt_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskRegistryCredential>,
        >,
        /// One or more `source_trigger` blocks as defined below.
        pub source_triggers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::containerservice::RegistryTaskSourceTrigger>>,
        >,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub timeout_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// One or more `timer_trigger` blocks as defined below.
        pub timer_triggers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::containerservice::RegistryTaskTimerTrigger>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegistryTaskArgs,
    ) -> RegistryTaskResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agent_pool_name_binding = args.agent_pool_name.get_output(context);
        let agent_setting_binding = args.agent_setting.get_output(context);
        let base_image_trigger_binding = args.base_image_trigger.get_output(context);
        let container_registry_id_binding = args
            .container_registry_id
            .get_output(context);
        let docker_step_binding = args.docker_step.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let encoded_step_binding = args.encoded_step.get_output(context);
        let file_step_binding = args.file_step.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let is_system_task_binding = args.is_system_task.get_output(context);
        let log_template_binding = args.log_template.get_output(context);
        let name_binding = args.name.get_output(context);
        let platform_binding = args.platform.get_output(context);
        let registry_credential_binding = args.registry_credential.get_output(context);
        let source_triggers_binding = args.source_triggers.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeout_in_seconds_binding = args.timeout_in_seconds.get_output(context);
        let timer_triggers_binding = args.timer_triggers.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/registryTask:RegistryTask".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentPoolName".into(),
                    value: agent_pool_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentSetting".into(),
                    value: agent_setting_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseImageTrigger".into(),
                    value: base_image_trigger_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistryId".into(),
                    value: container_registry_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dockerStep".into(),
                    value: docker_step_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encodedStep".into(),
                    value: encoded_step_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileStep".into(),
                    value: file_step_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isSystemTask".into(),
                    value: is_system_task_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logTemplate".into(),
                    value: log_template_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platform".into(),
                    value: platform_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registryCredential".into(),
                    value: registry_credential_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceTriggers".into(),
                    value: source_triggers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeoutInSeconds".into(),
                    value: timeout_in_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timerTriggers".into(),
                    value: timer_triggers_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegistryTaskResult {
            agent_pool_name: o.get_field("agentPoolName"),
            agent_setting: o.get_field("agentSetting"),
            base_image_trigger: o.get_field("baseImageTrigger"),
            container_registry_id: o.get_field("containerRegistryId"),
            docker_step: o.get_field("dockerStep"),
            enabled: o.get_field("enabled"),
            encoded_step: o.get_field("encodedStep"),
            file_step: o.get_field("fileStep"),
            identity: o.get_field("identity"),
            is_system_task: o.get_field("isSystemTask"),
            log_template: o.get_field("logTemplate"),
            name: o.get_field("name"),
            platform: o.get_field("platform"),
            registry_credential: o.get_field("registryCredential"),
            source_triggers: o.get_field("sourceTriggers"),
            tags: o.get_field("tags"),
            timeout_in_seconds: o.get_field("timeoutInSeconds"),
            timer_triggers: o.get_field("timerTriggers"),
        }
    }
}
