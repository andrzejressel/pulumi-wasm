/// Resource for managing an AWS CloudWatch Observability Access Manager Link.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:oam:Link
///     properties:
///       labelTemplate: $AccountName
///       resourceTypes:
///         - AWS::CloudWatch::Metric
///       sinkIdentifier: ${test.id}
///       tags:
///         Env: prod
/// ```
///
/// ### Log Group Filtering
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = link::create(
///         "example",
///         LinkArgs::builder()
///             .label_template("$AccountName")
///             .link_configuration(
///                 LinkLinkConfiguration::builder()
///                     .logGroupConfiguration(
///                         LinkLinkConfigurationLogGroupConfiguration::builder()
///                             .filter(
///                                 "LogGroupName LIKE 'aws/lambda/%' OR LogGroupName LIKE 'AWSLogs%'",
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .resource_types(vec!["AWS::Logs::LogGroup",])
///             .sink_identifier("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Metric Filtering
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = link::create(
///         "example",
///         LinkArgs::builder()
///             .label_template("$AccountName")
///             .link_configuration(
///                 LinkLinkConfiguration::builder()
///                     .metricConfiguration(
///                         LinkLinkConfigurationMetricConfiguration::builder()
///                             .filter("Namespace IN ('AWS/EC2', 'AWS/ELB', 'AWS/S3')")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .resource_types(vec!["AWS::CloudWatch::Metric",])
///             .sink_identifier("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Observability Access Manager Link using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:oam/link:Link example arn:aws:oam:us-west-2:123456789012:link/link-id
/// ```
pub mod link {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkArgs {
        /// Human-readable name to use to identify this source account when you are viewing data from it in the monitoring account.
        #[builder(into)]
        pub label_template: pulumi_wasm_rust::Output<String>,
        /// Configuration for creating filters that specify that only some metric namespaces or log groups are to be shared from the source account to the monitoring account. See `link_configuration` Block for details.
        #[builder(into, default)]
        pub link_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::oam::LinkLinkConfiguration>,
        >,
        /// Types of data that the source account shares with the monitoring account.
        #[builder(into)]
        pub resource_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// Identifier of the sink to use to create this link.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub sink_identifier: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LinkResult {
        /// ARN of the link.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Label that is assigned to this link.
        pub label: pulumi_wasm_rust::Output<String>,
        /// Human-readable name to use to identify this source account when you are viewing data from it in the monitoring account.
        pub label_template: pulumi_wasm_rust::Output<String>,
        /// Configuration for creating filters that specify that only some metric namespaces or log groups are to be shared from the source account to the monitoring account. See `link_configuration` Block for details.
        pub link_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::oam::LinkLinkConfiguration>,
        >,
        /// ID string that AWS generated as part of the link ARN.
        pub link_id: pulumi_wasm_rust::Output<String>,
        /// Types of data that the source account shares with the monitoring account.
        pub resource_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// ARN of the sink that is used for this link.
        pub sink_arn: pulumi_wasm_rust::Output<String>,
        /// Identifier of the sink to use to create this link.
        ///
        /// The following arguments are optional:
        pub sink_identifier: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LinkArgs) -> LinkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let label_template_binding = args.label_template.get_inner();
        let link_configuration_binding = args.link_configuration.get_inner();
        let resource_types_binding = args.resource_types.get_inner();
        let sink_identifier_binding = args.sink_identifier.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:oam/link:Link".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "labelTemplate".into(),
                    value: &label_template_binding,
                },
                register_interface::ObjectField {
                    name: "linkConfiguration".into(),
                    value: &link_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTypes".into(),
                    value: &resource_types_binding,
                },
                register_interface::ObjectField {
                    name: "sinkIdentifier".into(),
                    value: &sink_identifier_binding,
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
                    name: "label".into(),
                },
                register_interface::ResultField {
                    name: "labelTemplate".into(),
                },
                register_interface::ResultField {
                    name: "linkConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "linkId".into(),
                },
                register_interface::ResultField {
                    name: "resourceTypes".into(),
                },
                register_interface::ResultField {
                    name: "sinkArn".into(),
                },
                register_interface::ResultField {
                    name: "sinkIdentifier".into(),
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
        LinkResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("label").unwrap(),
            ),
            label_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelTemplate").unwrap(),
            ),
            link_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkConfiguration").unwrap(),
            ),
            link_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkId").unwrap(),
            ),
            resource_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTypes").unwrap(),
            ),
            sink_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sinkArn").unwrap(),
            ),
            sink_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sinkIdentifier").unwrap(),
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