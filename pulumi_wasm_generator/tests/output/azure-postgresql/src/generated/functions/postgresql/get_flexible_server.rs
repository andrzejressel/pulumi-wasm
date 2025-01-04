pub mod get_flexible_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFlexibleServerArgs {
        /// The name of this PostgreSQL Flexible Server.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the PostgreSQL Flexible Server exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetFlexibleServerResult {
        /// The Administrator login for the PostgreSQL Flexible Server.
        pub administrator_login: pulumi_wasm_rust::Output<String>,
        /// Is the storage auto grow for PostgreSQL Flexible Server enabled?
        pub auto_grow_enabled: pulumi_wasm_rust::Output<bool>,
        /// The backup retention days for the PostgreSQL Flexible Server.
        pub backup_retention_days: pulumi_wasm_rust::Output<i32>,
        /// The ID of the virtual network subnet to create the PostgreSQL Flexible Server.
        pub delegated_subnet_id: pulumi_wasm_rust::Output<String>,
        /// The FQDN of the PostgreSQL Flexible Server.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the PostgreSQL Flexible Server exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Is public network access enabled?
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The SKU Name for the PostgreSQL Flexible Server. The name of the SKU, follows the `tier` + `name` pattern (e.g. `B_Standard_B1ms`, `GP_Standard_D2s_v3`, `MO_Standard_E4s_v3`).
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// The max storage allowed for the PostgreSQL Flexible Server.
        pub storage_mb: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags assigned to the PostgreSQL Flexible Server.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The version of PostgreSQL Flexible Server to use.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFlexibleServerArgs) -> GetFlexibleServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:postgresql/getFlexibleServer:getFlexibleServer".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "administratorLogin".into(),
                },
                register_interface::ResultField {
                    name: "autoGrowEnabled".into(),
                },
                register_interface::ResultField {
                    name: "backupRetentionDays".into(),
                },
                register_interface::ResultField {
                    name: "delegatedSubnetId".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "storageMb".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFlexibleServerResult {
            administrator_login: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLogin").unwrap(),
            ),
            auto_grow_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoGrowEnabled").unwrap(),
            ),
            backup_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRetentionDays").unwrap(),
            ),
            delegated_subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegatedSubnetId").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            storage_mb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageMb").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
