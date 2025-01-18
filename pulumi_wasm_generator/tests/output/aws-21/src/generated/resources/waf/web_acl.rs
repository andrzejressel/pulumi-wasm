/// Provides a WAF Web ACL Resource
///
/// ## Example Usage
///
/// This example blocks requests coming from `192.0.7.0/24` and allows everything else.
///
/// ```yaml
/// resources:
///   ipset:
///     type: aws:waf:IpSet
///     properties:
///       name: tfIPSet
///       ipSetDescriptors:
///         - type: IPV4
///           value: 192.0.7.0/24
///   wafrule:
///     type: aws:waf:Rule
///     properties:
///       name: tfWAFRule
///       metricName: tfWAFRule
///       predicates:
///         - dataId: ${ipset.id}
///           negated: false
///           type: IPMatch
///     options:
///       dependsOn:
///         - ${ipset}
///   wafAcl:
///     type: aws:waf:WebAcl
///     name: waf_acl
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
///     options:
///       dependsOn:
///         - ${ipset}
///         - ${wafrule}
/// ```
///
/// ### Logging
///
/// > *NOTE:* The Kinesis Firehose Delivery Stream name must begin with `aws-waf-logs-` and be located in `us-east-1` region. See the [AWS WAF Developer Guide](https://docs.aws.amazon.com/waf/latest/developerguide/logging.html) for more information about enabling WAF logging.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// Using `pulumi import`, import WAF Web ACL using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:waf/webAcl:WebAcl main 0c8e583e-18f3-4c13-9e2a-67c4805d2f94
/// ```
pub mod web_acl {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAclArgs {
        /// Configuration block with action that you want AWS WAF to take when a request doesn't match the criteria in any of the rules that are associated with the web ACL. Detailed below.
        #[builder(into)]
        pub default_action: pulumi_wasm_rust::Output<
            super::super::types::waf::WebAclDefaultAction,
        >,
        /// Configuration block to enable WAF logging. Detailed below.
        #[builder(into, default)]
        pub logging_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::waf::WebAclLoggingConfiguration>,
        >,
        /// The name or description for the Amazon CloudWatch metric of this web ACL.
        #[builder(into)]
        pub metric_name: pulumi_wasm_rust::Output<String>,
        /// The name or description of the web ACL.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration blocks containing rules to associate with the web ACL and the settings for each rule. Detailed below.
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::waf::WebAclRule>>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WebAclResult {
        /// The ARN of the WAF WebACL.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block with action that you want AWS WAF to take when a request doesn't match the criteria in any of the rules that are associated with the web ACL. Detailed below.
        pub default_action: pulumi_wasm_rust::Output<
            super::super::types::waf::WebAclDefaultAction,
        >,
        /// Configuration block to enable WAF logging. Detailed below.
        pub logging_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::waf::WebAclLoggingConfiguration>,
        >,
        /// The name or description for the Amazon CloudWatch metric of this web ACL.
        pub metric_name: pulumi_wasm_rust::Output<String>,
        /// The name or description of the web ACL.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configuration blocks containing rules to associate with the web ACL and the settings for each rule. Detailed below.
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::waf::WebAclRule>>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WebAclArgs) -> WebAclResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_action_binding = args.default_action.get_inner();
        let logging_configuration_binding = args.logging_configuration.get_inner();
        let metric_name_binding = args.metric_name.get_inner();
        let name_binding = args.name.get_inner();
        let rules_binding = args.rules.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:waf/webAcl:WebAcl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultAction".into(),
                    value: &default_action_binding,
                },
                register_interface::ObjectField {
                    name: "loggingConfiguration".into(),
                    value: &logging_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "metricName".into(),
                    value: &metric_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "defaultAction".into(),
                },
                register_interface::ResultField {
                    name: "loggingConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "metricName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WebAclResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            default_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultAction").unwrap(),
            ),
            logging_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfiguration").unwrap(),
            ),
            metric_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
