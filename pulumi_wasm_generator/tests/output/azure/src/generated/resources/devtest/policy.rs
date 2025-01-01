/// Manages a Policy within a Dev Test Policy Set.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleLab:
///     type: azure:devtest:Lab
///     name: example
///     properties:
///       name: example-devtestlab
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         Sydney: Australia
///   examplePolicy:
///     type: azure:devtest:Policy
///     name: example
///     properties:
///       name: LabVmCount
///       policySetName: default
///       labName: ${exampleLab.name}
///       resourceGroupName: ${example.name}
///       factData: ""
///       threshold: '999'
///       evaluatorType: MaxValuePolicy
///       tags:
///         Acceptance: Test
/// ```
///
/// ## Import
///
/// Dev Test Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devtest/policy:Policy policy1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevTestLab/labs/lab1/policySets/default/policies/policy1
/// ```
///
pub mod policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// A description for the Policy.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Evaluation Type used for this Policy. Possible values include: 'AllowedValuesPolicy', 'MaxValuePolicy'. Changing this forces a new resource to be created.
        #[builder(into)]
        pub evaluator_type: pulumi_wasm_rust::Output<String>,
        /// The Fact Data for this Policy.
        #[builder(into, default)]
        pub fact_data: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Dev Test Lab in which the Policy should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lab_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Dev Test Policy. Possible values are `GalleryImage`, `LabPremiumVmCount`, `LabTargetCost`, `LabVmCount`, `LabVmSize`, `UserOwnedLabPremiumVmCount`, `UserOwnedLabVmCount` and `UserOwnedLabVmCountInSubnet`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Policy Set within the Dev Test Lab where this policy should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub policy_set_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Dev Test Lab resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Threshold for this Policy.
        #[builder(into)]
        pub threshold: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// A description for the Policy.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Evaluation Type used for this Policy. Possible values include: 'AllowedValuesPolicy', 'MaxValuePolicy'. Changing this forces a new resource to be created.
        pub evaluator_type: pulumi_wasm_rust::Output<String>,
        /// The Fact Data for this Policy.
        pub fact_data: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Dev Test Lab in which the Policy should be created. Changing this forces a new resource to be created.
        pub lab_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Dev Test Policy. Possible values are `GalleryImage`, `LabPremiumVmCount`, `LabTargetCost`, `LabVmCount`, `LabVmSize`, `UserOwnedLabPremiumVmCount`, `UserOwnedLabVmCount` and `UserOwnedLabVmCountInSubnet`. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Policy Set within the Dev Test Lab where this policy should be created. Changing this forces a new resource to be created.
        pub policy_set_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Dev Test Lab resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Threshold for this Policy.
        pub threshold: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyArgs) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let evaluator_type_binding = args.evaluator_type.get_inner();
        let fact_data_binding = args.fact_data.get_inner();
        let lab_name_binding = args.lab_name.get_inner();
        let name_binding = args.name.get_inner();
        let policy_set_name_binding = args.policy_set_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let threshold_binding = args.threshold.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devtest/policy:Policy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "evaluatorType".into(),
                    value: &evaluator_type_binding,
                },
                register_interface::ObjectField {
                    name: "factData".into(),
                    value: &fact_data_binding,
                },
                register_interface::ObjectField {
                    name: "labName".into(),
                    value: &lab_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policySetName".into(),
                    value: &policy_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "threshold".into(),
                    value: &threshold_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "evaluatorType".into(),
                },
                register_interface::ResultField {
                    name: "factData".into(),
                },
                register_interface::ResultField {
                    name: "labName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policySetName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "threshold".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicyResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            evaluator_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("evaluatorType").unwrap(),
            ),
            fact_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("factData").unwrap(),
            ),
            lab_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policySetName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            threshold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threshold").unwrap(),
            ),
        }
    }
}
