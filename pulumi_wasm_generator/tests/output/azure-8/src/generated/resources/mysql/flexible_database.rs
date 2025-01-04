/// Manages a MySQL Database within a MySQL Flexible Server
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
///     let exampleFlexibleDatabase = flexible_database::create(
///         "exampleFlexibleDatabase",
///         FlexibleDatabaseArgs::builder()
///             .charset("utf8")
///             .collation("utf8_unicode_ci")
///             .name("exampledb")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleFlexibleServer.name}")
///             .build_struct(),
///     );
///     let exampleFlexibleServer = flexible_server::create(
///         "exampleFlexibleServer",
///         FlexibleServerArgs::builder()
///             .administrator_login("mysqladminun")
///             .administrator_password("H@Sh1CoR3!")
///             .location("${example.location}")
///             .name("example-mysql-flexible-server")
///             .resource_group_name("${example.name}")
///             .sku_name("B_Standard_B1s")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MySQL Database's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mysql/flexibleDatabase:FlexibleDatabase database1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.DBforMySQL/flexibleServers/flexibleserver1/databases/database1
/// ```
///
pub mod flexible_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleDatabaseArgs {
        /// Specifies the Charset for the MySQL Database, which needs [to be a valid MySQL Charset](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html). Changing this forces a new resource to be created.
        #[builder(into)]
        pub charset: pulumi_wasm_rust::Output<String>,
        /// Specifies the Collation for the MySQL Database, which needs [to be a valid MySQL Collation](https://dev.mysql.com/doc/refman/5.7/en/charset-mysql.html). Changing this forces a new resource to be created.
        #[builder(into)]
        pub collation: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the MySQL Database, which needs [to be a valid MySQL identifier](https://dev.mysql.com/doc/refman/5.7/en/identifiers.html). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the MySQL Server exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the MySQL Flexible Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleDatabaseResult {
        /// Specifies the Charset for the MySQL Database, which needs [to be a valid MySQL Charset](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html). Changing this forces a new resource to be created.
        pub charset: pulumi_wasm_rust::Output<String>,
        /// Specifies the Collation for the MySQL Database, which needs [to be a valid MySQL Collation](https://dev.mysql.com/doc/refman/5.7/en/charset-mysql.html). Changing this forces a new resource to be created.
        pub collation: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the MySQL Database, which needs [to be a valid MySQL identifier](https://dev.mysql.com/doc/refman/5.7/en/identifiers.html). Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the MySQL Server exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the MySQL Flexible Server. Changing this forces a new resource to be created.
        pub server_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FlexibleDatabaseArgs) -> FlexibleDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let charset_binding = args.charset.get_inner();
        let collation_binding = args.collation.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let server_name_binding = args.server_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mysql/flexibleDatabase:FlexibleDatabase".into(),
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
        FlexibleDatabaseResult {
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
