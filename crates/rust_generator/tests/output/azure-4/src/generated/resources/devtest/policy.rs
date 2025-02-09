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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// A description for the Policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Evaluation Type used for this Policy. Possible values include: 'AllowedValuesPolicy', 'MaxValuePolicy'. Changing this forces a new resource to be created.
        #[builder(into)]
        pub evaluator_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Fact Data for this Policy.
        #[builder(into, default)]
        pub fact_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Dev Test Lab in which the Policy should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lab_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Dev Test Policy. Possible values are `GalleryImage`, `LabPremiumVmCount`, `LabTargetCost`, `LabVmCount`, `LabVmSize`, `UserOwnedLabPremiumVmCount`, `UserOwnedLabVmCount` and `UserOwnedLabVmCountInSubnet`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Policy Set within the Dev Test Lab where this policy should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub policy_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the Dev Test Lab resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Threshold for this Policy.
        #[builder(into)]
        pub threshold: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// A description for the Policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Evaluation Type used for this Policy. Possible values include: 'AllowedValuesPolicy', 'MaxValuePolicy'. Changing this forces a new resource to be created.
        pub evaluator_type: pulumi_gestalt_rust::Output<String>,
        /// The Fact Data for this Policy.
        pub fact_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Dev Test Lab in which the Policy should be created. Changing this forces a new resource to be created.
        pub lab_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Dev Test Policy. Possible values are `GalleryImage`, `LabPremiumVmCount`, `LabTargetCost`, `LabVmCount`, `LabVmSize`, `UserOwnedLabPremiumVmCount`, `UserOwnedLabVmCount` and `UserOwnedLabVmCountInSubnet`. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Policy Set within the Dev Test Lab where this policy should be created. Changing this forces a new resource to be created.
        pub policy_set_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Dev Test Lab resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Threshold for this Policy.
        pub threshold: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let evaluator_type_binding = args.evaluator_type.get_output(context);
        let fact_data_binding = args.fact_data.get_output(context);
        let lab_name_binding = args.lab_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_set_name_binding = args.policy_set_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let threshold_binding = args.threshold.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devtest/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "evaluatorType".into(),
                    value: evaluator_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "factData".into(),
                    value: fact_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labName".into(),
                    value: lab_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policySetName".into(),
                    value: policy_set_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "threshold".into(),
                    value: threshold_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyResult {
            description: o.get_field("description"),
            evaluator_type: o.get_field("evaluatorType"),
            fact_data: o.get_field("factData"),
            lab_name: o.get_field("labName"),
            name: o.get_field("name"),
            policy_set_name: o.get_field("policySetName"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            threshold: o.get_field("threshold"),
        }
    }
}
