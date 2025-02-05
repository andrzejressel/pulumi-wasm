pub mod get_sql_role_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSqlRoleDefinitionArgs {
        /// The name of the Cosmos DB Account.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Cosmos DB SQL Role Definition is created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The GUID as the name of the Cosmos DB SQL Role Definition.
        #[builder(into)]
        pub role_definition_id: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSqlRoleDefinitionArgs,
    ) -> GetSqlRoleDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let role_definition_id_binding = args
            .role_definition_id
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSqlRoleDefinitionResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            assignable_scopes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("assignableScopes"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            role_definition_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleDefinitionId"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
