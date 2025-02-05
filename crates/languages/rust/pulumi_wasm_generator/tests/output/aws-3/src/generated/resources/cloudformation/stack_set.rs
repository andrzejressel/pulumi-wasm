/// Manages a CloudFormation StackSet. StackSets allow CloudFormation templates to be easily deployed across multiple accounts and regions via StackSet Instances (`aws.cloudformation.StackSetInstance` resource). Additional information about StackSets can be found in the [AWS CloudFormation User Guide](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/what-is-cfnstacksets.html).
///
/// > **NOTE:** All template parameters, including those with a `Default`, must be configured or ignored with the `lifecycle` configuration block `ignore_changes` argument.
///
/// > **NOTE:** All `NoEcho` template parameters must be ignored with the `lifecycle` configuration block `ignore_changes` argument.
///
/// > **NOTE:** When using a delegated administrator account, ensure that your IAM User or Role has the `organizations:ListDelegatedAdministrators` permission. Otherwise, you may get an error like `ValidationError: Account used is not a delegated administrator`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   aWSCloudFormationStackSetAdministrationRole:
///     type: aws:iam:Role
///     name: AWSCloudFormationStackSetAdministrationRole
///     properties:
///       assumeRolePolicy: ${aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy.json}
///       name: AWSCloudFormationStackSetAdministrationRole
///   example:
///     type: aws:cloudformation:StackSet
///     properties:
///       administrationRoleArn: ${aWSCloudFormationStackSetAdministrationRole.arn}
///       name: example
///       parameters:
///         VPCCidr: 10.0.0.0/16
///       templateBody:
///         fn::toJSON:
///           Parameters:
///             VPCCidr:
///               Type: String
///               Default: 10.0.0.0/16
///               Description: Enter the CIDR block for the VPC. Default is 10.0.0.0/16.
///           Resources:
///             myVpc:
///               Type: AWS::EC2::VPC
///               Properties:
///                 CidrBlock:
///                   Ref: VPCCidr
///                 Tags:
///                   - Key: Name
///                     Value: Primary_CF_VPC
///   aWSCloudFormationStackSetAdministrationRoleExecutionPolicyRolePolicy:
///     type: aws:iam:RolePolicy
///     name: AWSCloudFormationStackSetAdministrationRole_ExecutionPolicy
///     properties:
///       name: ExecutionPolicy
///       policy: ${aWSCloudFormationStackSetAdministrationRoleExecutionPolicy.json}
///       role: ${aWSCloudFormationStackSetAdministrationRole.name}
/// variables:
///   aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             effect: Allow
///             principals:
///               - identifiers:
///                   - cloudformation.amazonaws.com
///                 type: Service
///   aWSCloudFormationStackSetAdministrationRoleExecutionPolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             effect: Allow
///             resources:
///               - arn:aws:iam::*:role/${example.executionRoleName}
/// ```
///
/// ## Import
///
/// Import CloudFormation StackSets when acting a delegated administrator in a member account using the `name` and `call_as` values separated by a comma (`,`). For example:
///
/// Using `pulumi import`, import CloudFormation StackSets using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/stackSet:StackSet example example
/// ```
/// Using `pulumi import`, import CloudFormation StackSets when acting a delegated administrator in a member account using the `name` and `call_as` values separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/stackSet:StackSet example example,DELEGATED_ADMIN
/// ```
pub mod stack_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StackSetArgs {
        /// Amazon Resource Number (ARN) of the IAM Role in the administrator account. This must be defined when using the `SELF_MANAGED` permission model.
        #[builder(into, default)]
        pub administration_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block containing the auto-deployment model for your StackSet. This can only be defined when using the `SERVICE_MANAGED` permission model.
        #[builder(into, default)]
        pub auto_deployment: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudformation::StackSetAutoDeployment>,
        >,
        /// Specifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
        #[builder(into, default)]
        pub call_as: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of capabilities. Valid values: `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, `CAPABILITY_AUTO_EXPAND`.
        #[builder(into, default)]
        pub capabilities: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Description of the StackSet.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the IAM Role in all target accounts for StackSet operations. Defaults to `AWSCloudFormationStackSetExecutionRole` when using the `SELF_MANAGED` permission model. This should not be defined when using the `SERVICE_MANAGED` permission model.
        #[builder(into, default)]
        pub execution_role_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block to allow StackSets to perform non-conflicting operations concurrently and queues conflicting operations.
        #[builder(into, default)]
        pub managed_execution: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudformation::StackSetManagedExecution>,
        >,
        /// Name of the StackSet. The name must be unique in the region where you create your StackSet. The name can contain only alphanumeric characters (case-sensitive) and hyphens. It must start with an alphabetic character and cannot be longer than 128 characters.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Preferences for how AWS CloudFormation performs a stack set update.
        #[builder(into, default)]
        pub operation_preferences: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudformation::StackSetOperationPreferences>,
        >,
        /// Key-value map of input parameters for the StackSet template. All template parameters, including those with a `Default`, must be configured or ignored with `lifecycle` configuration block `ignore_changes` argument. All `NoEcho` template parameters must be ignored with the `lifecycle` configuration block `ignore_changes` argument.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Describes how the IAM roles required for your StackSet are created. Valid values: `SELF_MANAGED` (default), `SERVICE_MANAGED`.
        #[builder(into, default)]
        pub permission_model: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value map of tags to associate with this StackSet and the Stacks created from it. AWS CloudFormation also propagates these tags to supported resources that are created in the Stacks. A maximum number of 50 tags can be specified. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// String containing the CloudFormation template body. Maximum size: 51,200 bytes. Conflicts with `template_url`.
        #[builder(into, default)]
        pub template_body: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// String containing the location of a file containing the CloudFormation template body. The URL must point to a template that is located in an Amazon S3 bucket. Maximum location file size: 460,800 bytes. Conflicts with `template_body`.
        #[builder(into, default)]
        pub template_url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StackSetResult {
        /// Amazon Resource Number (ARN) of the IAM Role in the administrator account. This must be defined when using the `SELF_MANAGED` permission model.
        pub administration_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the StackSet.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block containing the auto-deployment model for your StackSet. This can only be defined when using the `SERVICE_MANAGED` permission model.
        pub auto_deployment: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudformation::StackSetAutoDeployment>,
        >,
        /// Specifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
        pub call_as: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of capabilities. Valid values: `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, `CAPABILITY_AUTO_EXPAND`.
        pub capabilities: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Description of the StackSet.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the IAM Role in all target accounts for StackSet operations. Defaults to `AWSCloudFormationStackSetExecutionRole` when using the `SELF_MANAGED` permission model. This should not be defined when using the `SERVICE_MANAGED` permission model.
        pub execution_role_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block to allow StackSets to perform non-conflicting operations concurrently and queues conflicting operations.
        pub managed_execution: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudformation::StackSetManagedExecution>,
        >,
        /// Name of the StackSet. The name must be unique in the region where you create your StackSet. The name can contain only alphanumeric characters (case-sensitive) and hyphens. It must start with an alphabetic character and cannot be longer than 128 characters.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Preferences for how AWS CloudFormation performs a stack set update.
        pub operation_preferences: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudformation::StackSetOperationPreferences>,
        >,
        /// Key-value map of input parameters for the StackSet template. All template parameters, including those with a `Default`, must be configured or ignored with `lifecycle` configuration block `ignore_changes` argument. All `NoEcho` template parameters must be ignored with the `lifecycle` configuration block `ignore_changes` argument.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Describes how the IAM roles required for your StackSet are created. Valid values: `SELF_MANAGED` (default), `SERVICE_MANAGED`.
        pub permission_model: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique identifier of the StackSet.
        pub stack_set_id: pulumi_wasm_rust::Output<String>,
        /// Key-value map of tags to associate with this StackSet and the Stacks created from it. AWS CloudFormation also propagates these tags to supported resources that are created in the Stacks. A maximum number of 50 tags can be specified. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// String containing the CloudFormation template body. Maximum size: 51,200 bytes. Conflicts with `template_url`.
        pub template_body: pulumi_wasm_rust::Output<String>,
        /// String containing the location of a file containing the CloudFormation template body. The URL must point to a template that is located in an Amazon S3 bucket. Maximum location file size: 460,800 bytes. Conflicts with `template_body`.
        pub template_url: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StackSetArgs,
    ) -> StackSetResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let administration_role_arn_binding = args
            .administration_role_arn
            .get_output(context)
            .get_inner();
        let auto_deployment_binding = args
            .auto_deployment
            .get_output(context)
            .get_inner();
        let call_as_binding = args.call_as.get_output(context).get_inner();
        let capabilities_binding = args.capabilities.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let execution_role_name_binding = args
            .execution_role_name
            .get_output(context)
            .get_inner();
        let managed_execution_binding = args
            .managed_execution
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let operation_preferences_binding = args
            .operation_preferences
            .get_output(context)
            .get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let permission_model_binding = args
            .permission_model
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let template_body_binding = args.template_body.get_output(context).get_inner();
        let template_url_binding = args.template_url.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudformation/stackSet:StackSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "administrationRoleArn".into(),
                    value: &administration_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "autoDeployment".into(),
                    value: &auto_deployment_binding,
                },
                register_interface::ObjectField {
                    name: "callAs".into(),
                    value: &call_as_binding,
                },
                register_interface::ObjectField {
                    name: "capabilities".into(),
                    value: &capabilities_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "executionRoleName".into(),
                    value: &execution_role_name_binding,
                },
                register_interface::ObjectField {
                    name: "managedExecution".into(),
                    value: &managed_execution_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "operationPreferences".into(),
                    value: &operation_preferences_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "permissionModel".into(),
                    value: &permission_model_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "templateBody".into(),
                    value: &template_body_binding,
                },
                register_interface::ObjectField {
                    name: "templateUrl".into(),
                    value: &template_url_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StackSetResult {
            administration_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("administrationRoleArn"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            auto_deployment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoDeployment"),
            ),
            call_as: pulumi_wasm_rust::__private::into_domain(o.extract_field("callAs")),
            capabilities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capabilities"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            execution_role_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("executionRoleName"),
            ),
            managed_execution: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managedExecution"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            operation_preferences: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("operationPreferences"),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            permission_model: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permissionModel"),
            ),
            stack_set_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stackSetId"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            template_body: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("templateBody"),
            ),
            template_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("templateUrl"),
            ),
        }
    }
}
