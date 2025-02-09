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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// ```yaml
/// resources:
///   aWSCloudFormationStackSetExecutionRole:
///     type: aws:iam:Role
///     name: AWSCloudFormationStackSetExecutionRole
///     properties:
///       assumeRolePolicy: ${aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.json}
///       name: AWSCloudFormationStackSetExecutionRole
///   aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy:
///     type: aws:iam:RolePolicy
///     name: AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy
///     properties:
///       name: MinimumExecutionPolicy
///       policy: ${aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.json}
///       role: ${aWSCloudFormationStackSetExecutionRole.name}
/// variables:
///   aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             effect: Allow
///             principals:
///               - identifiers:
///                   - ${aWSCloudFormationStackSetAdministrationRole.arn}
///                 type: AWS
///   # Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
///   # Additional IAM permissions necessary depend on the resources defined in the StackSet template
///   aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - cloudformation:*
///               - s3:*
///               - sns:*
///             effect: Allow
///             resources:
///               - '*'
/// ```
///
/// ### Example Deployment across Organizations account
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stack_set_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StackSetInstanceArgs {
        /// Target AWS Account ID to create a Stack based on the StackSet. Defaults to current account.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
        #[builder(into, default)]
        pub call_as: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS Organizations accounts to which StackSets deploys. StackSets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for this argument. See deployment_targets below.
        #[builder(into, default)]
        pub deployment_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cloudformation::StackSetInstanceDeploymentTargets,
            >,
        >,
        /// Preferences for how AWS CloudFormation performs a stack set operation.
        #[builder(into, default)]
        pub operation_preferences: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::cloudformation::StackSetInstanceOperationPreferences,
            >,
        >,
        /// Key-value map of input parameters to override from the StackSet for this Instance.
        #[builder(into, default)]
        pub parameter_overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Target AWS Region to create a Stack based on the StackSet. Defaults to current region.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// During resource destroy, remove Instance from StackSet while keeping the Stack and its associated resources. Must be enabled in the state _before_ destroy operation to take effect. You cannot reassociate a retained Stack or add an existing, saved Stack to a new StackSet. Defaults to `false`.
        #[builder(into, default)]
        pub retain_stack: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the StackSet.
        #[builder(into)]
        pub stack_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StackSetInstanceResult {
        /// Target AWS Account ID to create a Stack based on the StackSet. Defaults to current account.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
        pub call_as: pulumi_gestalt_rust::Output<Option<String>>,
        /// AWS Organizations accounts to which StackSets deploys. StackSets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for this argument. See deployment_targets below.
        pub deployment_targets: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cloudformation::StackSetInstanceDeploymentTargets,
            >,
        >,
        /// Preferences for how AWS CloudFormation performs a stack set operation.
        pub operation_preferences: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::cloudformation::StackSetInstanceOperationPreferences,
            >,
        >,
        /// Organizational unit ID in which the stack is deployed.
        pub organizational_unit_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of input parameters to override from the StackSet for this Instance.
        pub parameter_overrides: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Target AWS Region to create a Stack based on the StackSet. Defaults to current region.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// During resource destroy, remove Instance from StackSet while keeping the Stack and its associated resources. Must be enabled in the state _before_ destroy operation to take effect. You cannot reassociate a retained Stack or add an existing, saved Stack to a new StackSet. Defaults to `false`.
        pub retain_stack: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Stack identifier.
        pub stack_id: pulumi_gestalt_rust::Output<String>,
        /// List of stack instances created from an organizational unit deployment target. This will only be populated when `deployment_targets` is set. See `stack_instance_summaries`.
        pub stack_instance_summaries: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::cloudformation::StackSetInstanceStackInstanceSummary,
            >,
        >,
        /// Name of the StackSet.
        pub stack_set_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: StackSetInstanceArgs,
    ) -> StackSetInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let call_as_binding_1 = args.call_as.get_output(context);
        let call_as_binding = call_as_binding_1.get_inner();
        let deployment_targets_binding_1 = args.deployment_targets.get_output(context);
        let deployment_targets_binding = deployment_targets_binding_1.get_inner();
        let operation_preferences_binding_1 = args
            .operation_preferences
            .get_output(context);
        let operation_preferences_binding = operation_preferences_binding_1.get_inner();
        let parameter_overrides_binding_1 = args.parameter_overrides.get_output(context);
        let parameter_overrides_binding = parameter_overrides_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let retain_stack_binding_1 = args.retain_stack.get_output(context);
        let retain_stack_binding = retain_stack_binding_1.get_inner();
        let stack_set_name_binding_1 = args.stack_set_name.get_output(context);
        let stack_set_name_binding = stack_set_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudformation/stackSetInstance:StackSetInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        StackSetInstanceResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            call_as: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("callAs"),
            ),
            deployment_targets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deploymentTargets"),
            ),
            operation_preferences: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("operationPreferences"),
            ),
            organizational_unit_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("organizationalUnitId"),
            ),
            parameter_overrides: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameterOverrides"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            retain_stack: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retainStack"),
            ),
            stack_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stackId"),
            ),
            stack_instance_summaries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stackInstanceSummaries"),
            ),
            stack_set_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stackSetName"),
            ),
        }
    }
}
