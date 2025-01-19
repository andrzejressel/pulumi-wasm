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
///             .name("api-rg-pro")
///             .build_struct(),
///     );
///     let exampleDatabase = database::create(
///         "exampleDatabase",
///         DatabaseArgs::builder()
///             .charset("UTF8")
///             .collation("English_United States.1252")
///             .name("exampledb")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleServer.name}")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("psqladmin")
///             .administrator_login_password("H@Sh1CoR3!")
///             .auto_grow_enabled(true)
///             .backup_retention_days(7)
///             .geo_redundant_backup_enabled(false)
///             .location("${example.location}")
///             .name("postgresql-server-1")
///             .resource_group_name("${example.name}")
///             .sku_name("B_Gen5_2")
///             .ssl_enforcement_enabled(true)
///             .storage_mb(5120)
///             .version("9.5")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PostgreSQL Database's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/database:Database database1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.DBforPostgreSQL/servers/server1/databases/database1
/// ```
///
pub mod database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// Specifies the Charset for the PostgreSQL Database, which needs [to be a valid PostgreSQL Charset](https://www.postgresql.org/docs/current/static/multibyte.html). Changing this forces a new resource to be created.
        #[builder(into)]
        pub charset: pulumi_wasm_rust::Output<String>,
        /// Specifies the Collation for the PostgreSQL Database, which needs [to be a valid PostgreSQL Collation](https://www.postgresql.org/docs/current/static/collation.html). Note that Microsoft uses different [notation](https://msdn.microsoft.com/library/windows/desktop/dd373814.aspx) - en-US instead of en_US. Changing this forces a new resource to be created.
        #[builder(into)]
        pub collation: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the PostgreSQL Database, which needs [to be a valid PostgreSQL identifier](https://www.postgresql.org/docs/current/static/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the PostgreSQL Server exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the PostgreSQL Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// Specifies the Charset for the PostgreSQL Database, which needs [to be a valid PostgreSQL Charset](https://www.postgresql.org/docs/current/static/multibyte.html). Changing this forces a new resource to be created.
        pub charset: pulumi_wasm_rust::Output<String>,
        /// Specifies the Collation for the PostgreSQL Database, which needs [to be a valid PostgreSQL Collation](https://www.postgresql.org/docs/current/static/collation.html). Note that Microsoft uses different [notation](https://msdn.microsoft.com/library/windows/desktop/dd373814.aspx) - en-US instead of en_US. Changing this forces a new resource to be created.
        pub collation: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the PostgreSQL Database, which needs [to be a valid PostgreSQL identifier](https://www.postgresql.org/docs/current/static/sql-syntax-lexical.html#SQL-SYNTAX-IDENTIFIERS). Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the PostgreSQL Server exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the PostgreSQL Server. Changing this forces a new resource to be created.
        pub server_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DatabaseArgs) -> DatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let charset_binding = args.charset.get_inner();
        let collation_binding = args.collation.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let server_name_binding = args.server_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serverName".into(),
                    value: &server_name_binding,
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
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serverName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatabaseResult {
            charset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("charset").unwrap(),
            ),
            collation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collation").unwrap(),
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
        }
    }
}
