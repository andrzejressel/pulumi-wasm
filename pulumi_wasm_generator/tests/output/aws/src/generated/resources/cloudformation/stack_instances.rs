/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = stack_instances::create(
///         "example",
///         StackInstancesArgs::builder()
///             .accounts(vec!["123456789012", "234567890123",])
///             .regions(vec!["us-east-1", "us-west-2",])
///             .stack_set_name("${exampleAwsCloudformationStackSet.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example IAM Setup in Target Account
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["${aWSCloudFormationStackSetAdministrationRole.arn}",])
///                     . type ("AWS").build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["cloudformation:*", "s3:*", "sns:*",]).effect("Allow")
///                     .resources(vec!["*",]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let aWSCloudFormationStackSetExecutionRole = role::create(
///         "aWSCloudFormationStackSetExecutionRole",
///         RoleArgs::builder()
///             .assume_role_policy(
///                 "${aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.json}",
///             )
///             .name("AWSCloudFormationStackSetExecutionRole")
///             .build_struct(),
///     );
///     let aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy = role_policy::create(
///         "aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy",
///         RolePolicyArgs::builder()
///             .name("MinimumExecutionPolicy")
///             .policy(
///                 "${aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.json}",
///             )
///             .role("${aWSCloudFormationStackSetExecutionRole.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example Deployment across Organizations account
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = stack_instances::create(
///         "example",
///         StackInstancesArgs::builder()
///             .deployment_targets(
///                 StackInstancesDeploymentTargets::builder()
///                     .organizationalUnitIds(
///                         vec!["${exampleAwsOrganizationsOrganization.roots[0].id}",],
///                     )
///                     .build_struct(),
///             )
///             .regions(vec!["us-west-2", "us-east-1",])
///             .stack_set_name("${exampleAwsCloudformationStackSet.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Import CloudFormation stack instances that target OUs, using the stack set name, `call_as`, and "OU" separated by commas (`,`). For example:
///
/// Using `pulumi import`, import CloudFormation stack instances using the stack set name and `call_as` separated by commas (`,`). If you are importing a stack instance targeting OUs, see the example below. For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/stackInstances:StackInstances example example,SELF
/// ```
/// Using `pulumi import`, Import CloudFormation stack instances that target OUs, using the stack set name, `call_as`, and "OU" separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/stackInstances:StackInstances example example,SELF,OU
/// ```
pub mod stack_instances {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StackInstancesArgs {
        /// Accounts where you want to create stack instances in the specified `regions`. You can specify either `accounts` or `deployment_targets`, but not both.
        #[builder(into, default)]
        pub accounts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
        #[builder(into, default)]
        pub call_as: pulumi_wasm_rust::Output<Option<String>>,
        /// AWS Organizations accounts for which to create stack instances in the `regions`. stack sets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for most of this argument. See deployment_targets below.
        #[builder(into, default)]
        pub deployment_targets: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudformation::StackInstancesDeploymentTargets>,
        >,
        /// Preferences for how AWS CloudFormation performs a stack set operation. See operation_preferences below.
        #[builder(into, default)]
        pub operation_preferences: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudformation::StackInstancesOperationPreferences,
            >,
        >,
        /// Key-value map of input parameters to override from the stack set for these instances. This argument's drift detection is limited to the first account and region since each instance can have unique parameters.
        #[builder(into, default)]
        pub parameter_overrides: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Regions where you want to create stack instances in the specified `accounts`.
        #[builder(into, default)]
        pub regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether to remove the stack instances from the stack set, but not delete the stacks. You can't reassociate a retained stack or add an existing, saved stack to a new stack set. To retain the stack, ensure `retain_stacks = true` has been successfully applied _before_ an apply that would destroy the resource. Defaults to `false`.
        #[builder(into, default)]
        pub retain_stacks: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the stack set.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub stack_set_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct StackInstancesResult {
        /// Accounts where you want to create stack instances in the specified `regions`. You can specify either `accounts` or `deployment_targets`, but not both.
        pub accounts: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
        pub call_as: pulumi_wasm_rust::Output<Option<String>>,
        /// AWS Organizations accounts for which to create stack instances in the `regions`. stack sets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for most of this argument. See deployment_targets below.
        pub deployment_targets: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudformation::StackInstancesDeploymentTargets>,
        >,
        /// Preferences for how AWS CloudFormation performs a stack set operation. See operation_preferences below.
        pub operation_preferences: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudformation::StackInstancesOperationPreferences,
            >,
        >,
        /// Key-value map of input parameters to override from the stack set for these instances. This argument's drift detection is limited to the first account and region since each instance can have unique parameters.
        pub parameter_overrides: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Regions where you want to create stack instances in the specified `accounts`.
        pub regions: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether to remove the stack instances from the stack set, but not delete the stacks. You can't reassociate a retained stack or add an existing, saved stack to a new stack set. To retain the stack, ensure `retain_stacks = true` has been successfully applied _before_ an apply that would destroy the resource. Defaults to `false`.
        pub retain_stacks: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of stack instances created from an organizational unit deployment target. This may not always be set depending on whether CloudFormation returns summaries for your configuration. See `stack_instance_summaries`.
        pub stack_instance_summaries: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudformation::StackInstancesStackInstanceSummary>,
        >,
        /// Name or unique ID of the stack set that the stack instance is associated with.
        pub stack_set_id: pulumi_wasm_rust::Output<String>,
        /// Name of the stack set.
        ///
        /// The following arguments are optional:
        pub stack_set_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StackInstancesArgs) -> StackInstancesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accounts_binding = args.accounts.get_inner();
        let call_as_binding = args.call_as.get_inner();
        let deployment_targets_binding = args.deployment_targets.get_inner();
        let operation_preferences_binding = args.operation_preferences.get_inner();
        let parameter_overrides_binding = args.parameter_overrides.get_inner();
        let regions_binding = args.regions.get_inner();
        let retain_stacks_binding = args.retain_stacks.get_inner();
        let stack_set_name_binding = args.stack_set_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudformation/stackInstances:StackInstances".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accounts".into(),
                    value: &accounts_binding,
                },
                register_interface::ObjectField {
                    name: "callAs".into(),
                    value: &call_as_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentTargets".into(),
                    value: &deployment_targets_binding,
                },
                register_interface::ObjectField {
                    name: "operationPreferences".into(),
                    value: &operation_preferences_binding,
                },
                register_interface::ObjectField {
                    name: "parameterOverrides".into(),
                    value: &parameter_overrides_binding,
                },
                register_interface::ObjectField {
                    name: "regions".into(),
                    value: &regions_binding,
                },
                register_interface::ObjectField {
                    name: "retainStacks".into(),
                    value: &retain_stacks_binding,
                },
                register_interface::ObjectField {
                    name: "stackSetName".into(),
                    value: &stack_set_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accounts".into(),
                },
                register_interface::ResultField {
                    name: "callAs".into(),
                },
                register_interface::ResultField {
                    name: "deploymentTargets".into(),
                },
                register_interface::ResultField {
                    name: "operationPreferences".into(),
                },
                register_interface::ResultField {
                    name: "parameterOverrides".into(),
                },
                register_interface::ResultField {
                    name: "regions".into(),
                },
                register_interface::ResultField {
                    name: "retainStacks".into(),
                },
                register_interface::ResultField {
                    name: "stackInstanceSummaries".into(),
                },
                register_interface::ResultField {
                    name: "stackSetId".into(),
                },
                register_interface::ResultField {
                    name: "stackSetName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StackInstancesResult {
            accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accounts").unwrap(),
            ),
            call_as: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("callAs").unwrap(),
            ),
            deployment_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentTargets").unwrap(),
            ),
            operation_preferences: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operationPreferences").unwrap(),
            ),
            parameter_overrides: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterOverrides").unwrap(),
            ),
            regions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regions").unwrap(),
            ),
            retain_stacks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retainStacks").unwrap(),
            ),
            stack_instance_summaries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackInstanceSummaries").unwrap(),
            ),
            stack_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackSetId").unwrap(),
            ),
            stack_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackSetName").unwrap(),
            ),
        }
    }
}