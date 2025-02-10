/// Provides a WAF Regional Web ACL Resource for use with Application Load Balancer.
///
/// ## Example Usage
///
/// ### Regular Rule
///
/// ```yaml
/// resources:
///   ipset:
///     type: aws:wafregional:IpSet
///     properties:
///       name: tfIPSet
///       ipSetDescriptors:
///         - type: IPV4
///           value: 192.0.7.0/24
///   wafrule:
///     type: aws:wafregional:Rule
///     properties:
///       name: tfWAFRule
///       metricName: tfWAFRule
///       predicates:
///         - dataId: ${ipset.id}
///           negated: false
///           type: IPMatch
///   wafacl:
///     type: aws:wafregional:WebAcl
///     properties:
///       name: tfWebACL
///       metricName: tfWebACL
///       defaultAction:
///         type: ALLOW
///       rules:
///         - action:
///             type: BLOCK
///           priority: 1
///           ruleId: ${wafrule.id}
///           type: REGULAR
/// ```
///
/// ### Group Rule
///
/// ```yaml
/// resources:
///   example:
///     type: aws:wafregional:WebAcl
///     properties:
///       name: example
///       metricName: example
///       defaultAction:
///         type: ALLOW
///       rules:
///         - priority: 1
///           ruleId: ${exampleAwsWafregionalRuleGroup.id}
///           type: GROUP
///           overrideAction:
///             type: NONE
/// ```
///
/// ### Logging
///
/// > *NOTE:* The Kinesis Firehose Delivery Stream name must begin with `aws-waf-logs-`. See the [AWS WAF Developer Guide](https://docs.aws.amazon.com/waf/latest/developerguide/logging.html) for more information about enabling WAF logging.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = web_acl::create(
///         "example",
///         WebAclArgs::builder()
///             .logging_configuration(
///                 WebAclLoggingConfiguration::builder()
///                     .logDestination("${exampleAwsKinesisFirehoseDeliveryStream.arn}")
///                     .redactedFields(
///                         WebAclLoggingConfigurationRedactedFields::builder()
///                             .fieldToMatches(
///                                 vec![
///                                     WebAclLoggingConfigurationRedactedFieldsFieldToMatch::builder()
///                                     . type ("URI").build_struct(),
///                                     WebAclLoggingConfigurationRedactedFieldsFieldToMatch::builder()
///                                     .data("referer"). type ("HEADER").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Regional Web ACL using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/webAcl:WebAcl wafacl a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_acl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAclArgs {
        /// The action that you want AWS WAF Regional to take when a request doesn't match the criteria in any of the rules that are associated with the web ACL.
        #[builder(into)]
        pub default_action: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::wafregional::WebAclDefaultAction,
        >,
        /// Configuration block to enable WAF logging. Detailed below.
        #[builder(into, default)]
        pub logging_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::wafregional::WebAclLoggingConfiguration>,
        >,
        /// The name or description for the Amazon CloudWatch metric of this web ACL.
        #[builder(into)]
        pub metric_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name or description of the web ACL.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of configuration blocks containing rules for the web ACL. Detailed below.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafregional::WebAclRule>>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WebAclResult {
        /// Amazon Resource Name (ARN) of the WAF Regional WebACL.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The action that you want AWS WAF Regional to take when a request doesn't match the criteria in any of the rules that are associated with the web ACL.
        pub default_action: pulumi_gestalt_rust::Output<
            super::super::types::wafregional::WebAclDefaultAction,
        >,
        /// Configuration block to enable WAF logging. Detailed below.
        pub logging_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::wafregional::WebAclLoggingConfiguration>,
        >,
        /// The name or description for the Amazon CloudWatch metric of this web ACL.
        pub metric_name: pulumi_gestalt_rust::Output<String>,
        /// The name or description of the web ACL.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Set of configuration blocks containing rules for the web ACL. Detailed below.
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafregional::WebAclRule>>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebAclArgs,
    ) -> WebAclResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_action_binding = args.default_action.get_output(context);
        let logging_configuration_binding = args
            .logging_configuration
            .get_output(context);
        let metric_name_binding = args.metric_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafregional/webAcl:WebAcl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultAction".into(),
                    value: default_action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingConfiguration".into(),
                    value: logging_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricName".into(),
                    value: metric_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebAclResult {
            arn: o.get_field("arn"),
            default_action: o.get_field("defaultAction"),
            logging_configuration: o.get_field("loggingConfiguration"),
            metric_name: o.get_field("metricName"),
            name: o.get_field("name"),
            rules: o.get_field("rules"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
