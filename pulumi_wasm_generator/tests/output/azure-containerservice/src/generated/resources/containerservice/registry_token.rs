/// Manages an Azure Container Registry token. Tokens are a preview feature only available in Premium SKU Container registries.
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
pub mod registry_token {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryTokenArgs {
        /// The name of the Container Registry. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_registry_name: pulumi_wasm_rust::Output<String>,
        /// Should the Container Registry token be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the token. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Container Registry token. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Container Registry Scope Map associated with the token.
        #[builder(into)]
        pub scope_map_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct RegistryTokenResult {
        /// The name of the Container Registry. Changing this forces a new resource to be created.
        pub container_registry_name: pulumi_wasm_rust::Output<String>,
        /// Should the Container Registry token be enabled? Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the token. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Container Registry token. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Container Registry Scope Map associated with the token.
        pub scope_map_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RegistryTokenArgs) -> RegistryTokenResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_registry_name_binding = args.container_registry_name.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let scope_map_id_binding = args.scope_map_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/registryToken:RegistryToken".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerRegistryName".into(),
                    value: &container_registry_name_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scopeMapId".into(),
                    value: &scope_map_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerRegistryName".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scopeMapId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegistryTokenResult {
            container_registry_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerRegistryName").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scope_map_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopeMapId").unwrap(),
            ),
        }
    }
}
