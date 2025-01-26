/// Manages a Firewall Rule for a PostgreSQL Server
///
/// ## Example Usage
///
/// ### Single IP Address)
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
///             .name("api-rg-pro")
///             .build_struct(),
///     );
///     let exampleFirewallRule = firewall_rule::create(
///         "exampleFirewallRule",
///         FirewallRuleArgs::builder()
///             .end_ip_address("40.112.8.12")
///             .name("office")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleServer.name}")
///             .start_ip_address("40.112.8.12")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .location("${example.location}")
///             .name("example-postgre-server")
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Gen5_2")
///             .ssl_enforcement_enabled(true)
///             .version("11")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### IP Range)
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
///             .name("api-rg-pro")
///             .build_struct(),
///     );
///     let exampleFirewallRule = firewall_rule::create(
///         "exampleFirewallRule",
///         FirewallRuleArgs::builder()
///             .end_ip_address("40.112.255.255")
///             .name("office")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleServer.name}")
///             .start_ip_address("40.112.0.0")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PostgreSQL Firewall Rule's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/firewallRule:FirewallRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.DBforPostgreSQL/servers/server1/firewallRules/rule1
/// ```
///
pub mod firewall_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallRuleArgs {
        /// Specifies the End IP Address associated with this Firewall Rule. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The Azure feature `Allow access to Azure services` can be enabled by setting `start_ip_address` and `end_ip_address` to `0.0.0.0` which ([is documented in the Azure API Docs](https://docs.microsoft.com/rest/api/sql/firewallrules/createorupdate)).
        #[builder(into)]
        pub end_ip_address: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the PostgreSQL Firewall Rule. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the PostgreSQL Server exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the PostgreSQL Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the Start IP Address associated with this Firewall Rule. Changing this forces a new resource to be created.
        #[builder(into)]
        pub start_ip_address: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallRuleResult {
        /// Specifies the End IP Address associated with this Firewall Rule. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The Azure feature `Allow access to Azure services` can be enabled by setting `start_ip_address` and `end_ip_address` to `0.0.0.0` which ([is documented in the Azure API Docs](https://docs.microsoft.com/rest/api/sql/firewallrules/createorupdate)).
        pub end_ip_address: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the PostgreSQL Firewall Rule. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the PostgreSQL Server exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the PostgreSQL Server. Changing this forces a new resource to be created.
        pub server_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Start IP Address associated with this Firewall Rule. Changing this forces a new resource to be created.
        pub start_ip_address: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FirewallRuleArgs,
    ) -> FirewallRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let end_ip_address_binding = args.end_ip_address.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let server_name_binding = args.server_name.get_output(context).get_inner();
        let start_ip_address_binding = args
            .start_ip_address
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/firewallRule:FirewallRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serverName".into(),
                    value: &server_name_binding,
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
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serverName".into(),
                },
                register_interface::ResultField {
                    name: "startIpAddress".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FirewallRuleResult {
            end_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endIpAddress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            server_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverName").unwrap(),
            ),
            start_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startIpAddress").unwrap(),
            ),
        }
    }
}
