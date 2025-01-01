/// Manages a Dapr Component for a Container App Environment.
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
pub mod environment_dapr_component {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentDaprComponentArgs {
        /// The Dapr Component Type. For example `state.azure.blobstorage`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub component_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the Container App Managed Environment for this Dapr Component. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_environment_id: pulumi_wasm_rust::Output<String>,
        /// Should the Dapr sidecar to continue initialisation if the component fails to load. Defaults to `false`
        #[builder(into, default)]
        pub ignore_errors: pulumi_wasm_rust::Output<Option<bool>>,
        /// The timeout for component initialisation as a `ISO8601` formatted string. e.g. `5s`, `2h`, `1m`. Defaults to `5s`.
        #[builder(into, default)]
        pub init_timeout: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `metadata` blocks as detailed below.
        #[builder(into, default)]
        pub metadatas: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::containerapp::EnvironmentDaprComponentMetadata>,
            >,
        >,
        /// The name for this Dapr component. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of scopes to which this component applies.
        ///
        /// > **NOTE:** See the official docs for more information at https://learn.microsoft.com/en-us/azure/container-apps/dapr-overview?tabs=bicep1%2Cyaml#component-scopes
        #[builder(into, default)]
        pub scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `secret` block as detailed below.
        #[builder(into, default)]
        pub secrets: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::containerapp::EnvironmentDaprComponentSecret>,
            >,
        >,
        /// The version of the component.
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentDaprComponentResult {
        /// The Dapr Component Type. For example `state.azure.blobstorage`. Changing this forces a new resource to be created.
        pub component_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the Container App Managed Environment for this Dapr Component. Changing this forces a new resource to be created.
        pub container_app_environment_id: pulumi_wasm_rust::Output<String>,
        /// Should the Dapr sidecar to continue initialisation if the component fails to load. Defaults to `false`
        pub ignore_errors: pulumi_wasm_rust::Output<Option<bool>>,
        /// The timeout for component initialisation as a `ISO8601` formatted string. e.g. `5s`, `2h`, `1m`. Defaults to `5s`.
        pub init_timeout: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `metadata` blocks as detailed below.
        pub metadatas: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::containerapp::EnvironmentDaprComponentMetadata>,
            >,
        >,
        /// The name for this Dapr component. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of scopes to which this component applies.
        ///
        /// > **NOTE:** See the official docs for more information at https://learn.microsoft.com/en-us/azure/container-apps/dapr-overview?tabs=bicep1%2Cyaml#component-scopes
        pub scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `secret` block as detailed below.
        pub secrets: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::containerapp::EnvironmentDaprComponentSecret>,
            >,
        >,
        /// The version of the component.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: EnvironmentDaprComponentArgs,
    ) -> EnvironmentDaprComponentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let component_type_binding = args.component_type.get_inner();
        let container_app_environment_id_binding = args
            .container_app_environment_id
            .get_inner();
        let ignore_errors_binding = args.ignore_errors.get_inner();
        let init_timeout_binding = args.init_timeout.get_inner();
        let metadatas_binding = args.metadatas.get_inner();
        let name_binding = args.name.get_inner();
        let scopes_binding = args.scopes.get_inner();
        let secrets_binding = args.secrets.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerapp/environmentDaprComponent:EnvironmentDaprComponent"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "componentType".into(),
                    value: &component_type_binding,
                },
                register_interface::ObjectField {
                    name: "containerAppEnvironmentId".into(),
                    value: &container_app_environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "ignoreErrors".into(),
                    value: &ignore_errors_binding,
                },
                register_interface::ObjectField {
                    name: "initTimeout".into(),
                    value: &init_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "metadatas".into(),
                    value: &metadatas_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scopes".into(),
                    value: &scopes_binding,
                },
                register_interface::ObjectField {
                    name: "secrets".into(),
                    value: &secrets_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "componentType".into(),
                },
                register_interface::ResultField {
                    name: "containerAppEnvironmentId".into(),
                },
                register_interface::ResultField {
                    name: "ignoreErrors".into(),
                },
                register_interface::ResultField {
                    name: "initTimeout".into(),
                },
                register_interface::ResultField {
                    name: "metadatas".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "scopes".into(),
                },
                register_interface::ResultField {
                    name: "secrets".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnvironmentDaprComponentResult {
            component_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("componentType").unwrap(),
            ),
            container_app_environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerAppEnvironmentId").unwrap(),
            ),
            ignore_errors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignoreErrors").unwrap(),
            ),
            init_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("initTimeout").unwrap(),
            ),
            metadatas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadatas").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopes").unwrap(),
            ),
            secrets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secrets").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
