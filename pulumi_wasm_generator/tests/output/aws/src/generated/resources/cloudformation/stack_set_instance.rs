/// Manages a CloudFormation StackSet Instance. Instances are managed in the account and region of the StackSet after the target account permissions have been configured. Additional information about StackSets can be found in the [AWS CloudFormation User Guide](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/what-is-cfnstacksets.html).
///
/// > **NOTE:** All target accounts must have an IAM Role created that matches the name of the execution role configured in the StackSet (the `execution_role_name` argument in the `aws.cloudformation.StackSet` resource) in a trust relationship with the administrative account or administration IAM Role. The execution role must have appropriate permissions to manage resources defined in the template along with those required for StackSets to operate. See the [AWS CloudFormation User Guide](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html) for more details.
///
/// > **NOTE:** To retain the Stack during resource destroy, ensure `retain_stack` has been set to `true` in the state first. This must be completed _before_ a deployment that would destroy the resource.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = stack_set_instance::create(
///         "example",
///         StackSetInstanceArgs::builder()
///             .account_id("123456789012")
///             .region("us-east-1")
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
///     let example = stack_set_instance::create(
///         "example",
///         StackSetInstanceArgs::builder()
///             .deployment_targets(
///                 StackSetInstanceDeploymentTargets::builder()
///                     .organizationalUnitIds(
///                         vec!["${exampleAwsOrganizationsOrganization.roots[0].id}",],
///                     )
///                     .build_struct(),
///             )
///             .region("us-east-1")
///             .stack_set_name("${exampleAwsCloudformationStackSet.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Import CloudFormation StackSet Instances that target AWS Organizational Units using the StackSet name, a slash (`/`) separated list of organizational unit IDs, and target AWS Region separated by commas (`,`). For example:
///
/// Import CloudFormation StackSet Instances when acting a delegated administrator in a member account using the StackSet name, target AWS account ID or slash (`/`) separated list of organizational unit IDs, target AWS Region and `call_as` value separated by commas (`,`). For example:
///
/// Using `pulumi import`, import CloudFormation StackSet Instances that target an AWS Account ID using the StackSet name, target AWS account ID, and target AWS Region separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/stackSetInstance:StackSetInstance example example,123456789012,us-east-1
/// ```
/// Using `pulumi import`, import CloudFormation StackSet Instances that target AWS Organizational Units using the StackSet name, a slash (`/`) separated list of organizational unit IDs, and target AWS Region separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/stackSetInstance:StackSetInstance example example,ou-sdas-123123123/ou-sdas-789789789,us-east-1
/// ```
/// Using `pulumi import`, import CloudFormation StackSet Instances when acting a delegated administrator in a member account using the StackSet name, target AWS account ID or slash (`/`) separated list of organizational unit IDs, target AWS Region and `call_as` value separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/stackSetInstance:StackSetInstance example example,ou-sdas-123123123/ou-sdas-789789789,us-east-1,DELEGATED_ADMIN
/// ```
pub mod stack_set_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StackSetInstanceArgs {
        /// Target AWS Account ID to create a Stack based on the StackSet. Defaults to current account.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
        #[builder(into, default)]
        pub call_as: pulumi_wasm_rust::Output<Option<String>>,
        /// AWS Organizations accounts to which StackSets deploys. StackSets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for this argument. See deployment_targets below.
        #[builder(into, default)]
        pub deployment_targets: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudformation::StackSetInstanceDeploymentTargets,
            >,
        >,
        /// Preferences for how AWS CloudFormation performs a stack set operation.
        #[builder(into, default)]
        pub operation_preferences: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudformation::StackSetInstanceOperationPreferences,
            >,
        >,
        /// Key-value map of input parameters to override from the StackSet for this Instance.
        #[builder(into, default)]
        pub parameter_overrides: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Target AWS Region to create a Stack based on the StackSet. Defaults to current region.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// During resource destroy, remove Instance from StackSet while keeping the Stack and its associated resources. Must be enabled in the state _before_ destroy operation to take effect. You cannot reassociate a retained Stack or add an existing, saved Stack to a new StackSet. Defaults to `false`.
        #[builder(into, default)]
        pub retain_stack: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the StackSet.
        #[builder(into)]
        pub stack_set_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct StackSetInstanceResult {
        /// Target AWS Account ID to create a Stack based on the StackSet. Defaults to current account.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Specifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
        pub call_as: pulumi_wasm_rust::Output<Option<String>>,
        /// AWS Organizations accounts to which StackSets deploys. StackSets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for this argument. See deployment_targets below.
        pub deployment_targets: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudformation::StackSetInstanceDeploymentTargets,
            >,
        >,
        /// Preferences for how AWS CloudFormation performs a stack set operation.
        pub operation_preferences: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudformation::StackSetInstanceOperationPreferences,
            >,
        >,
        /// Organizational unit ID in which the stack is deployed.
        pub organizational_unit_id: pulumi_wasm_rust::Output<String>,
        /// Key-value map of input parameters to override from the StackSet for this Instance.
        pub parameter_overrides: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Target AWS Region to create a Stack based on the StackSet. Defaults to current region.
        pub region: pulumi_wasm_rust::Output<String>,
        /// During resource destroy, remove Instance from StackSet while keeping the Stack and its associated resources. Must be enabled in the state _before_ destroy operation to take effect. You cannot reassociate a retained Stack or add an existing, saved Stack to a new StackSet. Defaults to `false`.
        pub retain_stack: pulumi_wasm_rust::Output<Option<bool>>,
        /// Stack identifier.
        pub stack_id: pulumi_wasm_rust::Output<String>,
        /// List of stack instances created from an organizational unit deployment target. This will only be populated when `deployment_targets` is set. See `stack_instance_summaries`.
        pub stack_instance_summaries: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::cloudformation::StackSetInstanceStackInstanceSummary,
            >,
        >,
        /// Name of the StackSet.
        pub stack_set_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StackSetInstanceArgs) -> StackSetInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let call_as_binding = args.call_as.get_inner();
        let deployment_targets_binding = args.deployment_targets.get_inner();
        let operation_preferences_binding = args.operation_preferences.get_inner();
        let parameter_overrides_binding = args.parameter_overrides.get_inner();
        let region_binding = args.region.get_inner();
        let retain_stack_binding = args.retain_stack.get_inner();
        let stack_set_name_binding = args.stack_set_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudformation/stackSetInstance:StackSetInstance".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
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
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "retainStack".into(),
                    value: &retain_stack_binding,
                },
                register_interface::ObjectField {
                    name: "stackSetName".into(),
                    value: &stack_set_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
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
                    name: "organizationalUnitId".into(),
                },
                register_interface::ResultField {
                    name: "parameterOverrides".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "retainStack".into(),
                },
                register_interface::ResultField {
                    name: "stackId".into(),
                },
                register_interface::ResultField {
                    name: "stackInstanceSummaries".into(),
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
        StackSetInstanceResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
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
            organizational_unit_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationalUnitId").unwrap(),
            ),
            parameter_overrides: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterOverrides").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            retain_stack: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retainStack").unwrap(),
            ),
            stack_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackId").unwrap(),
            ),
            stack_instance_summaries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackInstanceSummaries").unwrap(),
            ),
            stack_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackSetName").unwrap(),
            ),
        }
    }
}