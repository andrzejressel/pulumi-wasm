/// Provides a SageMaker Model Package Group Policy resource.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Model Package Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/modelPackageGroupPolicy:ModelPackageGroupPolicy example example
/// ```
pub mod model_package_group_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelPackageGroupPolicyArgs {
        /// The name of the model package group.
        #[builder(into)]
        pub model_package_group_name: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub resource_policy: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ModelPackageGroupPolicyResult {
        /// The name of the model package group.
        pub model_package_group_name: pulumi_wasm_rust::Output<String>,
        pub resource_policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ModelPackageGroupPolicyArgs,
    ) -> ModelPackageGroupPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let model_package_group_name_binding = args.model_package_group_name.get_inner();
        let resource_policy_binding = args.resource_policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/modelPackageGroupPolicy:ModelPackageGroupPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "modelPackageGroupName".into(),
                    value: &model_package_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourcePolicy".into(),
                    value: &resource_policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "modelPackageGroupName".into(),
                },
                register_interface::ResultField {
                    name: "resourcePolicy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ModelPackageGroupPolicyResult {
            model_package_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelPackageGroupName").unwrap(),
            ),
            resource_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourcePolicy").unwrap(),
            ),
        }
    }
}