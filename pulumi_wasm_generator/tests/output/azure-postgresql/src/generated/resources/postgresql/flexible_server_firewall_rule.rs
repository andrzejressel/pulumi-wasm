/// Manages a PostgreSQL Flexible Server Firewall Rule.
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
///     let exampleFlexibleServer = flexible_server::create(
///         "exampleFlexibleServer",
///         FlexibleServerArgs::builder()
///             .administrator_login("psqladmin")
///             .administrator_password("H@Sh1CoR3!")
///             .location("${example.location}")
///             .name("example-psqlflexibleserver")
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Standard_D4s_v3")
///             .storage_mb(32768)
///             .version("12")
///             .build_struct(),
///     );
///     let exampleFlexibleServerFirewallRule = flexible_server_firewall_rule::create(
///         "exampleFlexibleServerFirewallRule",
///         FlexibleServerFirewallRuleArgs::builder()
///             .end_ip_address("122.122.0.0")
///             .name("example-fw")
///             .server_id("${exampleFlexibleServer.id}")
///             .start_ip_address("122.122.0.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PostgreSQL Flexible Server Firewall Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/flexibleServerFirewallRule:FlexibleServerFirewallRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DBforPostgreSQL/flexibleServers/flexibleServer1/firewallRules/firewallRule1
/// ```
///
pub mod flexible_server_firewall_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerFirewallRuleArgs {
        /// The End IP Address associated with this PostgreSQL Flexible Server Firewall Rule.
        #[builder(into)]
        pub end_ip_address: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this PostgreSQL Flexible Server Firewall Rule. Changing this forces a new PostgreSQL Flexible Server Firewall Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the PostgreSQL Flexible Server from which to create this PostgreSQL Flexible Server Firewall Rule. Changing this forces a new PostgreSQL Flexible Server Firewall Rule to be created.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// The Start IP Address associated with this PostgreSQL Flexible Server Firewall Rule.
        #[builder(into)]
        pub start_ip_address: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerFirewallRuleResult {
        /// The End IP Address associated with this PostgreSQL Flexible Server Firewall Rule.
        pub end_ip_address: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this PostgreSQL Flexible Server Firewall Rule. Changing this forces a new PostgreSQL Flexible Server Firewall Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the PostgreSQL Flexible Server from which to create this PostgreSQL Flexible Server Firewall Rule. Changing this forces a new PostgreSQL Flexible Server Firewall Rule to be created.
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// The Start IP Address associated with this PostgreSQL Flexible Server Firewall Rule.
        pub start_ip_address: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FlexibleServerFirewallRuleArgs,
    ) -> FlexibleServerFirewallRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let end_ip_address_binding = args.end_ip_address.get_inner();
        let name_binding = args.name.get_inner();
        let server_id_binding = args.server_id.get_inner();
        let start_ip_address_binding = args.start_ip_address.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/flexibleServerFirewallRule:FlexibleServerFirewallRule"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endIpAddress".into(),
                    value: &end_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
                register_interface::ObjectField {
                    name: "startIpAddress".into(),
                    value: &start_ip_address_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "endIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "serverId".into(),
                },
                register_interface::ResultField {
                    name: "startIpAddress".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FlexibleServerFirewallRuleResult {
            end_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endIpAddress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
            start_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startIpAddress").unwrap(),
            ),
        }
    }
}