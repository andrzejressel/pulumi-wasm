/// Provides an AWS Config Remediation Configuration.
///
/// > **Note:** Config Remediation Configuration requires an existing Config Rule to be present.
///
/// ## Example Usage
///
/// AWS managed rules can be used by setting the source owner to `AWS` and the source identifier to the name of the managed rule. More information about AWS managed rules can be found in the [AWS Config Developer Guide](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html).
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let this = rule::create(
///         "this",
///         RuleArgs::builder()
///             .name("example")
///             .source(
///                 RuleSource::builder()
///                     .owner("AWS")
///                     .sourceIdentifier("S3_BUCKET_VERSIONING_ENABLED")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let thisRemediationConfiguration = remediation_configuration::create(
///         "thisRemediationConfiguration",
///         RemediationConfigurationArgs::builder()
///             .automatic(true)
///             .config_rule_name("${this.name}")
///             .execution_controls(
///                 RemediationConfigurationExecutionControls::builder()
///                     .ssmControls(
///                         RemediationConfigurationExecutionControlsSsmControls::builder()
///                             .concurrentExecutionRatePercentage(25)
///                             .errorPercentage(20)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .maximum_automatic_attempts(10)
///             .parameters(
///                 vec![
///                     RemediationConfigurationParameter::builder()
///                     .name("AutomationAssumeRole")
///                     .staticValue("arn:aws:iam::875924563244:role/security_config")
///                     .build_struct(), RemediationConfigurationParameter::builder()
///                     .name("BucketName").resourceValue("RESOURCE_ID").build_struct(),
///                     RemediationConfigurationParameter::builder().name("SSEAlgorithm")
///                     .staticValue("AES256").build_struct(),
///                 ],
///             )
///             .resource_type("AWS::S3::Bucket")
///             .retry_attempt_seconds(600)
///             .target_id("AWS-EnableS3BucketEncryption")
///             .target_type("SSM_DOCUMENT")
///             .target_version("1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Remediation Configurations using the name config_rule_name. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/remediationConfiguration:RemediationConfiguration this example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod remediation_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RemediationConfigurationArgs {
        /// Remediation is triggered automatically if `true`.
        #[builder(into, default)]
        pub automatic: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the AWS Config rule.
        #[builder(into)]
        pub config_rule_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for execution controls. See below.
        #[builder(into, default)]
        pub execution_controls: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cfg::RemediationConfigurationExecutionControls>,
        >,
        /// Maximum number of failed attempts for auto-remediation. If you do not select a number, the default is 5.
        #[builder(into, default)]
        pub maximum_automatic_attempts: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Can be specified multiple times for each parameter. Each parameter block supports arguments below.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cfg::RemediationConfigurationParameter>>,
        >,
        /// Type of resource.
        #[builder(into, default)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Maximum time in seconds that AWS Config runs auto-remediation. If you do not select a number, the default is 60 seconds.
        #[builder(into, default)]
        pub retry_attempt_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Target ID is the name of the public document.
        #[builder(into)]
        pub target_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of the target. Target executes remediation. For example, SSM document.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the target. For example, version of the SSM document
        #[builder(into, default)]
        pub target_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RemediationConfigurationResult {
        /// ARN of the Config Remediation Configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Remediation is triggered automatically if `true`.
        pub automatic: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the AWS Config rule.
        pub config_rule_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for execution controls. See below.
        pub execution_controls: pulumi_gestalt_rust::Output<
            Option<super::super::types::cfg::RemediationConfigurationExecutionControls>,
        >,
        /// Maximum number of failed attempts for auto-remediation. If you do not select a number, the default is 5.
        pub maximum_automatic_attempts: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Can be specified multiple times for each parameter. Each parameter block supports arguments below.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cfg::RemediationConfigurationParameter>>,
        >,
        /// Type of resource.
        pub resource_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Maximum time in seconds that AWS Config runs auto-remediation. If you do not select a number, the default is 60 seconds.
        pub retry_attempt_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Target ID is the name of the public document.
        pub target_id: pulumi_gestalt_rust::Output<String>,
        /// Type of the target. Target executes remediation. For example, SSM document.
        ///
        /// The following arguments are optional:
        pub target_type: pulumi_gestalt_rust::Output<String>,
        /// Version of the target. For example, version of the SSM document
        pub target_version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RemediationConfigurationArgs,
    ) -> RemediationConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automatic_binding = args.automatic.get_output(context);
        let config_rule_name_binding = args.config_rule_name.get_output(context);
        let execution_controls_binding = args.execution_controls.get_output(context);
        let maximum_automatic_attempts_binding = args
            .maximum_automatic_attempts
            .get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let retry_attempt_seconds_binding = args
            .retry_attempt_seconds
            .get_output(context);
        let target_id_binding = args.target_id.get_output(context);
        let target_type_binding = args.target_type.get_output(context);
        let target_version_binding = args.target_version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/remediationConfiguration:RemediationConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automatic".into(),
                    value: &automatic_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configRuleName".into(),
                    value: &config_rule_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionControls".into(),
                    value: &execution_controls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maximumAutomaticAttempts".into(),
                    value: &maximum_automatic_attempts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retryAttemptSeconds".into(),
                    value: &retry_attempt_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetId".into(),
                    value: &target_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetType".into(),
                    value: &target_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetVersion".into(),
                    value: &target_version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RemediationConfigurationResult {
            arn: o.get_field("arn"),
            automatic: o.get_field("automatic"),
            config_rule_name: o.get_field("configRuleName"),
            execution_controls: o.get_field("executionControls"),
            maximum_automatic_attempts: o.get_field("maximumAutomaticAttempts"),
            parameters: o.get_field("parameters"),
            resource_type: o.get_field("resourceType"),
            retry_attempt_seconds: o.get_field("retryAttemptSeconds"),
            target_id: o.get_field("targetId"),
            target_type: o.get_field("targetType"),
            target_version: o.get_field("targetVersion"),
        }
    }
}
