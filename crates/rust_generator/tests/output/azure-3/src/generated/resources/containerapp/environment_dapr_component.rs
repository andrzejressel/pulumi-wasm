/// Manages a Dapr Component for a Container App Environment.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("acctest-01")
///             .resource_group_name("${example.name}")
///             .retention_in_days(30)
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleEnvironment = environment::create(
///         "exampleEnvironment",
///         EnvironmentArgs::builder()
///             .location("${example.location}")
///             .log_analytics_workspace_id("${exampleAnalyticsWorkspace.id}")
///             .name("Example-Environment")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleEnvironmentDaprComponent = environment_dapr_component::create(
///         "exampleEnvironmentDaprComponent",
///         EnvironmentDaprComponentArgs::builder()
///             .component_type("state.azure.blobstorage")
///             .container_app_environment_id("${exampleEnvironment.id}")
///             .name("example-component")
///             .version("v1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// A Dapr Component for a Container App Environment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerapp/environmentDaprComponent:EnvironmentDaprComponent example "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.App/managedEnvironments/myenv/daprComponents/mydaprcomponent"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment_dapr_component {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentDaprComponentArgs {
        /// The Dapr Component Type. For example `state.azure.blobstorage`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub component_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Container App Managed Environment for this Dapr Component. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should the Dapr sidecar to continue initialisation if the component fails to load. Defaults to `false`
        #[builder(into, default)]
        pub ignore_errors: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The timeout for component initialisation as a `ISO8601` formatted string. e.g. `5s`, `2h`, `1m`. Defaults to `5s`.
        #[builder(into, default)]
        pub init_timeout: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `metadata` blocks as detailed below.
        #[builder(into, default)]
        pub metadatas: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::containerapp::EnvironmentDaprComponentMetadata>,
            >,
        >,
        /// The name for this Dapr component. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of scopes to which this component applies.
        ///
        /// > **NOTE:** See the official docs for more information at https://learn.microsoft.com/en-us/azure/container-apps/dapr-overview?tabs=bicep1%2Cyaml#component-scopes
        #[builder(into, default)]
        pub scopes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A `secret` block as detailed below.
        #[builder(into, default)]
        pub secrets: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::containerapp::EnvironmentDaprComponentSecret>,
            >,
        >,
        /// The version of the component.
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentDaprComponentResult {
        /// The Dapr Component Type. For example `state.azure.blobstorage`. Changing this forces a new resource to be created.
        pub component_type: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Container App Managed Environment for this Dapr Component. Changing this forces a new resource to be created.
        pub container_app_environment_id: pulumi_gestalt_rust::Output<String>,
        /// Should the Dapr sidecar to continue initialisation if the component fails to load. Defaults to `false`
        pub ignore_errors: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The timeout for component initialisation as a `ISO8601` formatted string. e.g. `5s`, `2h`, `1m`. Defaults to `5s`.
        pub init_timeout: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `metadata` blocks as detailed below.
        pub metadatas: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::containerapp::EnvironmentDaprComponentMetadata>,
            >,
        >,
        /// The name for this Dapr component. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of scopes to which this component applies.
        ///
        /// > **NOTE:** See the official docs for more information at https://learn.microsoft.com/en-us/azure/container-apps/dapr-overview?tabs=bicep1%2Cyaml#component-scopes
        pub scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A `secret` block as detailed below.
        pub secrets: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::containerapp::EnvironmentDaprComponentSecret>,
            >,
        >,
        /// The version of the component.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentDaprComponentArgs,
    ) -> EnvironmentDaprComponentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let component_type_binding = args.component_type.get_output(context);
        let container_app_environment_id_binding = args
            .container_app_environment_id
            .get_output(context);
        let ignore_errors_binding = args.ignore_errors.get_output(context);
        let init_timeout_binding = args.init_timeout.get_output(context);
        let metadatas_binding = args.metadatas.get_output(context);
        let name_binding = args.name.get_output(context);
        let scopes_binding = args.scopes.get_output(context);
        let secrets_binding = args.secrets.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerapp/environmentDaprComponent:EnvironmentDaprComponent"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "componentType".into(),
                    value: &component_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerAppEnvironmentId".into(),
                    value: &container_app_environment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreErrors".into(),
                    value: &ignore_errors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initTimeout".into(),
                    value: &init_timeout_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadatas".into(),
                    value: &metadatas_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentDaprComponentResult {
            component_type: o.get_field("componentType"),
            container_app_environment_id: o.get_field("containerAppEnvironmentId"),
            ignore_errors: o.get_field("ignoreErrors"),
            init_timeout: o.get_field("initTimeout"),
            metadatas: o.get_field("metadatas"),
            name: o.get_field("name"),
            scopes: o.get_field("scopes"),
            secrets: o.get_field("secrets"),
            version: o.get_field("version"),
        }
    }
}
