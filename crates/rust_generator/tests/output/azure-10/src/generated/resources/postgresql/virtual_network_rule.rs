/// Manages a PostgreSQL Virtual Network Rule.
///
/// > **NOTE:** PostgreSQL Virtual Network Rules [can only be used with SKU Tiers of `GeneralPurpose` or `MemoryOptimized`](https://docs.microsoft.com/azure/postgresql/concepts-data-access-and-security-vnet)
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
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("psqladmin")
///             .administrator_login_password("H@Sh1CoR3!")
///             .backup_retention_days(7)
///             .location("${example.location}")
///             .name("postgresql-server-1")
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Gen5_2")
///             .ssl_enforcement_enabled(true)
///             .storage_mb(5120)
///             .version("9.5")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.7.29.0/29",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetworkRule = virtual_network_rule::create(
///         "exampleVirtualNetworkRule",
///         VirtualNetworkRuleArgs::builder()
///             .ignore_missing_vnet_service_endpoint(true)
///             .name("postgresql-vnet-rule")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleServer.name}")
///             .subnet_id("${internal.id}")
///             .build_struct(),
///     );
///     let internal = subnet::create(
///         "internal",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.7.29.0/29",])
///             .name("internal")
///             .resource_group_name("${example.name}")
///             .service_endpoints(vec!["Microsoft.Sql",])
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PostgreSQL Virtual Network Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/virtualNetworkRule:VirtualNetworkRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.DBforPostgreSQL/servers/myserver/virtualNetworkRules/vnetrulename
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_network_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkRuleArgs {
        /// Should the Virtual Network Rule be created before the Subnet has the Virtual Network Service Endpoint enabled?
        #[builder(into, default)]
        pub ignore_missing_vnet_service_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the PostgreSQL virtual network rule. Cannot be empty and must only contain alphanumeric characters and hyphens. Cannot start with a number, and cannot start or end with a hyphen. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `name` must be between 1-128 characters long and must satisfy all of the requirements below:
        ///
        /// 1. Contains only alphanumeric and hyphen characters
        /// 2. Cannot start with a number or hyphen
        /// 3. Cannot end with a hyphen
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group where the PostgreSQL server resides. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the SQL Server to which this PostgreSQL virtual network rule will be applied to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the subnet that the PostgreSQL server will be connected to.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkRuleResult {
        /// Should the Virtual Network Rule be created before the Subnet has the Virtual Network Service Endpoint enabled?
        pub ignore_missing_vnet_service_endpoint: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The name of the PostgreSQL virtual network rule. Cannot be empty and must only contain alphanumeric characters and hyphens. Cannot start with a number, and cannot start or end with a hyphen. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `name` must be between 1-128 characters long and must satisfy all of the requirements below:
        ///
        /// 1. Contains only alphanumeric and hyphen characters
        /// 2. Cannot start with a number or hyphen
        /// 3. Cannot end with a hyphen
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group where the PostgreSQL server resides. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the SQL Server to which this PostgreSQL virtual network rule will be applied to. Changing this forces a new resource to be created.
        pub server_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the subnet that the PostgreSQL server will be connected to.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualNetworkRuleArgs,
    ) -> VirtualNetworkRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ignore_missing_vnet_service_endpoint_binding = args
            .ignore_missing_vnet_service_endpoint
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let server_name_binding = args.server_name.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:postgresql/virtualNetworkRule:VirtualNetworkRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreMissingVnetServiceEndpoint".into(),
                    value: ignore_missing_vnet_service_endpoint_binding.get_id(),
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
                    name: "serverName".into(),
                    value: server_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualNetworkRuleResult {
            ignore_missing_vnet_service_endpoint: o
                .get_field("ignoreMissingVnetServiceEndpoint"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            server_name: o.get_field("serverName"),
            subnet_id: o.get_field("subnetId"),
        }
    }
}
