/// Manages an Azure Container Registry scope map.  Scope Maps are a preview feature only available in Premium SKU Container registries.
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
///             .name("exampleregistry")
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
/// }
/// ```
///
/// ## Import
///
/// Container Registries can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/registryScopeMap:RegistryScopeMap example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ContainerRegistry/registries/myregistry1/scopeMaps/scopemap1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod registry_scope_map {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryScopeMapArgs {
        /// A list of actions to attach to the scope map (e.g. `repo/content/read`, `repo2/content/delete`).
        #[builder(into)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The name of the Container Registry. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_registry_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the Container Registry.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the scope map. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Container Registry token. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegistryScopeMapResult {
        /// A list of actions to attach to the scope map (e.g. `repo/content/read`, `repo2/content/delete`).
        pub actions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the Container Registry. Changing this forces a new resource to be created.
        pub container_registry_name: pulumi_gestalt_rust::Output<String>,
        /// The description of the Container Registry.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the scope map. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Container Registry token. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegistryScopeMapArgs,
    ) -> RegistryScopeMapResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let container_registry_name_binding = args
            .container_registry_name
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/registryScopeMap:RegistryScopeMap".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: actions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistryName".into(),
                    value: container_registry_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegistryScopeMapResult {
            actions: o.get_field("actions"),
            container_registry_name: o.get_field("containerRegistryName"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
