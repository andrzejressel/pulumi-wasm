/// Manages a Subscription Policy Assignment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:policy:Definition
///     properties:
///       name: only-deploy-in-westeurope
///       policyType: Custom
///       mode: All
///       displayName: Allowed resource types
///       policyRule: |2
///          {
///             "if": {
///               "not": {
///                 "field": "location",
///                 "equals": "westeurope"
///               }
///             },
///             "then": {
///               "effect": "Deny"
///             }
///           }
///   exampleSubscriptionPolicyAssignment:
///     type: azure:core:SubscriptionPolicyAssignment
///     name: example
///     properties:
///       name: example
///       policyDefinitionId: ${example.id}
///       subscriptionId: ${current.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Subscription Policy Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/subscriptionPolicyAssignment:SubscriptionPolicyAssignment example /subscriptions/00000000-0000-0000-000000000000/providers/Microsoft.Authorization/policyAssignments/assignment1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subscription_policy_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionPolicyAssignmentArgs {
        /// A description which should be used for this Policy Assignment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Display Name for this Policy Assignment.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if this Policy should be enforced or not? Defaults to `true`.
        #[builder(into, default)]
        pub enforce: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        ///
        /// > **Note:** The `location` field must also be specified when `identity` is specified.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::core::SubscriptionPolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment should exist. Changing this forces a new Policy Assignment to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A JSON mapping of any Metadata for this Policy.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Policy Assignment. Changing this forces a new Policy Assignment to be created. Cannot exceed 64 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `non_compliance_message` blocks as defined below.
        #[builder(into, default)]
        pub non_compliance_messages: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::core::SubscriptionPolicyAssignmentNonComplianceMessage,
                >,
            >,
        >,
        /// Specifies a list of Resource Scopes (for example a Subscription, or a Resource Group) within this Management Group which are excluded from this Policy.
        #[builder(into, default)]
        pub not_scopes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more `overrides` blocks as defined below. More detail about `overrides` and `resource_selectors` see [policy assignment structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/assignment-structure#resource-selectors-preview)
        #[builder(into, default)]
        pub overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::core::SubscriptionPolicyAssignmentOverride>>,
        >,
        /// A JSON mapping of any Parameters for this Policy.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Policy Definition or Policy Definition Set. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub policy_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `resource_selectors` blocks as defined below to filter polices by resource properties.
        #[builder(into, default)]
        pub resource_selectors: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::core::SubscriptionPolicyAssignmentResourceSelector,
                >,
            >,
        >,
        /// The ID of the Subscription where this Policy Assignment should be created. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionPolicyAssignmentResult {
        /// A description which should be used for this Policy Assignment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Display Name for this Policy Assignment.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if this Policy should be enforced or not? Defaults to `true`.
        pub enforce: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        ///
        /// > **Note:** The `location` field must also be specified when `identity` is specified.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::core::SubscriptionPolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment should exist. Changing this forces a new Policy Assignment to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Policy.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Policy Assignment. Changing this forces a new Policy Assignment to be created. Cannot exceed 64 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `non_compliance_message` blocks as defined below.
        pub non_compliance_messages: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::core::SubscriptionPolicyAssignmentNonComplianceMessage,
                >,
            >,
        >,
        /// Specifies a list of Resource Scopes (for example a Subscription, or a Resource Group) within this Management Group which are excluded from this Policy.
        pub not_scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// One or more `overrides` blocks as defined below. More detail about `overrides` and `resource_selectors` see [policy assignment structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/assignment-structure#resource-selectors-preview)
        pub overrides: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::core::SubscriptionPolicyAssignmentOverride>>,
        >,
        /// A JSON mapping of any Parameters for this Policy.
        pub parameters: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Policy Definition or Policy Definition Set. Changing this forces a new Policy Assignment to be created.
        pub policy_definition_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `resource_selectors` blocks as defined below to filter polices by resource properties.
        pub resource_selectors: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::core::SubscriptionPolicyAssignmentResourceSelector,
                >,
            >,
        >,
        /// The ID of the Subscription where this Policy Assignment should be created. Changing this forces a new Policy Assignment to be created.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubscriptionPolicyAssignmentArgs,
    ) -> SubscriptionPolicyAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enforce_binding = args.enforce.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let non_compliance_messages_binding = args
            .non_compliance_messages
            .get_output(context);
        let not_scopes_binding = args.not_scopes.get_output(context);
        let overrides_binding = args.overrides.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let policy_definition_id_binding = args.policy_definition_id.get_output(context);
        let resource_selectors_binding = args.resource_selectors.get_output(context);
        let subscription_id_binding = args.subscription_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:core/subscriptionPolicyAssignment:SubscriptionPolicyAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enforce".into(),
                    value: enforce_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nonComplianceMessages".into(),
                    value: non_compliance_messages_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notScopes".into(),
                    value: not_scopes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overrides".into(),
                    value: overrides_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDefinitionId".into(),
                    value: policy_definition_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceSelectors".into(),
                    value: resource_selectors_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscriptionId".into(),
                    value: subscription_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubscriptionPolicyAssignmentResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            enforce: o.get_field("enforce"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            non_compliance_messages: o.get_field("nonComplianceMessages"),
            not_scopes: o.get_field("notScopes"),
            overrides: o.get_field("overrides"),
            parameters: o.get_field("parameters"),
            policy_definition_id: o.get_field("policyDefinitionId"),
            resource_selectors: o.get_field("resourceSelectors"),
            subscription_id: o.get_field("subscriptionId"),
        }
    }
}
