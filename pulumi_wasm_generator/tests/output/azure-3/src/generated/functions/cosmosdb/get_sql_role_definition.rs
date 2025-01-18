pub mod get_sql_role_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSqlRoleDefinitionArgs {
        /// The name of the Cosmos DB Account.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Cosmos DB SQL Role Definition is created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The GUID as the name of the Cosmos DB SQL Role Definition.
        #[builder(into)]
        pub role_definition_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSqlRoleDefinitionResult {
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// A list of fully qualified scopes at or below which Role Assignments may be created using this Cosmos DB SQL Role Definition.
        pub assignable_scopes: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The role name of the Cosmos DB SQL Role Definition.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `permissions` block as defined below.
        pub permissions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cosmosdb::GetSqlRoleDefinitionPermission>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub role_definition_id: pulumi_wasm_rust::Output<String>,
        /// The type of the Cosmos DB SQL Role Definition.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSqlRoleDefinitionArgs) -> GetSqlRoleDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let role_definition_id_binding = args.role_definition_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cosmosdb/getSqlRoleDefinition:getSqlRoleDefinition".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "assignableScopes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "roleDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSqlRoleDefinitionResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            assignable_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assignableScopes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            role_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleDefinitionId").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
