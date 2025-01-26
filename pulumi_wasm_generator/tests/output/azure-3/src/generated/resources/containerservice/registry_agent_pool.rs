/// Manages an Azure Container Registry Agent Pool.
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
///             .location("West Europ")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleRegistry = registry::create(
///         "exampleRegistry",
///         RegistryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .sku("Premium")
///             .build_struct(),
///     );
///     let exampleRegistryAgentPool = registry_agent_pool::create(
///         "exampleRegistryAgentPool",
///         RegistryAgentPoolArgs::builder()
///             .container_registry_name("${exampleRegistry.name}")
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure Container Registry Agent Pool can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/registryAgentPool:RegistryAgentPool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.ContainerRegistry/registries/registry1/agentPools/agentpool1
/// ```
///
pub mod registry_agent_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryAgentPoolArgs {
        /// Name of Azure Container Registry to create an Agent Pool for. Changing this forces a new Azure Container Registry Agent Pool to be created.
        #[builder(into)]
        pub container_registry_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// VMSS instance count. Defaults to `1`.
        #[builder(into, default)]
        pub instance_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The Azure Region where the Azure Container Registry Agent Pool should exist. Changing this forces a new Azure Container Registry Agent Pool to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Azure Container Registry Agent Pool. Changing this forces a new Azure Container Registry Agent Pool to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Azure Container Registry Agent Pool should exist. Changing this forces a new Azure Container Registry Agent Pool to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Azure Container Registry Agent Pool.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Sets the VM your agent pool will run on. Valid values are: `S1` (2 vCPUs, 3 GiB RAM), `S2` (4 vCPUs, 8 GiB RAM), `S3` (8 vCPUs, 16 GiB RAM) or `I6` (64 vCPUs, 216 GiB RAM, Isolated). Defaults to `S1`. Changing this forces a new Azure Container Registry Agent Pool to be created.
        #[builder(into, default)]
        pub tier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Virtual Network Subnet Resource where the agent machines will be running. Changing this forces a new Azure Container Registry Agent Pool to be created.
        #[builder(into, default)]
        pub virtual_network_subnet_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegistryAgentPoolResult {
        /// Name of Azure Container Registry to create an Agent Pool for. Changing this forces a new Azure Container Registry Agent Pool to be created.
        pub container_registry_name: pulumi_wasm_rust::Output<String>,
        /// VMSS instance count. Defaults to `1`.
        pub instance_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Azure Region where the Azure Container Registry Agent Pool should exist. Changing this forces a new Azure Container Registry Agent Pool to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Azure Container Registry Agent Pool. Changing this forces a new Azure Container Registry Agent Pool to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Azure Container Registry Agent Pool should exist. Changing this forces a new Azure Container Registry Agent Pool to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Azure Container Registry Agent Pool.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Sets the VM your agent pool will run on. Valid values are: `S1` (2 vCPUs, 3 GiB RAM), `S2` (4 vCPUs, 8 GiB RAM), `S3` (8 vCPUs, 16 GiB RAM) or `I6` (64 vCPUs, 216 GiB RAM, Isolated). Defaults to `S1`. Changing this forces a new Azure Container Registry Agent Pool to be created.
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Virtual Network Subnet Resource where the agent machines will be running. Changing this forces a new Azure Container Registry Agent Pool to be created.
        pub virtual_network_subnet_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegistryAgentPoolArgs,
    ) -> RegistryAgentPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_registry_name_binding = args
            .container_registry_name
            .get_output(context)
            .get_inner();
        let instance_count_binding = args.instance_count.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tier_binding = args.tier.get_output(context).get_inner();
        let virtual_network_subnet_id_binding = args
            .virtual_network_subnet_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/registryAgentPool:RegistryAgentPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerRegistryName".into(),
                    value: &container_registry_name_binding,
                },
                register_interface::ObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkSubnetId".into(),
                    value: &virtual_network_subnet_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegistryAgentPoolResult {
            container_registry_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerRegistryName"),
            ),
            instance_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceCount"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tier: pulumi_wasm_rust::__private::into_domain(o.extract_field("tier")),
            virtual_network_subnet_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualNetworkSubnetId"),
            ),
        }
    }
}
