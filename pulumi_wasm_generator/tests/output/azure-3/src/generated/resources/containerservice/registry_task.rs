/// Manages a Container Registry Task.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod registry_task {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryTaskArgs {
        /// The name of the dedicated Container Registry Agent Pool for this Container Registry Task.
        #[builder(into, default)]
        pub agent_pool_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `agent_setting` block as defined below.
        ///
        /// > **NOTE:** Only one of `agent_pool_name` and `agent_setting` can be specified.
        #[builder(into, default)]
        pub agent_setting: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskAgentSetting>,
        >,
        /// A `base_image_trigger` block as defined below.
        #[builder(into, default)]
        pub base_image_trigger: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskBaseImageTrigger>,
        >,
        /// The ID of the Container Registry that this Container Registry Task resides in. Changing this forces a new Container Registry Task to be created.
        #[builder(into)]
        pub container_registry_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `docker_step` block as defined below.
        #[builder(into, default)]
        pub docker_step: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskDockerStep>,
        >,
        /// Should this Container Registry Task be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A `encoded_step` block as defined below.
        #[builder(into, default)]
        pub encoded_step: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskEncodedStep>,
        >,
        /// A `file_step` block as defined below.
        ///
        /// > **NOTE:** For non-system task (when `is_system_task` is set to `false`), one and only one of the `docker_step`, `encoded_step` and `file_step` should be specified.
        #[builder(into, default)]
        pub file_step: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskFileStep>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskIdentity>,
        >,
        /// Whether this Container Registry Task is a system task. Changing this forces a new Container Registry Task to be created. Defaults to `false`.
        #[builder(into, default)]
        pub is_system_task: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub log_template: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Container Registry Task. Changing this forces a new Container Registry Task to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `platform` block as defined below.
        ///
        /// > **NOTE:** The `platform` is required for non-system task (when `is_system_task` is set to `false`).
        #[builder(into, default)]
        pub platform: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskPlatform>,
        >,
        #[builder(into, default)]
        pub registry_credential: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryTaskRegistryCredential>,
        >,
        /// One or more `source_trigger` blocks as defined below.
        #[builder(into, default)]
        pub source_triggers: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::containerservice::RegistryTaskSourceTrigger>>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeout_in_seconds: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// One or more `timer_trigger` blocks as defined below.
        #[builder(into, default)]
        pub timer_triggers: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::containerservice::RegistryTaskTimerTrigger>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegistryTaskResult {
        /// The name of the dedicated Container Registry Agent Pool for this Container Registry Task.
        pub agent_pool_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `agent_setting` block as defined below.
        ///
        /// > **NOTE:** Only one of `agent_pool_name` and `agent_setting` can be specified.
        pub agent_setting: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskAgentSetting>,
        >,
        /// A `base_image_trigger` block as defined below.
        pub base_image_trigger: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskBaseImageTrigger>,
        >,
        /// The ID of the Container Registry that this Container Registry Task resides in. Changing this forces a new Container Registry Task to be created.
        pub container_registry_id: pulumi_wasm_rust::Output<String>,
        /// A `docker_step` block as defined below.
        pub docker_step: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskDockerStep>,
        >,
        /// Should this Container Registry Task be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `encoded_step` block as defined below.
        pub encoded_step: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskEncodedStep>,
        >,
        /// A `file_step` block as defined below.
        ///
        /// > **NOTE:** For non-system task (when `is_system_task` is set to `false`), one and only one of the `docker_step`, `encoded_step` and `file_step` should be specified.
        pub file_step: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskFileStep>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskIdentity>,
        >,
        /// Whether this Container Registry Task is a system task. Changing this forces a new Container Registry Task to be created. Defaults to `false`.
        pub is_system_task: pulumi_wasm_rust::Output<Option<bool>>,
        pub log_template: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Container Registry Task. Changing this forces a new Container Registry Task to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `platform` block as defined below.
        ///
        /// > **NOTE:** The `platform` is required for non-system task (when `is_system_task` is set to `false`).
        pub platform: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskPlatform>,
        >,
        pub registry_credential: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::RegistryTaskRegistryCredential>,
        >,
        /// One or more `source_trigger` blocks as defined below.
        pub source_triggers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerservice::RegistryTaskSourceTrigger>>,
        >,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub timeout_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// One or more `timer_trigger` blocks as defined below.
        pub timer_triggers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerservice::RegistryTaskTimerTrigger>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegistryTaskArgs,
    ) -> RegistryTaskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let agent_pool_name_binding = args
            .agent_pool_name
            .get_output(context)
            .get_inner();
        let agent_setting_binding = args.agent_setting.get_output(context).get_inner();
        let base_image_trigger_binding = args
            .base_image_trigger
            .get_output(context)
            .get_inner();
        let container_registry_id_binding = args
            .container_registry_id
            .get_output(context)
            .get_inner();
        let docker_step_binding = args.docker_step.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let encoded_step_binding = args.encoded_step.get_output(context).get_inner();
        let file_step_binding = args.file_step.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let is_system_task_binding = args.is_system_task.get_output(context).get_inner();
        let log_template_binding = args.log_template.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let platform_binding = args.platform.get_output(context).get_inner();
        let registry_credential_binding = args
            .registry_credential
            .get_output(context)
            .get_inner();
        let source_triggers_binding = args
            .source_triggers
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeout_in_seconds_binding = args
            .timeout_in_seconds
            .get_output(context)
            .get_inner();
        let timer_triggers_binding = args.timer_triggers.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/registryTask:RegistryTask".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentPoolName".into(),
                    value: &agent_pool_name_binding,
                },
                register_interface::ObjectField {
                    name: "agentSetting".into(),
                    value: &agent_setting_binding,
                },
                register_interface::ObjectField {
                    name: "baseImageTrigger".into(),
                    value: &base_image_trigger_binding,
                },
                register_interface::ObjectField {
                    name: "containerRegistryId".into(),
                    value: &container_registry_id_binding,
                },
                register_interface::ObjectField {
                    name: "dockerStep".into(),
                    value: &docker_step_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "encodedStep".into(),
                    value: &encoded_step_binding,
                },
                register_interface::ObjectField {
                    name: "fileStep".into(),
                    value: &file_step_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "isSystemTask".into(),
                    value: &is_system_task_binding,
                },
                register_interface::ObjectField {
                    name: "logTemplate".into(),
                    value: &log_template_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "platform".into(),
                    value: &platform_binding,
                },
                register_interface::ObjectField {
                    name: "registryCredential".into(),
                    value: &registry_credential_binding,
                },
                register_interface::ObjectField {
                    name: "sourceTriggers".into(),
                    value: &source_triggers_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeoutInSeconds".into(),
                    value: &timeout_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "timerTriggers".into(),
                    value: &timer_triggers_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegistryTaskResult {
            agent_pool_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentPoolName"),
            ),
            agent_setting: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentSetting"),
            ),
            base_image_trigger: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("baseImageTrigger"),
            ),
            container_registry_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerRegistryId"),
            ),
            docker_step: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dockerStep"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            encoded_step: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encodedStep"),
            ),
            file_step: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fileStep"),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            is_system_task: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isSystemTask"),
            ),
            log_template: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logTemplate"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            platform: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            registry_credential: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("registryCredential"),
            ),
            source_triggers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceTriggers"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            timeout_in_seconds: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeoutInSeconds"),
            ),
            timer_triggers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timerTriggers"),
            ),
        }
    }
}
