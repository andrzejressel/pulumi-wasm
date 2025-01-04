pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The Azure Region where the Service is located.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Healthcare Service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the Healthcare Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        pub access_policy_object_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// An `authentication_configuration` block as defined below.
        pub authentication_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::healthcare::GetServiceAuthenticationConfiguration,
            >,
        >,
        /// A `cors_configuration` block as defined below.
        pub cors_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::healthcare::GetServiceCorsConfiguration>,
        >,
        /// The versionless Key Vault Key ID for CMK encryption of the backing database.
        pub cosmosdb_key_vault_key_versionless_id: pulumi_wasm_rust::Output<String>,
        /// The provisioned throughput for the backing database.
        pub cosmosdb_throughput: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The type of the service.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Service is located.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServiceArgs) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:healthcare/getService:getService".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
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
                    name: "accessPolicyObjectIds".into(),
                },
                register_interface::ResultField {
                    name: "authenticationConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "corsConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "cosmosdbKeyVaultKeyVersionlessId".into(),
                },
                register_interface::ResultField {
                    name: "cosmosdbThroughput".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceResult {
            access_policy_object_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPolicyObjectIds").unwrap(),
            ),
            authentication_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationConfigurations").unwrap(),
            ),
            cors_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("corsConfigurations").unwrap(),
            ),
            cosmosdb_key_vault_key_versionless_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cosmosdbKeyVaultKeyVersionlessId").unwrap(),
            ),
            cosmosdb_throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cosmosdbThroughput").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
