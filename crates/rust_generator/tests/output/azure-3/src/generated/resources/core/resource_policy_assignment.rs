/// Manages a Policy Assignment to a Resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleDefinition:
///     type: azure:policy:Definition
///     name: example
///     properties:
///       name: only-deploy-in-westeurope
///       policyType: Custom
///       mode: All
///       displayName: my-policy-definition
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
///   exampleResourcePolicyAssignment:
///     type: azure:core:ResourcePolicyAssignment
///     name: example
///     properties:
///       name: example-policy-assignment
///       resourceId: ${example.id}
///       policyDefinitionId: ${exampleDefinition.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:network:getVirtualNetwork
///       arguments:
///         name: production
///         resourceGroupName: networking
/// ```
///
/// ## Import
///
/// Resource Policy Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/resourcePolicyAssignment:ResourcePolicyAssignment example "{resource}/providers/Microsoft.Authorization/policyAssignments/assignment1"
/// ```
///
/// where `{resource}` is a Resource ID in the form `/subscriptions/00000000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualNetworks/network1`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_policy_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyAssignmentArgs {
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
            Option<super::super::types::core::ResourcePolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment should exist. Changing this forces a new Policy Assignment to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A JSON mapping of any Metadata for this Policy.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Policy Assignment. Changing this forces a new Resource Policy Assignment to be created. Cannot exceed 64 characters in length.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `non_compliance_message` blocks as defined below.
        #[builder(into, default)]
        pub non_compliance_messages: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::core::ResourcePolicyAssignmentNonComplianceMessage,
                >,
            >,
        >,
        /// Specifies a list of Resource Scopes (for example a Subscription, or a Resource Group) within this Management Group which are excluded from this Policy.
        #[builder(into, default)]
        pub not_scopes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more `overrides` blocks as defined below. More detail about `overrides` and `resource_selectors` see [policy assignment structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/assignment-structure#resource-selectors-preview)
        #[builder(into, default)]
        pub overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::core::ResourcePolicyAssignmentOverride>>,
        >,
        /// A JSON mapping of any Parameters for this Policy.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Policy Definition or Policy Definition Set. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub policy_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Resource (or Resource Scope) where this should be applied. Changing this forces a new Resource Policy Assignment to be created.
        ///
        /// > To create a Policy Assignment at a Management Group use the `azure.management.GroupPolicyAssignment` resource, for a Resource Group use the `azure.core.ResourceGroupPolicyAssignment` and for a Subscription use the `azure.core.SubscriptionPolicyAssignment` resource.
        #[builder(into)]
        pub resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `resource_selectors` blocks as defined below to filter polices by resource properties.
        #[builder(into, default)]
        pub resource_selectors: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::core::ResourcePolicyAssignmentResourceSelector>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyAssignmentResult {
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
            Option<super::super::types::core::ResourcePolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment should exist. Changing this forces a new Policy Assignment to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Policy.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Policy Assignment. Changing this forces a new Resource Policy Assignment to be created. Cannot exceed 64 characters in length.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `non_compliance_message` blocks as defined below.
        pub non_compliance_messages: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::core::ResourcePolicyAssignmentNonComplianceMessage,
                >,
            >,
        >,
        /// Specifies a list of Resource Scopes (for example a Subscription, or a Resource Group) within this Management Group which are excluded from this Policy.
        pub not_scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// One or more `overrides` blocks as defined below. More detail about `overrides` and `resource_selectors` see [policy assignment structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/assignment-structure#resource-selectors-preview)
        pub overrides: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::core::ResourcePolicyAssignmentOverride>>,
        >,
        /// A JSON mapping of any Parameters for this Policy.
        pub parameters: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Policy Definition or Policy Definition Set. Changing this forces a new Policy Assignment to be created.
        pub policy_definition_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Resource (or Resource Scope) where this should be applied. Changing this forces a new Resource Policy Assignment to be created.
        ///
        /// > To create a Policy Assignment at a Management Group use the `azure.management.GroupPolicyAssignment` resource, for a Resource Group use the `azure.core.ResourceGroupPolicyAssignment` and for a Subscription use the `azure.core.SubscriptionPolicyAssignment` resource.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `resource_selectors` blocks as defined below to filter polices by resource properties.
        pub resource_selectors: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::core::ResourcePolicyAssignmentResourceSelector>,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResourcePolicyAssignmentArgs,
    ) -> ResourcePolicyAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let enforce_binding = args.enforce.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let non_compliance_messages_binding = args
            .non_compliance_messages
            .get_output(context)
            .get_inner();
        let not_scopes_binding = args.not_scopes.get_output(context).get_inner();
        let overrides_binding = args.overrides.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let policy_definition_id_binding = args
            .policy_definition_id
            .get_output(context)
            .get_inner();
        let resource_id_binding = args.resource_id.get_output(context).get_inner();
        let resource_selectors_binding = args
            .resource_selectors
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:core/resourcePolicyAssignment:ResourcePolicyAssignment".into(),
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
                    name: "enforce".into(),
                    value: &enforce_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "nonComplianceMessages".into(),
                    value: &non_compliance_messages_binding,
                },
                register_interface::ObjectField {
                    name: "notScopes".into(),
                    value: &not_scopes_binding,
                },
                register_interface::ObjectField {
                    name: "overrides".into(),
                    value: &overrides_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "policyDefinitionId".into(),
                    value: &policy_definition_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceSelectors".into(),
                    value: &resource_selectors_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourcePolicyAssignmentResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enforce: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enforce"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            non_compliance_messages: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nonComplianceMessages"),
            ),
            not_scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notScopes"),
            ),
            overrides: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("overrides"),
            ),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            policy_definition_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyDefinitionId"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            resource_selectors: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceSelectors"),
            ),
        }
    }
}
