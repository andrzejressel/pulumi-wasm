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
///     let exampleFlexibleServerDatabase = flexible_server_database::create(
///         "exampleFlexibleServerDatabase",
///         FlexibleServerDatabaseArgs::builder()
///             .charset("utf8")
///             .collation("en_US.utf8")
///             .name("exampledb")
///             .server_id("${exampleFlexibleServer.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure PostgreSQL Flexible Server Database can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/flexibleServerDatabase:FlexibleServerDatabase example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.DBforPostgreSQL/flexibleServers/flexibleServer1/databases/database1
/// ```
///
pub mod flexible_server_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerDatabaseArgs {
        /// Specifies the Charset for the Azure PostgreSQL Flexible Server Database, which needs [to be a valid PostgreSQL Charset](https://www.postgresql.org/docs/current/static/multibyte.html). Defaults to `UTF8`. Changing this forces a new Azure PostgreSQL Flexible Server Database to be created.
        #[builder(into, default)]
        pub charset: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Collation for the Azure PostgreSQL Flexible Server Database, which needs [to be a valid PostgreSQL Collation](https://www.postgresql.org/docs/current/static/collation.html). Defaults to `en_US.utf8`. Changing this forces a new Azure PostgreSQL Flexible Server Database to be created.
        #[builder(into, default)]
        pub collation: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the PostgreSQL Database, which needs [to be a valid PostgreSQL identifier](https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS). Changing this forces a new Azure PostgreSQL Flexible Server Database to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Azure PostgreSQL Flexible Server from which to create this PostgreSQL Flexible Server Database. Changing this forces a new Azure PostgreSQL Flexible Server Database to be created.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerDatabaseResult {
        /// Specifies the Charset for the Azure PostgreSQL Flexible Server Database, which needs [to be a valid PostgreSQL Charset](https://www.postgresql.org/docs/current/static/multibyte.html). Defaults to `UTF8`. Changing this forces a new Azure PostgreSQL Flexible Server Database to be created.
        pub charset: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Collation for the Azure PostgreSQL Flexible Server Database, which needs [to be a valid PostgreSQL Collation](https://www.postgresql.org/docs/current/static/collation.html). Defaults to `en_US.utf8`. Changing this forces a new Azure PostgreSQL Flexible Server Database to be created.
        pub collation: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the PostgreSQL Database, which needs [to be a valid PostgreSQL identifier](https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS). Changing this forces a new Azure PostgreSQL Flexible Server Database to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure PostgreSQL Flexible Server from which to create this PostgreSQL Flexible Server Database. Changing this forces a new Azure PostgreSQL Flexible Server Database to be created.
        pub server_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FlexibleServerDatabaseArgs,
    ) -> FlexibleServerDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let charset_binding = args.charset.get_inner();
        let collation_binding = args.collation.get_inner();
        let name_binding = args.name.get_inner();
        let server_id_binding = args.server_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/flexibleServerDatabase:FlexibleServerDatabase"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "charset".into(),
                    value: &charset_binding,
                },
                register_interface::ObjectField {
                    name: "collation".into(),
                    value: &collation_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "charset".into(),
                },
                register_interface::ResultField {
                    name: "collation".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "serverId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FlexibleServerDatabaseResult {
            charset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("charset").unwrap(),
            ),
            collation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collation").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
        }
    }
}