/// Sets a MySQL Flexible Server Configuration value on a MySQL Flexible Server.
///
/// ## Disclaimers
///
/// > **Note:** Since this resource is provisioned by default, the Azure Provider will not check for the presence of an existing resource prior to attempting to create it.
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
///             .administrator_login("adminTerraform")
///             .administrator_password("H@Sh1CoR3!")
///             .location("${example.location}")
///             .name("example-fs")
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Standard_D2ds_v4")
///             .build_struct(),
///     );
///     let exampleFlexibleServerConfiguration = flexible_server_configuration::create(
///         "exampleFlexibleServerConfiguration",
///         FlexibleServerConfigurationArgs::builder()
///             .name("interactive_timeout")
///             .resource_group_name("${example.name}")
///             .server_name("${exampleFlexibleServer.name}")
///             .value("600")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MySQL Flexible Server Configurations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mysql/flexibleServerConfiguration:FlexibleServerConfiguration interactive_timeout /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DBforMySQL/flexibleServers/flexibleServer1/configurations/interactive_timeout
/// ```
///
pub mod flexible_server_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerConfigurationArgs {
        /// Specifies the name of the MySQL Flexible Server Configuration, which needs [to be a valid MySQL configuration name](https://dev.mysql.com/doc/refman/5.7/en/server-configuration.html). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the MySQL Flexible Server exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the MySQL Flexible Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the value of the MySQL Flexible Server Configuration. See the MySQL documentation for valid values.
        #[builder(into)]
        pub value: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerConfigurationResult {
        /// Specifies the name of the MySQL Flexible Server Configuration, which needs [to be a valid MySQL configuration name](https://dev.mysql.com/doc/refman/5.7/en/server-configuration.html). Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the MySQL Flexible Server exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the MySQL Flexible Server. Changing this forces a new resource to be created.
        pub server_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the value of the MySQL Flexible Server Configuration. See the MySQL documentation for valid values.
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
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let server_name_binding = args.server_name.get_inner();
        let value_binding = args.value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mysql/flexibleServerConfiguration:FlexibleServerConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
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
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
            results: Vec::from([
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
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            server_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverName").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
        }
    }
}
