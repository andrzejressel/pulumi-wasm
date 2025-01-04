/// Manages a Container Connected Registry.
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
///     let exampleConnectedRegistry = connected_registry::create(
///         "exampleConnectedRegistry",
///         ConnectedRegistryArgs::builder()
///             .container_registry_id("${exampleRegistry.id}")
///             .name("examplecr")
///             .sync_token_id("${exampleRegistryToken.id}")
///             .build_struct(),
///     );
///     let exampleRegistry = registry::create(
///         "exampleRegistry",
///         RegistryArgs::builder()
///             .data_endpoint_enabled(true)
///             .location("${example.location}")
///             .name("exampleacr")
///             .resource_group_name("${example.name}")
///             .sku("Premium")
///             .build_struct(),
///     );
///     let exampleRegistryScopeMap = registry_scope_map::create(
///         "exampleRegistryScopeMap",
///         RegistryScopeMapArgs::builder()
///             .actions(
///                 vec![
///                     "repositories/hello-world/content/delete",
///                     "repositories/hello-world/content/read",
///                     "repositories/hello-world/content/write",
///                     "repositories/hello-world/metadata/read",
///                     "repositories/hello-world/metadata/write",
///                     "gateway/examplecr/config/read", "gateway/examplecr/config/write",
///                     "gateway/examplecr/message/read", "gateway/examplecr/message/write",
///                 ],
///             )
///             .container_registry_name("${exampleRegistry.name}")
///             .name("examplescopemap")
///             .resource_group_name("${exampleRegistry.resourceGroupName}")
///             .build_struct(),
///     );
///     let exampleRegistryToken = registry_token::create(
///         "exampleRegistryToken",
///         RegistryTokenArgs::builder()
///             .container_registry_name("${exampleRegistry.name}")
///             .name("exampletoken")
///             .resource_group_name("${exampleRegistry.resourceGroupName}")
///             .scope_map_id("${exampleRegistryScopeMap.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Container Connected Registries can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/connectedRegistry:ConnectedRegistry example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.ContainerRegistry/registries/registry1/connectedRegistries/registry1
/// ```
///
pub mod connected_registry {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectedRegistryArgs {
        /// Should the log auditing be enabled?
        #[builder(into, default)]
        pub audit_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies a list of IDs of Container Registry Tokens, which are meant to be used by the clients to connect to the Connected Registry.
        #[builder(into, default)]
        pub client_token_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the Container Registry that this Connected Registry will reside in. Changing this forces a new Container Connected Registry to be created.
        ///
        /// > If `parent_registry_id` is not specified, the Connected Registry will be connected to the Container Registry identified by `container_registry_id`.
        #[builder(into)]
        pub container_registry_id: pulumi_wasm_rust::Output<String>,
        /// The verbosity of the logs. Possible values are `None`, `Debug`, `Information`, `Warning` and `Error`. Defaults to `None`.
        #[builder(into, default)]
        pub log_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The mode of the Connected Registry. Possible values are `Mirror`, `ReadOnly`, `ReadWrite` and `Registry`. Changing this forces a new Container Connected Registry to be created. Defaults to `ReadWrite`.
        #[builder(into, default)]
        pub mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Container Connected Registry. Changing this forces a new Container Connected Registry to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `notification` blocks as defined below.
        #[builder(into, default)]
        pub notifications: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::containerservice::ConnectedRegistryNotification>,
            >,
        >,
        /// The ID of the parent registry. This can be either a Container Registry ID or a Connected Registry ID. Changing this forces a new Container Connected Registry to be created.
        #[builder(into, default)]
        pub parent_registry_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The period of time (in form of ISO8601) for which a message is available to sync before it is expired. Allowed range is from `P1D` to `P90D`. Defaults to `P1D`.
        #[builder(into, default)]
        pub sync_message_ttl: pulumi_wasm_rust::Output<Option<String>>,
        /// The cron expression indicating the schedule that the Connected Registry will sync with its parent. Defaults to `* * * * *`.
        #[builder(into, default)]
        pub sync_schedule: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Container Registry Token which is used for synchronizing the Connected Registry. Changing this forces a new Container Connected Registry to be created.
        #[builder(into)]
        pub sync_token_id: pulumi_wasm_rust::Output<String>,
        /// The time window (in form of ISO8601) during which sync is enabled for each schedule occurrence. Allowed range is from `PT3H` to `P7D`.
        #[builder(into, default)]
        pub sync_window: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConnectedRegistryResult {
        /// Should the log auditing be enabled?
        pub audit_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies a list of IDs of Container Registry Tokens, which are meant to be used by the clients to connect to the Connected Registry.
        pub client_token_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ID of the Container Registry that this Connected Registry will reside in. Changing this forces a new Container Connected Registry to be created.
        ///
        /// > If `parent_registry_id` is not specified, the Connected Registry will be connected to the Container Registry identified by `container_registry_id`.
        pub container_registry_id: pulumi_wasm_rust::Output<String>,
        /// The verbosity of the logs. Possible values are `None`, `Debug`, `Information`, `Warning` and `Error`. Defaults to `None`.
        pub log_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The mode of the Connected Registry. Possible values are `Mirror`, `ReadOnly`, `ReadWrite` and `Registry`. Changing this forces a new Container Connected Registry to be created. Defaults to `ReadWrite`.
        pub mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Container Connected Registry. Changing this forces a new Container Connected Registry to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `notification` blocks as defined below.
        pub notifications: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::containerservice::ConnectedRegistryNotification>,
            >,
        >,
        /// The ID of the parent registry. This can be either a Container Registry ID or a Connected Registry ID. Changing this forces a new Container Connected Registry to be created.
        pub parent_registry_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The period of time (in form of ISO8601) for which a message is available to sync before it is expired. Allowed range is from `P1D` to `P90D`. Defaults to `P1D`.
        pub sync_message_ttl: pulumi_wasm_rust::Output<Option<String>>,
        /// The cron expression indicating the schedule that the Connected Registry will sync with its parent. Defaults to `* * * * *`.
        pub sync_schedule: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Container Registry Token which is used for synchronizing the Connected Registry. Changing this forces a new Container Connected Registry to be created.
        pub sync_token_id: pulumi_wasm_rust::Output<String>,
        /// The time window (in form of ISO8601) during which sync is enabled for each schedule occurrence. Allowed range is from `PT3H` to `P7D`.
        pub sync_window: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectedRegistryArgs) -> ConnectedRegistryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audit_log_enabled_binding = args.audit_log_enabled.get_inner();
        let client_token_ids_binding = args.client_token_ids.get_inner();
        let container_registry_id_binding = args.container_registry_id.get_inner();
        let log_level_binding = args.log_level.get_inner();
        let mode_binding = args.mode.get_inner();
        let name_binding = args.name.get_inner();
        let notifications_binding = args.notifications.get_inner();
        let parent_registry_id_binding = args.parent_registry_id.get_inner();
        let sync_message_ttl_binding = args.sync_message_ttl.get_inner();
        let sync_schedule_binding = args.sync_schedule.get_inner();
        let sync_token_id_binding = args.sync_token_id.get_inner();
        let sync_window_binding = args.sync_window.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/connectedRegistry:ConnectedRegistry".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "auditLogEnabled".into(),
                    value: &audit_log_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "clientTokenIds".into(),
                    value: &client_token_ids_binding,
                },
                register_interface::ObjectField {
                    name: "containerRegistryId".into(),
                    value: &container_registry_id_binding,
                },
                register_interface::ObjectField {
                    name: "logLevel".into(),
                    value: &log_level_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notifications".into(),
                    value: &notifications_binding,
                },
                register_interface::ObjectField {
                    name: "parentRegistryId".into(),
                    value: &parent_registry_id_binding,
                },
                register_interface::ObjectField {
                    name: "syncMessageTtl".into(),
                    value: &sync_message_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "syncSchedule".into(),
                    value: &sync_schedule_binding,
                },
                register_interface::ObjectField {
                    name: "syncTokenId".into(),
                    value: &sync_token_id_binding,
                },
                register_interface::ObjectField {
                    name: "syncWindow".into(),
                    value: &sync_window_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "auditLogEnabled".into(),
                },
                register_interface::ResultField {
                    name: "clientTokenIds".into(),
                },
                register_interface::ResultField {
                    name: "containerRegistryId".into(),
                },
                register_interface::ResultField {
                    name: "logLevel".into(),
                },
                register_interface::ResultField {
                    name: "mode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notifications".into(),
                },
                register_interface::ResultField {
                    name: "parentRegistryId".into(),
                },
                register_interface::ResultField {
                    name: "syncMessageTtl".into(),
                },
                register_interface::ResultField {
                    name: "syncSchedule".into(),
                },
                register_interface::ResultField {
                    name: "syncTokenId".into(),
                },
                register_interface::ResultField {
                    name: "syncWindow".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectedRegistryResult {
            audit_log_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("auditLogEnabled").unwrap(),
            ),
            client_token_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientTokenIds").unwrap(),
            ),
            container_registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerRegistryId").unwrap(),
            ),
            log_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logLevel").unwrap(),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notifications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notifications").unwrap(),
            ),
            parent_registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentRegistryId").unwrap(),
            ),
            sync_message_ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("syncMessageTtl").unwrap(),
            ),
            sync_schedule: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("syncSchedule").unwrap(),
            ),
            sync_token_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("syncTokenId").unwrap(),
            ),
            sync_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("syncWindow").unwrap(),
            ),
        }
    }
}
