pub mod role_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoleAssignmentArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub condition_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub delegated_managed_identity_resource_id: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub principal_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub role_definition_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub role_definition_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub skip_service_principal_aad_check: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct RoleAssignmentResult {
        pub condition: pulumi_wasm_rust::Output<Option<String>>,
        pub condition_version: pulumi_wasm_rust::Output<Option<String>>,
        pub delegated_managed_identity_resource_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub principal_id: pulumi_wasm_rust::Output<String>,
        pub principal_type: pulumi_wasm_rust::Output<String>,
        pub role_definition_id: pulumi_wasm_rust::Output<Option<String>>,
        pub role_definition_name: pulumi_wasm_rust::Output<Option<String>>,
        pub skip_service_principal_aad_check: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RoleAssignmentArgs,
    ) -> RoleAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_output(context).get_inner();
        let condition_version_binding = args
            .condition_version
            .get_output(context)
            .get_inner();
        let delegated_managed_identity_resource_id_binding = args
            .delegated_managed_identity_resource_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let principal_id_binding = args.principal_id.get_output(context).get_inner();
        let role_definition_id_binding = args
            .role_definition_id
            .get_output(context)
            .get_inner();
        let role_definition_name_binding = args
            .role_definition_name
            .get_output(context)
            .get_inner();
        let skip_service_principal_aad_check_binding = args
            .skip_service_principal_aad_check
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:marketplace/roleAssignment:RoleAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "conditionVersion".into(),
                    value: &condition_version_binding,
                },
                register_interface::ObjectField {
                    name: "delegatedManagedIdentityResourceId".into(),
                    value: &delegated_managed_identity_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "roleDefinitionId".into(),
                    value: &role_definition_id_binding,
                },
                register_interface::ObjectField {
                    name: "roleDefinitionName".into(),
                    value: &role_definition_name_binding,
                },
                register_interface::ObjectField {
                    name: "skipServicePrincipalAadCheck".into(),
                    value: &skip_service_principal_aad_check_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "conditionVersion".into(),
                },
                register_interface::ResultField {
                    name: "delegatedManagedIdentityResourceId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "principalId".into(),
                },
                register_interface::ResultField {
                    name: "principalType".into(),
                },
                register_interface::ResultField {
                    name: "roleDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "roleDefinitionName".into(),
                },
                register_interface::ResultField {
                    name: "skipServicePrincipalAadCheck".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RoleAssignmentResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            condition_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditionVersion").unwrap(),
            ),
            delegated_managed_identity_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegatedManagedIdentityResourceId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            principal_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalId").unwrap(),
            ),
            principal_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalType").unwrap(),
            ),
            role_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleDefinitionId").unwrap(),
            ),
            role_definition_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleDefinitionName").unwrap(),
            ),
            skip_service_principal_aad_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipServicePrincipalAadCheck").unwrap(),
            ),
        }
    }
}
