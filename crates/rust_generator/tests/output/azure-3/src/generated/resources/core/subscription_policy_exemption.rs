/// Manages a Subscription Policy Exemption.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleSubscriptionPolicyAssignment:
///     type: azure:core:SubscriptionPolicyAssignment
///     name: example
///     properties:
///       name: exampleAssignment
///       subscriptionId: ${example.id}
///       policyDefinitionId: ${exampleGetPolicySetDefinition.id}
///       location: westus
///       identity:
///         type: SystemAssigned
///   exampleSubscriptionPolicyExemption:
///     type: azure:core:SubscriptionPolicyExemption
///     name: example
///     properties:
///       name: exampleExemption
///       subscriptionId: ${example.id}
///       policyAssignmentId: ${exampleSubscriptionPolicyAssignment.id}
///       exemptionCategory: Mitigated
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   exampleGetPolicySetDefinition:
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
/// $ pulumi import azure:core/subscriptionPolicyExemption:SubscriptionPolicyExemption exemption1 /subscriptions/00000000-0000-0000-000000000000/providers/Microsoft.Authorization/policyExemptions/exemption1
/// ```
///
pub mod subscription_policy_exemption {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionPolicyExemptionArgs {
        /// A description to use for this Policy Exemption.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A friendly display name to use for this Policy Exemption.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The category of this policy exemption. Possible values are `Waiver` and `Mitigated`.
        #[builder(into)]
        pub exemption_category: pulumi_wasm_rust::InputOrOutput<String>,
        /// The expiration date and time in UTC ISO 8601 format of this policy exemption.
        #[builder(into, default)]
        pub expires_on: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The metadata for this policy exemption. This is a JSON string representing additional metadata that should be stored with the policy exemption.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Policy Exemption. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Policy Assignment to be exempted at the specified Scope. Changing this forces a new resource to be created.
        #[builder(into)]
        pub policy_assignment_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The policy definition reference ID list when the associated policy assignment is an assignment of a policy set definition.
        #[builder(into, default)]
        pub policy_definition_reference_ids: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The Subscription ID where the Policy Exemption should be applied. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subscription_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionPolicyExemptionResult {
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
        /// The Subscription ID where the Policy Exemption should be applied. Changing this forces a new resource to be created.
        pub subscription_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SubscriptionPolicyExemptionArgs,
    ) -> SubscriptionPolicyExemptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let exemption_category_binding = args
            .exemption_category
            .get_output(context)
            .get_inner();
        let expires_on_binding = args.expires_on.get_output(context).get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let policy_assignment_id_binding = args
            .policy_assignment_id
            .get_output(context)
            .get_inner();
        let policy_definition_reference_ids_binding = args
            .policy_definition_reference_ids
            .get_output(context)
            .get_inner();
        let subscription_id_binding = args
            .subscription_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/subscriptionPolicyExemption:SubscriptionPolicyExemption"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubscriptionPolicyExemptionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            exemption_category: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("exemptionCategory"),
            ),
            expires_on: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expiresOn"),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy_assignment_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyAssignmentId"),
            ),
            policy_definition_reference_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyDefinitionReferenceIds"),
            ),
            subscription_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subscriptionId"),
            ),
        }
    }
}
