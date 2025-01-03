pub mod get_environment_blueprint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnvironmentBlueprintArgs {
        /// ID of the domain.
        #[builder(into)]
        pub domain_id: pulumi_wasm_rust::Output<String>,
        /// Whether the blueprint is managed by Amazon DataZone.
        #[builder(into)]
        pub managed: pulumi_wasm_rust::Output<bool>,
        /// Name of the blueprint.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnvironmentBlueprintResult {
        /// Provider of the blueprint
        pub blueprint_provider: pulumi_wasm_rust::Output<String>,
        /// Description of the blueprint
        pub description: pulumi_wasm_rust::Output<String>,
        pub domain_id: pulumi_wasm_rust::Output<String>,
        /// ID of the environment blueprint
        pub id: pulumi_wasm_rust::Output<String>,
        pub managed: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetEnvironmentBlueprintArgs) -> GetEnvironmentBlueprintResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_id_binding = args.domain_id.get_inner();
        let managed_binding = args.managed.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:datazone/getEnvironmentBlueprint:getEnvironmentBlueprint".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainId".into(),
                    value: &domain_id_binding,
                },
                register_interface::ObjectField {
                    name: "managed".into(),
                    value: &managed_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "blueprintProvider".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "domainId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "managed".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEnvironmentBlueprintResult {
            blueprint_provider: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blueprintProvider").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            domain_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            managed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managed").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
