/// Sets a PostgreSQL Configuration value on a Azure PostgreSQL Flexible Server.
///
/// > **Note:** Changes to static server parameters will automatically trigger Azure Flex Server restart. This behavior can be disabled in the provider `features` block by setting the `restart_server_on_configuration_value_change` field to `false` within the `postgresql_flexible_server` block.
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
///     let exampleFlexibleServerConfiguration = flexible_server_configuration::create(
///         "exampleFlexibleServerConfiguration",
///         FlexibleServerConfigurationArgs::builder()
///             .name("backslash_quote")
///             .server_id("${exampleFlexibleServer.id}")
///             .value("on")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Azure Extensions
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
///     let exampleFlexibleServerConfiguration = flexible_server_configuration::create(
///         "exampleFlexibleServerConfiguration",
///         FlexibleServerConfigurationArgs::builder()
///             .name("azure.extensions")
///             .server_id("${exampleFlexibleServer.id}")
///             .value("CUBE,CITEXT,BTREE_GIST")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PostgreSQL Configurations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/flexibleServerConfiguration:FlexibleServerConfiguration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.DBforPostgreSQL/flexibleServers/server1/configurations/configuration1
/// ```
///
pub mod flexible_server_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerConfigurationArgs {
        /// Specifies the name of the PostgreSQL Configuration, which needs [to be a valid PostgreSQL configuration name](https://www.postgresql.org/docs/current/static/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIER). Changing this forces a new resource to be created.
        ///
        /// > **Note:** PostgreSQL provides the ability to extend the functionality using azure extensions, with PostgreSQL azure extensions you should specify the `name` value as `azure.extensions` and the `value` you wish to allow in the [extensions list](https://learn.microsoft.com/en-us/azure/postgresql/flexible-server/concepts-extensions?WT.mc_id=Portal-Microsoft_Azure_OSSDatabases#extension-versions).
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the PostgreSQL Flexible Server where we want to change configuration. Changing this forces a new PostgreSQL Flexible Server Configuration resource.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the value of the PostgreSQL Configuration. See the PostgreSQL documentation for valid values.
        #[builder(into)]
        pub value: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerConfigurationResult {
        /// Specifies the name of the PostgreSQL Configuration, which needs [to be a valid PostgreSQL configuration name](https://www.postgresql.org/docs/current/static/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIER). Changing this forces a new resource to be created.
        ///
        /// > **Note:** PostgreSQL provides the ability to extend the functionality using azure extensions, with PostgreSQL azure extensions you should specify the `name` value as `azure.extensions` and the `value` you wish to allow in the [extensions list](https://learn.microsoft.com/en-us/azure/postgresql/flexible-server/concepts-extensions?WT.mc_id=Portal-Microsoft_Azure_OSSDatabases#extension-versions).
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the PostgreSQL Flexible Server where we want to change configuration. Changing this forces a new PostgreSQL Flexible Server Configuration resource.
        pub server_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the value of the PostgreSQL Configuration. See the PostgreSQL documentation for valid values.
        pub value: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FlexibleServerConfigurationArgs,
    ) -> FlexibleServerConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let server_id_binding = args.server_id.get_inner();
        let value_binding = args.value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/flexibleServerConfiguration:FlexibleServerConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "serverId".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FlexibleServerConfigurationResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
        }
    }
}
