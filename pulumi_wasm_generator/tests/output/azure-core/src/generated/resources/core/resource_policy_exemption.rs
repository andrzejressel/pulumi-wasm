/// Manages a Resource Policy Exemption.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: group1
///       location: westus
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: network1
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       addressSpaces:
///         - 10.0.0.0/16
///   exampleResourcePolicyAssignment:
///     type: azure:core:ResourcePolicyAssignment
///     name: example
///     properties:
///       name: assignment1
///       resourceId: ${exampleVirtualNetwork.id}
///       policyDefinitionId: ${example.id}
///       location: ${exampleResourceGroup.location}
///       identity:
///         type: SystemAssigned
///   exampleResourcePolicyExemption:
///     type: azure:core:ResourcePolicyExemption
///     name: example
///     properties:
///       name: exemption1
///       resourceId: ${exampleResourcePolicyAssignment.resourceId}
///       policyAssignmentId: ${exampleResourcePolicyAssignment.id}
///       exemptionCategory: Mitigated
/// variables:
///   example:
///     fn::invoke:
///       function: azure:policy:getPolicySetDefinition
///       arguments:
///         displayName: Audit machines with insecure password security settings
/// ```
///
/// ## Import
///
/// Policy Exemptions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/resourcePolicyExemption:ResourcePolicyExemption exemption1 /subscriptions/00000000-0000-0000-000000000000/resourceGroups/resGroup1/providers/Microsoft.Authorization/policyExemptions/exemption1
/// ```
///
pub mod resource_policy_exemption {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyExemptionArgs {
        /// A description to use for this Policy Exemption.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A friendly display name to use for this Policy Exemption.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The category of this policy exemption. Possible values are `Waiver` and `Mitigated`.
        #[builder(into)]
        pub exemption_category: pulumi_wasm_rust::Output<String>,
        /// The expiration date and time in UTC ISO 8601 format of this policy exemption.
        #[builder(into, default)]
        pub expires_on: pulumi_wasm_rust::Output<Option<String>>,
        /// The metadata for this policy exemption. This is a JSON string representing additional metadata that should be stored with the policy exemption.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Policy Exemption. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Policy Assignment to be exempted at the specified Scope. Changing this forces a new resource to be created.
        #[builder(into)]
        pub policy_assignment_id: pulumi_wasm_rust::Output<String>,
        /// The policy definition reference ID list when the associated policy assignment is an assignment of a policy set definition.
        #[builder(into, default)]
        pub policy_definition_reference_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The Resource ID where the Policy Exemption should be applied. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyExemptionResult {
        /// A description to use for this Policy Exemption.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A friendly display name to use for this Policy Exemption.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The category of this policy exemption. Possible values are `Waiver` and `Mitigated`.
        pub exemption_category: pulumi_wasm_rust::Output<String>,
        /// The expiration date and time in UTC ISO 8601 format of this policy exemption.
        pub expires_on: pulumi_wasm_rust::Output<Option<String>>,
        /// The metadata for this policy exemption. This is a JSON string representing additional metadata that should be stored with the policy exemption.
        pub metadata: pulumi_wasm_rust::Output<String>,
        /// The name of the Policy Exemption. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Policy Assignment to be exempted at the specified Scope. Changing this forces a new resource to be created.
        pub policy_assignment_id: pulumi_wasm_rust::Output<String>,
        /// The policy definition reference ID list when the associated policy assignment is an assignment of a policy set definition.
        pub policy_definition_reference_ids: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The Resource ID where the Policy Exemption should be applied. Changing this forces a new resource to be created.
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ResourcePolicyExemptionArgs,
    ) -> ResourcePolicyExemptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let exemption_category_binding = args.exemption_category.get_inner();
        let expires_on_binding = args.expires_on.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let policy_assignment_id_binding = args.policy_assignment_id.get_inner();
        let policy_definition_reference_ids_binding = args
            .policy_definition_reference_ids
            .get_inner();
        let resource_id_binding = args.resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/resourcePolicyExemption:ResourcePolicyExemption".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "exemptionCategory".into(),
                    value: &exemption_category_binding,
                },
                register_interface::ObjectField {
                    name: "expiresOn".into(),
                    value: &expires_on_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policyAssignmentId".into(),
                    value: &policy_assignment_id_binding,
                },
                register_interface::ObjectField {
                    name: "policyDefinitionReferenceIds".into(),
                    value: &policy_definition_reference_ids_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "exemptionCategory".into(),
                },
                register_interface::ResultField {
                    name: "expiresOn".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policyAssignmentId".into(),
                },
                register_interface::ResultField {
                    name: "policyDefinitionReferenceIds".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourcePolicyExemptionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            exemption_category: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exemptionCategory").unwrap(),
            ),
            expires_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expiresOn").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy_assignment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyAssignmentId").unwrap(),
            ),
            policy_definition_reference_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDefinitionReferenceIds").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
        }
    }
}