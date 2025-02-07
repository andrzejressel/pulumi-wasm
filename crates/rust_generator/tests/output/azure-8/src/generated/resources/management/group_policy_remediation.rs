/// Manages an Azure Management Group Policy Remediation.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleGroup:
///     type: azure:management:Group
///     name: example
///     properties:
///       displayName: Example Management Group
///   exampleGroupPolicyAssignment:
///     type: azure:management:GroupPolicyAssignment
///     name: example
///     properties:
///       name: exampleAssignment
///       managementGroupId: ${exampleGroup.id}
///       policyDefinitionId: ${example.id}
///       parameters:
///         fn::toJSON:
///           listOfAllowedLocations:
///             value:
///               - East US
///   exampleGroupPolicyRemediation:
///     type: azure:management:GroupPolicyRemediation
///     name: example
///     properties:
///       name: example
///       managementGroupId: ${exampleGroup.id}
///       policyAssignmentId: ${exampleGroupPolicyAssignment.id}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:policy:getPolicyDefintion
///       arguments:
///         displayName: Allowed locations
/// ```
///
/// ## Import
///
/// Policy Remediations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:management/groupPolicyRemediation:GroupPolicyRemediation example /providers/Microsoft.Management/managementGroups/my-mgmt-group-id/providers/Microsoft.PolicyInsights/remediations/remediation1
/// ```
///
pub mod group_policy_remediation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPolicyRemediationArgs {
        /// A number between 0.0 to 1.0 representing the percentage failure threshold. The remediation will fail if the percentage of failed remediation operations (i.e. failed deployments) exceeds this threshold.
        #[builder(into, default)]
        pub failure_percentage: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// A list of the resource locations that will be remediated.
        #[builder(into, default)]
        pub location_filters: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Management Group ID at which the Policy Remediation should be applied. Changing this forces a new resource to be created.
        #[builder(into)]
        pub management_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Policy Remediation. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Determines how many resources to remediate at any given time. Can be used to increase or reduce the pace of the remediation. If not provided, the default parallel deployments value is used.
        #[builder(into, default)]
        pub parallel_deployments: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the Policy Assignment that should be remediated.
        #[builder(into)]
        pub policy_assignment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The unique ID for the policy definition reference within the policy set definition that should be remediated. Required when the policy assignment being remediated assigns a policy set definition.
        #[builder(into, default)]
        pub policy_definition_reference_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Determines the max number of resources that can be remediated by the remediation job. If not provided, the default resource count is used.
        #[builder(into, default)]
        pub resource_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GroupPolicyRemediationResult {
        /// A number between 0.0 to 1.0 representing the percentage failure threshold. The remediation will fail if the percentage of failed remediation operations (i.e. failed deployments) exceeds this threshold.
        pub failure_percentage: pulumi_gestalt_rust::Output<Option<f64>>,
        /// A list of the resource locations that will be remediated.
        pub location_filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Management Group ID at which the Policy Remediation should be applied. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Policy Remediation. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Determines how many resources to remediate at any given time. Can be used to increase or reduce the pace of the remediation. If not provided, the default parallel deployments value is used.
        pub parallel_deployments: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the Policy Assignment that should be remediated.
        pub policy_assignment_id: pulumi_gestalt_rust::Output<String>,
        /// The unique ID for the policy definition reference within the policy set definition that should be remediated. Required when the policy assignment being remediated assigns a policy set definition.
        pub policy_definition_reference_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Determines the max number of resources that can be remediated by the remediation job. If not provided, the default resource count is used.
        pub resource_count: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GroupPolicyRemediationArgs,
    ) -> GroupPolicyRemediationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let failure_percentage_binding = args
            .failure_percentage
            .get_output(context)
            .get_inner();
        let location_filters_binding = args
            .location_filters
            .get_output(context)
            .get_inner();
        let management_group_id_binding = args
            .management_group_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parallel_deployments_binding = args
            .parallel_deployments
            .get_output(context)
            .get_inner();
        let policy_assignment_id_binding = args
            .policy_assignment_id
            .get_output(context)
            .get_inner();
        let policy_definition_reference_id_binding = args
            .policy_definition_reference_id
            .get_output(context)
            .get_inner();
        let resource_count_binding = args.resource_count.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:management/groupPolicyRemediation:GroupPolicyRemediation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "failurePercentage".into(),
                    value: &failure_percentage_binding,
                },
                register_interface::ObjectField {
                    name: "locationFilters".into(),
                    value: &location_filters_binding,
                },
                register_interface::ObjectField {
                    name: "managementGroupId".into(),
                    value: &management_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parallelDeployments".into(),
                    value: &parallel_deployments_binding,
                },
                register_interface::ObjectField {
                    name: "policyAssignmentId".into(),
                    value: &policy_assignment_id_binding,
                },
                register_interface::ObjectField {
                    name: "policyDefinitionReferenceId".into(),
                    value: &policy_definition_reference_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceCount".into(),
                    value: &resource_count_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupPolicyRemediationResult {
            failure_percentage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("failurePercentage"),
            ),
            location_filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locationFilters"),
            ),
            management_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managementGroupId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parallel_deployments: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parallelDeployments"),
            ),
            policy_assignment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyAssignmentId"),
            ),
            policy_definition_reference_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyDefinitionReferenceId"),
            ),
            resource_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceCount"),
            ),
        }
    }
}
