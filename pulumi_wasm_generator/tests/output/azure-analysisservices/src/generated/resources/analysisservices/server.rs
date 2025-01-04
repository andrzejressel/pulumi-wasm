/// Manages an Analysis Services Server.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: analysis-services-server-test
///       location: West Europe
///   server:
///     type: azure:analysisservices:Server
///     properties:
///       name: analysisservicesserver
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: S0
///       adminUsers:
///         - myuser@domain.tld
///       powerBiServiceEnabled: true
///       ipv4FirewallRules:
///         - name: myRule1
///           rangeStart: 210.117.252.0
///           rangeEnd: 210.117.252.255
///       tags:
///         abc: 123
/// ```
///
/// > **NOTE:** The server resource will automatically be started and stopped during an update if it is in `paused` state.
///
/// ## Import
///
/// Analysis Services Server can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:analysisservices/server:Server server /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.AnalysisServices/servers/server1
/// ```
///
pub mod server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerArgs {
        /// List of email addresses of admin users.
        #[builder(into, default)]
        pub admin_users: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// URI and SAS token for a blob container to store backups.
        #[builder(into, default)]
        pub backup_blob_container_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `ipv4_firewall_rule` block(s) as defined below.
        #[builder(into, default)]
        pub ipv4_firewall_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::analysisservices::ServerIpv4FirewallRule>>,
        >,
        /// The Azure location where the Analysis Services Server exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Analysis Services Server. Only lowercase Alphanumeric characters allowed, starting with a letter. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates if the Power BI service is allowed to access or not.
        #[builder(into, default)]
        pub power_bi_service_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Controls how the read-write server is used in the query pool. If this value is set to `All` then read-write servers are also used for queries. Otherwise with `ReadOnly` these servers do not participate in query operations. Defaults to `All`.
        #[builder(into, default)]
        pub querypool_connection_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Analysis Services Server should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// SKU for the Analysis Services Server. Possible values are: `D1`, `B1`, `B2`, `S0`, `S1`, `S2`, `S4`, `S8`, `S9`, `S8v2` and `S9v2`.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServerResult {
        /// List of email addresses of admin users.
        pub admin_users: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// URI and SAS token for a blob container to store backups.
        pub backup_blob_container_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `ipv4_firewall_rule` block(s) as defined below.
        pub ipv4_firewall_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::analysisservices::ServerIpv4FirewallRule>>,
        >,
        /// The Azure location where the Analysis Services Server exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Analysis Services Server. Only lowercase Alphanumeric characters allowed, starting with a letter. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Indicates if the Power BI service is allowed to access or not.
        pub power_bi_service_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Controls how the read-write server is used in the query pool. If this value is set to `All` then read-write servers are also used for queries. Otherwise with `ReadOnly` these servers do not participate in query operations. Defaults to `All`.
        pub querypool_connection_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Analysis Services Server should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The full name of the Analysis Services Server.
        pub server_full_name: pulumi_wasm_rust::Output<String>,
        /// SKU for the Analysis Services Server. Possible values are: `D1`, `B1`, `B2`, `S0`, `S1`, `S2`, `S4`, `S8`, `S9`, `S8v2` and `S9v2`.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServerArgs) -> ServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_users_binding = args.admin_users.get_inner();
        let backup_blob_container_uri_binding = args
            .backup_blob_container_uri
            .get_inner();
        let ipv4_firewall_rules_binding = args.ipv4_firewall_rules.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let power_bi_service_enabled_binding = args.power_bi_service_enabled.get_inner();
        let querypool_connection_mode_binding = args
            .querypool_connection_mode
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:analysisservices/server:Server".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminUsers".into(),
                    value: &admin_users_binding,
                },
                register_interface::ObjectField {
                    name: "backupBlobContainerUri".into(),
                    value: &backup_blob_container_uri_binding,
                },
                register_interface::ObjectField {
                    name: "ipv4FirewallRules".into(),
                    value: &ipv4_firewall_rules_binding,
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
                    name: "powerBiServiceEnabled".into(),
                    value: &power_bi_service_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "querypoolConnectionMode".into(),
                    value: &querypool_connection_mode_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adminUsers".into(),
                },
                register_interface::ResultField {
                    name: "backupBlobContainerUri".into(),
                },
                register_interface::ResultField {
                    name: "ipv4FirewallRules".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "powerBiServiceEnabled".into(),
                },
                register_interface::ResultField {
                    name: "querypoolConnectionMode".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serverFullName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServerResult {
            admin_users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminUsers").unwrap(),
            ),
            backup_blob_container_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupBlobContainerUri").unwrap(),
            ),
            ipv4_firewall_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv4FirewallRules").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            power_bi_service_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("powerBiServiceEnabled").unwrap(),
            ),
            querypool_connection_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("querypoolConnectionMode").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            server_full_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverFullName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
