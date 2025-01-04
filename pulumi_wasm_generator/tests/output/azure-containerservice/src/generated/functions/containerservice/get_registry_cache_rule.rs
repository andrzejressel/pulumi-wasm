pub mod get_registry_cache_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegistryCacheRuleArgs {
        /// The ID of the container registry where the cache rule should apply. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_registry_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Container Registry Cache Rule. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetRegistryCacheRuleResult {
        pub container_registry_id: pulumi_wasm_rust::Output<String>,
        /// The ARM resource ID of the credential store which is associated with the cache rule.
        pub credential_set_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the source repository path.
        pub source_repo: pulumi_wasm_rust::Output<String>,
        /// The name of the new repository path to store artifacts.
        pub target_repo: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRegistryCacheRuleArgs) -> GetRegistryCacheRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_registry_id_binding = args.container_registry_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:containerservice/getRegistryCacheRule:getRegistryCacheRule"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerRegistryId".into(),
                    value: &container_registry_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerRegistryId".into(),
                },
                register_interface::ResultField {
                    name: "credentialSetId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sourceRepo".into(),
                },
                register_interface::ResultField {
                    name: "targetRepo".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRegistryCacheRuleResult {
            container_registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerRegistryId").unwrap(),
            ),
            credential_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentialSetId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            source_repo: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceRepo").unwrap(),
            ),
            target_repo: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetRepo").unwrap(),
            ),
        }
    }
}
