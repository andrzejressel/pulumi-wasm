/// Provides an AWS Config Remediation Configuration.
///
/// > **Note:** Config Remediation Configuration requires an existing Config Rule to be present.
///
/// ## Example Usage
///
/// AWS managed rules can be used by setting the source owner to `AWS` and the source identifier to the name of the managed rule. More information about AWS managed rules can be found in the [AWS Config Developer Guide](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html).
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod remediation_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RemediationConfigurationArgs {
        /// Remediation is triggered automatically if `true`.
        #[builder(into, default)]
        pub automatic: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the AWS Config rule.
        #[builder(into)]
        pub config_rule_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block for execution controls. See below.
        #[builder(into, default)]
        pub execution_controls: pulumi_wasm_rust::Output<
            Option<super::super::types::cfg::RemediationConfigurationExecutionControls>,
        >,
        /// Maximum number of failed attempts for auto-remediation. If you do not select a number, the default is 5.
        #[builder(into, default)]
        pub maximum_automatic_attempts: pulumi_wasm_rust::Output<Option<i32>>,
        /// Can be specified multiple times for each parameter. Each parameter block supports arguments below.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cfg::RemediationConfigurationParameter>>,
        >,
        /// Type of resource.
        #[builder(into, default)]
        pub resource_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum time in seconds that AWS Config runs auto-remediation. If you do not select a number, the default is 60 seconds.
        #[builder(into, default)]
        pub retry_attempt_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Target ID is the name of the public document.
        #[builder(into)]
        pub target_id: pulumi_wasm_rust::Output<String>,
        /// Type of the target. Target executes remediation. For example, SSM document.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target_type: pulumi_wasm_rust::Output<String>,
        /// Version of the target. For example, version of the SSM document
        #[builder(into, default)]
        pub target_version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RemediationConfigurationResult {
        /// ARN of the Config Remediation Configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Remediation is triggered automatically if `true`.
        pub automatic: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the AWS Config rule.
        pub config_rule_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block for execution controls. See below.
        pub execution_controls: pulumi_wasm_rust::Output<
            Option<super::super::types::cfg::RemediationConfigurationExecutionControls>,
        >,
        /// Maximum number of failed attempts for auto-remediation. If you do not select a number, the default is 5.
        pub maximum_automatic_attempts: pulumi_wasm_rust::Output<Option<i32>>,
        /// Can be specified multiple times for each parameter. Each parameter block supports arguments below.
        pub parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cfg::RemediationConfigurationParameter>>,
        >,
        /// Type of resource.
        pub resource_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum time in seconds that AWS Config runs auto-remediation. If you do not select a number, the default is 60 seconds.
        pub retry_attempt_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Target ID is the name of the public document.
        pub target_id: pulumi_wasm_rust::Output<String>,
        /// Type of the target. Target executes remediation. For example, SSM document.
        ///
        /// The following arguments are optional:
        pub target_type: pulumi_wasm_rust::Output<String>,
        /// Version of the target. For example, version of the SSM document
        pub target_version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: RemediationConfigurationArgs,
    ) -> RemediationConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automatic_binding = args.automatic.get_inner();
        let config_rule_name_binding = args.config_rule_name.get_inner();
        let execution_controls_binding = args.execution_controls.get_inner();
        let maximum_automatic_attempts_binding = args
            .maximum_automatic_attempts
            .get_inner();
        let parameters_binding = args.parameters.get_inner();
        let resource_type_binding = args.resource_type.get_inner();
        let retry_attempt_seconds_binding = args.retry_attempt_seconds.get_inner();
        let target_id_binding = args.target_id.get_inner();
        let target_type_binding = args.target_type.get_inner();
        let target_version_binding = args.target_version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cfg/remediationConfiguration:RemediationConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automatic".into(),
                    value: &automatic_binding,
                },
                register_interface::ObjectField {
                    name: "configRuleName".into(),
                    value: &config_rule_name_binding,
                },
                register_interface::ObjectField {
                    name: "executionControls".into(),
                    value: &execution_controls_binding,
                },
                register_interface::ObjectField {
                    name: "maximumAutomaticAttempts".into(),
                    value: &maximum_automatic_attempts_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding,
                },
                register_interface::ObjectField {
                    name: "retryAttemptSeconds".into(),
                    value: &retry_attempt_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "targetId".into(),
                    value: &target_id_binding,
                },
                register_interface::ObjectField {
                    name: "targetType".into(),
                    value: &target_type_binding,
                },
                register_interface::ObjectField {
                    name: "targetVersion".into(),
                    value: &target_version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "automatic".into(),
                },
                register_interface::ResultField {
                    name: "configRuleName".into(),
                },
                register_interface::ResultField {
                    name: "executionControls".into(),
                },
                register_interface::ResultField {
                    name: "maximumAutomaticAttempts".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "retryAttemptSeconds".into(),
                },
                register_interface::ResultField {
                    name: "targetId".into(),
                },
                register_interface::ResultField {
                    name: "targetType".into(),
                },
                register_interface::ResultField {
                    name: "targetVersion".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RemediationConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            automatic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automatic").unwrap(),
            ),
            config_rule_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configRuleName").unwrap(),
            ),
            execution_controls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionControls").unwrap(),
            ),
            maximum_automatic_attempts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumAutomaticAttempts").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            retry_attempt_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retryAttemptSeconds").unwrap(),
            ),
            target_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetId").unwrap(),
            ),
            target_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetType").unwrap(),
            ),
            target_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetVersion").unwrap(),
            ),
        }
    }
}