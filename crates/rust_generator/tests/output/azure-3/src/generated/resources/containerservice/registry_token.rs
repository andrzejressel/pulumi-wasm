/// Manages an Azure Container Registry token. Tokens are a preview feature only available in Premium SKU Container registries.
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
///             .name("example-resource-group")
///             .build_struct(),
///     );
///     let exampleRegistry = registry::create(
///         "exampleRegistry",
///         RegistryArgs::builder()
///             .admin_enabled(false)
///             .georeplications(
///                 vec![
///                     RegistryGeoreplication::builder().location("East US").build_struct(),
///                     RegistryGeoreplication::builder().location("West Europe")
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .sku("Premium")
///             .build_struct(),
///     );
///     let exampleRegistryScopeMap = registry_scope_map::create(
///         "exampleRegistryScopeMap",
///         RegistryScopeMapArgs::builder()
///             .actions(
///                 vec![
///                     "repositories/repo1/content/read",
///                     "repositories/repo1/content/write",
///                 ],
///             )
///             .container_registry_name("${exampleRegistry.name}")
///             .name("example-scope-map")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleRegistryToken = registry_token::create(
///         "exampleRegistryToken",
///         RegistryTokenArgs::builder()
///             .container_registry_name("${exampleRegistry.name}")
///             .name("exampletoken")
///             .resource_group_name("${example.name}")
///             .scope_map_id("${exampleRegistryScopeMap.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Container Registries can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/registryToken:RegistryToken example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ContainerRegistry/registries/myregistry1/tokens/token1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod registry_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryTokenArgs {
        /// The name of the Container Registry. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_registry_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Should the Container Registry token be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the token. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Container Registry token. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Container Registry Scope Map associated with the token.
        #[builder(into)]
        pub scope_map_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegistryTokenResult {
        /// The name of the Container Registry. Changing this forces a new resource to be created.
        pub container_registry_name: pulumi_gestalt_rust::Output<String>,
        /// Should the Container Registry token be enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the token. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Container Registry token. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Container Registry Scope Map associated with the token.
        pub scope_map_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegistryTokenArgs,
    ) -> RegistryTokenResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_registry_name_binding = args
            .container_registry_name
            .get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let scope_map_id_binding = args.scope_map_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/registryToken:RegistryToken".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistryName".into(),
                    value: container_registry_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeMapId".into(),
                    value: scope_map_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegistryTokenResult {
            container_registry_name: o.get_field("containerRegistryName"),
            enabled: o.get_field("enabled"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            scope_map_id: o.get_field("scopeMapId"),
        }
    }
}
