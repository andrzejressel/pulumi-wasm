/// Provides a CloudFormation Stack resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   network:
///     type: aws:cloudformation:Stack
///     properties:
///       name: networking-stack
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
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudformation Stacks using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudformation/stack:Stack stack networking-stack
/// ```
pub mod stack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StackArgs {
        /// A list of capabilities.
        /// Valid values: `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, or `CAPABILITY_AUTO_EXPAND`
        #[builder(into, default)]
        pub capabilities: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Set to true to disable rollback of the stack if stack creation failed.
        /// Conflicts with `on_failure`.
        #[builder(into, default)]
        pub disable_rollback: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ARN of an IAM role that AWS CloudFormation assumes to create the stack. If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.
        #[builder(into, default)]
        pub iam_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Stack name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of SNS topic ARNs to publish stack related events.
        #[builder(into, default)]
        pub notification_arns: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Action to be taken if stack creation fails. This must be
        /// one of: `DO_NOTHING`, `ROLLBACK`, or `DELETE`. Conflicts with `disable_rollback`.
        #[builder(into, default)]
        pub on_failure: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of Parameter structures that specify input parameters for the stack.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Structure containing the stack policy body.
        /// Conflicts w/ `policy_url`.
        #[builder(into, default)]
        pub policy_body: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Location of a file containing the stack policy.
        /// Conflicts w/ `policy_body`.
        #[builder(into, default)]
        pub policy_url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of resource tags to associate with this stack. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Structure containing the template body (max size: 51,200 bytes).
        #[builder(into, default)]
        pub template_body: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Location of a file containing the template body (max size: 460,800 bytes).
        #[builder(into, default)]
        pub template_url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The amount of time that can pass before the stack status becomes `CREATE_FAILED`.
        #[builder(into, default)]
        pub timeout_in_minutes: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct StackResult {
        /// A list of capabilities.
        /// Valid values: `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, or `CAPABILITY_AUTO_EXPAND`
        pub capabilities: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Set to true to disable rollback of the stack if stack creation failed.
        /// Conflicts with `on_failure`.
        pub disable_rollback: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of an IAM role that AWS CloudFormation assumes to create the stack. If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.
        pub iam_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Stack name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A list of SNS topic ARNs to publish stack related events.
        pub notification_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Action to be taken if stack creation fails. This must be
        /// one of: `DO_NOTHING`, `ROLLBACK`, or `DELETE`. Conflicts with `disable_rollback`.
        pub on_failure: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of outputs from the stack.
        pub outputs: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A map of Parameter structures that specify input parameters for the stack.
        pub parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Structure containing the stack policy body.
        /// Conflicts w/ `policy_url`.
        pub policy_body: pulumi_wasm_rust::Output<String>,
        /// Location of a file containing the stack policy.
        /// Conflicts w/ `policy_body`.
        pub policy_url: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of resource tags to associate with this stack. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Structure containing the template body (max size: 51,200 bytes).
        pub template_body: pulumi_wasm_rust::Output<String>,
        /// Location of a file containing the template body (max size: 460,800 bytes).
        pub template_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The amount of time that can pass before the stack status becomes `CREATE_FAILED`.
        pub timeout_in_minutes: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StackArgs,
    ) -> StackResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capabilities_binding = args.capabilities.get_output(context).get_inner();
        let disable_rollback_binding = args
            .disable_rollback
            .get_output(context)
            .get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let notification_arns_binding = args
            .notification_arns
            .get_output(context)
            .get_inner();
        let on_failure_binding = args.on_failure.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let policy_body_binding = args.policy_body.get_output(context).get_inner();
        let policy_url_binding = args.policy_url.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let template_body_binding = args.template_body.get_output(context).get_inner();
        let template_url_binding = args.template_url.get_output(context).get_inner();
        let timeout_in_minutes_binding = args
            .timeout_in_minutes
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudformation/stack:Stack".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capabilities".into(),
                    value: &capabilities_binding,
                },
                register_interface::ObjectField {
                    name: "disableRollback".into(),
                    value: &disable_rollback_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationArns".into(),
                    value: &notification_arns_binding,
                },
                register_interface::ObjectField {
                    name: "onFailure".into(),
                    value: &on_failure_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "policyBody".into(),
                    value: &policy_body_binding,
                },
                register_interface::ObjectField {
                    name: "policyUrl".into(),
                    value: &policy_url_binding,
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
                register_interface::ObjectField {
                    name: "timeoutInMinutes".into(),
                    value: &timeout_in_minutes_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StackResult {
            capabilities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capabilities"),
            ),
            disable_rollback: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disableRollback"),
            ),
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iamRoleArn"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            notification_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notificationArns"),
            ),
            on_failure: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("onFailure"),
            ),
            outputs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outputs"),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            policy_body: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyBody"),
            ),
            policy_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyUrl"),
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
            timeout_in_minutes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeoutInMinutes"),
            ),
        }
    }
}
