/// Manages a Container Connected Registry.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connected_registry {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectedRegistryArgs {
        /// Should the log auditing be enabled?
        #[builder(into, default)]
        pub audit_log_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies a list of IDs of Container Registry Tokens, which are meant to be used by the clients to connect to the Connected Registry.
        #[builder(into, default)]
        pub client_token_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the Container Registry that this Connected Registry will reside in. Changing this forces a new Container Connected Registry to be created.
        ///
        /// > If `parent_registry_id` is not specified, the Connected Registry will be connected to the Container Registry identified by `container_registry_id`.
        #[builder(into)]
        pub container_registry_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The verbosity of the logs. Possible values are `None`, `Debug`, `Information`, `Warning` and `Error`. Defaults to `None`.
        #[builder(into, default)]
        pub log_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The mode of the Connected Registry. Possible values are `Mirror`, `ReadOnly`, `ReadWrite` and `Registry`. Changing this forces a new Container Connected Registry to be created. Defaults to `ReadWrite`.
        #[builder(into, default)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Container Connected Registry. Changing this forces a new Container Connected Registry to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `notification` blocks as defined below.
        #[builder(into, default)]
        pub notifications: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::containerservice::ConnectedRegistryNotification>,
            >,
        >,
        /// The ID of the parent registry. This can be either a Container Registry ID or a Connected Registry ID. Changing this forces a new Container Connected Registry to be created.
        #[builder(into, default)]
        pub parent_registry_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The period of time (in form of ISO8601) for which a message is available to sync before it is expired. Allowed range is from `P1D` to `P90D`. Defaults to `P1D`.
        #[builder(into, default)]
        pub sync_message_ttl: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The cron expression indicating the schedule that the Connected Registry will sync with its parent. Defaults to `* * * * *`.
        #[builder(into, default)]
        pub sync_schedule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Container Registry Token which is used for synchronizing the Connected Registry. Changing this forces a new Container Connected Registry to be created.
        #[builder(into)]
        pub sync_token_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time window (in form of ISO8601) during which sync is enabled for each schedule occurrence. Allowed range is from `PT3H` to `P7D`.
        #[builder(into, default)]
        pub sync_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConnectedRegistryResult {
        /// Should the log auditing be enabled?
        pub audit_log_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies a list of IDs of Container Registry Tokens, which are meant to be used by the clients to connect to the Connected Registry.
        pub client_token_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the Container Registry that this Connected Registry will reside in. Changing this forces a new Container Connected Registry to be created.
        ///
        /// > If `parent_registry_id` is not specified, the Connected Registry will be connected to the Container Registry identified by `container_registry_id`.
        pub container_registry_id: pulumi_gestalt_rust::Output<String>,
        /// The verbosity of the logs. Possible values are `None`, `Debug`, `Information`, `Warning` and `Error`. Defaults to `None`.
        pub log_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// The mode of the Connected Registry. Possible values are `Mirror`, `ReadOnly`, `ReadWrite` and `Registry`. Changing this forces a new Container Connected Registry to be created. Defaults to `ReadWrite`.
        pub mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Container Connected Registry. Changing this forces a new Container Connected Registry to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `notification` blocks as defined below.
        pub notifications: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::containerservice::ConnectedRegistryNotification>,
            >,
        >,
        /// The ID of the parent registry. This can be either a Container Registry ID or a Connected Registry ID. Changing this forces a new Container Connected Registry to be created.
        pub parent_registry_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The period of time (in form of ISO8601) for which a message is available to sync before it is expired. Allowed range is from `P1D` to `P90D`. Defaults to `P1D`.
        pub sync_message_ttl: pulumi_gestalt_rust::Output<Option<String>>,
        /// The cron expression indicating the schedule that the Connected Registry will sync with its parent. Defaults to `* * * * *`.
        pub sync_schedule: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Container Registry Token which is used for synchronizing the Connected Registry. Changing this forces a new Container Connected Registry to be created.
        pub sync_token_id: pulumi_gestalt_rust::Output<String>,
        /// The time window (in form of ISO8601) during which sync is enabled for each schedule occurrence. Allowed range is from `PT3H` to `P7D`.
        pub sync_window: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectedRegistryArgs,
    ) -> ConnectedRegistryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let audit_log_enabled_binding = args.audit_log_enabled.get_output(context);
        let client_token_ids_binding = args.client_token_ids.get_output(context);
        let container_registry_id_binding = args
            .container_registry_id
            .get_output(context);
        let log_level_binding = args.log_level.get_output(context);
        let mode_binding = args.mode.get_output(context);
        let name_binding = args.name.get_output(context);
        let notifications_binding = args.notifications.get_output(context);
        let parent_registry_id_binding = args.parent_registry_id.get_output(context);
        let sync_message_ttl_binding = args.sync_message_ttl.get_output(context);
        let sync_schedule_binding = args.sync_schedule.get_output(context);
        let sync_token_id_binding = args.sync_token_id.get_output(context);
        let sync_window_binding = args.sync_window.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/connectedRegistry:ConnectedRegistry".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "auditLogEnabled".into(),
                    value: audit_log_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientTokenIds".into(),
                    value: client_token_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistryId".into(),
                    value: container_registry_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logLevel".into(),
                    value: log_level_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notifications".into(),
                    value: notifications_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentRegistryId".into(),
                    value: parent_registry_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "syncMessageTtl".into(),
                    value: sync_message_ttl_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "syncSchedule".into(),
                    value: sync_schedule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "syncTokenId".into(),
                    value: sync_token_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "syncWindow".into(),
                    value: sync_window_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectedRegistryResult {
            audit_log_enabled: o.get_field("auditLogEnabled"),
            client_token_ids: o.get_field("clientTokenIds"),
            container_registry_id: o.get_field("containerRegistryId"),
            log_level: o.get_field("logLevel"),
            mode: o.get_field("mode"),
            name: o.get_field("name"),
            notifications: o.get_field("notifications"),
            parent_registry_id: o.get_field("parentRegistryId"),
            sync_message_ttl: o.get_field("syncMessageTtl"),
            sync_schedule: o.get_field("syncSchedule"),
            sync_token_id: o.get_field("syncTokenId"),
            sync_window: o.get_field("syncWindow"),
        }
    }
}
