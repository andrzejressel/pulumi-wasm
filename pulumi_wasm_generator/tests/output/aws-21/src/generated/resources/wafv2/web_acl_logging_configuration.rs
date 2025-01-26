/// This resource creates a WAFv2 Web ACL Logging Configuration.
///
/// !> **WARNING:** When logging from a WAFv2 Web ACL to a CloudWatch Log Group, the WAFv2 service tries to create or update a generic Log Resource Policy named `AWSWAF-LOGS`. However, if there are a large number of Web ACLs or if the account frequently creates and deletes Web ACLs, this policy may exceed the maximum policy size. As a result, this resource type will fail to be created. More details about this issue can be found in this issue. To prevent this issue, you can manage a specific resource policy. Please refer to the example below for managing a CloudWatch Log Group with a managed CloudWatch Log Resource Policy.
///
/// ## Example Usage
///
/// ### With Redacted Fields
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = web_acl_logging_configuration::create(
///         "example",
///         WebAclLoggingConfigurationArgs::builder()
///             .log_destination_configs(
///                 vec!["${exampleAwsKinesisFirehoseDeliveryStream.arn}",],
///             )
///             .redacted_fields(
///                 vec![
///                     WebAclLoggingConfigurationRedactedField::builder()
///                     .singleHeader(WebAclLoggingConfigurationRedactedFieldSingleHeader::builder()
///                     .name("user-agent").build_struct()).build_struct(),
///                 ],
///             )
///             .resource_arn("${exampleAwsWafv2WebAcl.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Logging Filter
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = web_acl_logging_configuration::create(
///         "example",
///         WebAclLoggingConfigurationArgs::builder()
///             .log_destination_configs(
///                 vec!["${exampleAwsKinesisFirehoseDeliveryStream.arn}",],
///             )
///             .logging_filter(
///                 WebAclLoggingConfigurationLoggingFilter::builder()
///                     .defaultBehavior("KEEP")
///                     .filters(
///                         vec![
///                             WebAclLoggingConfigurationLoggingFilterFilter::builder()
///                             .behavior("DROP")
///                             .conditions(vec![WebAclLoggingConfigurationLoggingFilterFilterCondition::builder()
///                             .actionCondition(WebAclLoggingConfigurationLoggingFilterFilterConditionActionCondition::builder()
///                             .action("COUNT").build_struct()).build_struct(),
///                             WebAclLoggingConfigurationLoggingFilterFilterCondition::builder()
///                             .labelNameCondition(WebAclLoggingConfigurationLoggingFilterFilterConditionLabelNameCondition::builder()
///                             .labelName("awswaf:111122223333:rulegroup:testRules:LabelNameZ")
///                             .build_struct()).build_struct(),]).requirement("MEETS_ALL")
///                             .build_struct(),
///                             WebAclLoggingConfigurationLoggingFilterFilter::builder()
///                             .behavior("KEEP")
///                             .conditions(vec![WebAclLoggingConfigurationLoggingFilterFilterCondition::builder()
///                             .actionCondition(WebAclLoggingConfigurationLoggingFilterFilterConditionActionCondition::builder()
///                             .action("ALLOW").build_struct()).build_struct(),])
///                             .requirement("MEETS_ANY").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .resource_arn("${exampleAwsWafv2WebAcl.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAFv2 Web ACL Logging Configurations using the ARN of the WAFv2 Web ACL. For example:
///
/// ```sh
/// $ pulumi import aws:wafv2/webAclLoggingConfiguration:WebAclLoggingConfiguration example arn:aws:wafv2:us-west-2:123456789012:regional/webacl/test-logs/a1b2c3d4-5678-90ab-cdef
/// ```
pub mod web_acl_logging_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAclLoggingConfigurationArgs {
        /// Configuration block that allows you to associate Amazon Kinesis Data Firehose, Cloudwatch Log log group, or S3 bucket Amazon Resource Names (ARNs) with the web ACL. **Note:** data firehose, log group, or bucket name **must** be prefixed with `aws-waf-logs-`, e.g. `aws-waf-logs-example-firehose`, `aws-waf-logs-example-log-group`, or `aws-waf-logs-example-bucket`.
        #[builder(into)]
        pub log_destination_configs: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Configuration block that specifies which web requests are kept in the logs and which are dropped. It allows filtering based on the rule action and the web request labels applied by matching rules during web ACL evaluation. For more details, refer to the Logging Filter section below.
        #[builder(into, default)]
        pub logging_filter: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilter>,
        >,
        /// Configuration for parts of the request that you want to keep out of the logs. Up to 100 `redacted_fields` blocks are supported. See Redacted Fields below for more details.
        #[builder(into, default)]
        pub redacted_fields: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::wafv2::WebAclLoggingConfigurationRedactedField>,
            >,
        >,
        /// Amazon Resource Name (ARN) of the web ACL that you want to associate with `log_destination_configs`.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebAclLoggingConfigurationResult {
        /// Configuration block that allows you to associate Amazon Kinesis Data Firehose, Cloudwatch Log log group, or S3 bucket Amazon Resource Names (ARNs) with the web ACL. **Note:** data firehose, log group, or bucket name **must** be prefixed with `aws-waf-logs-`, e.g. `aws-waf-logs-example-firehose`, `aws-waf-logs-example-log-group`, or `aws-waf-logs-example-bucket`.
        pub log_destination_configs: pulumi_wasm_rust::Output<Vec<String>>,
        /// Configuration block that specifies which web requests are kept in the logs and which are dropped. It allows filtering based on the rule action and the web request labels applied by matching rules during web ACL evaluation. For more details, refer to the Logging Filter section below.
        pub logging_filter: pulumi_wasm_rust::Output<
            Option<super::super::types::wafv2::WebAclLoggingConfigurationLoggingFilter>,
        >,
        /// Configuration for parts of the request that you want to keep out of the logs. Up to 100 `redacted_fields` blocks are supported. See Redacted Fields below for more details.
        pub redacted_fields: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::wafv2::WebAclLoggingConfigurationRedactedField>,
            >,
        >,
        /// Amazon Resource Name (ARN) of the web ACL that you want to associate with `log_destination_configs`.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WebAclLoggingConfigurationArgs,
    ) -> WebAclLoggingConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let log_destination_configs_binding = args
            .log_destination_configs
            .get_output(context)
            .get_inner();
        let logging_filter_binding = args.logging_filter.get_output(context).get_inner();
        let redacted_fields_binding = args
            .redacted_fields
            .get_output(context)
            .get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafv2/webAclLoggingConfiguration:WebAclLoggingConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logDestinationConfigs".into(),
                    value: &log_destination_configs_binding,
                },
                register_interface::ObjectField {
                    name: "loggingFilter".into(),
                    value: &logging_filter_binding,
                },
                register_interface::ObjectField {
                    name: "redactedFields".into(),
                    value: &redacted_fields_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WebAclLoggingConfigurationResult {
            log_destination_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logDestinationConfigs"),
            ),
            logging_filter: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loggingFilter"),
            ),
            redacted_fields: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("redactedFields"),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
        }
    }
}
