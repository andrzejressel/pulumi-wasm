pub mod delivery_pipeline_iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeliveryPipelineIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::DeliveryPipelineIamBindingCondition>,
        >,
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into)]
        pub role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DeliveryPipelineIamBindingResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::DeliveryPipelineIamBindingCondition>,
        >,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DeliveryPipelineIamBindingArgs,
    ) -> DeliveryPipelineIamBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let location_binding = args.location.get_inner();
        let members_binding = args.members.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:clouddeploy/deliveryPipelineIamBinding:DeliveryPipelineIamBinding"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeliveryPipelineIamBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
