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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkArgs {
        /// Human-readable name to use to identify this source account when you are viewing data from it in the monitoring account.
        #[builder(into)]
        pub label_template: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration for creating filters that specify that only some metric namespaces or log groups are to be shared from the source account to the monitoring account. See `link_configuration` Block for details.
        #[builder(into, default)]
        pub link_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::oam::LinkLinkConfiguration>,
        >,
        /// Types of data that the source account shares with the monitoring account.
        #[builder(into)]
        pub resource_types: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Identifier of the sink to use to create this link.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub sink_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LinkResult {
        /// ARN of the link.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Label that is assigned to this link.
        pub label: pulumi_gestalt_rust::Output<String>,
        /// Human-readable name to use to identify this source account when you are viewing data from it in the monitoring account.
        pub label_template: pulumi_gestalt_rust::Output<String>,
        /// Configuration for creating filters that specify that only some metric namespaces or log groups are to be shared from the source account to the monitoring account. See `link_configuration` Block for details.
        pub link_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::oam::LinkLinkConfiguration>,
        >,
        /// ID string that AWS generated as part of the link ARN.
        pub link_id: pulumi_gestalt_rust::Output<String>,
        /// Types of data that the source account shares with the monitoring account.
        pub resource_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ARN of the sink that is used for this link.
        pub sink_arn: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the sink to use to create this link.
        ///
        /// The following arguments are optional:
        pub sink_identifier: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
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
        args: LinkArgs,
    ) -> LinkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let label_template_binding = args.label_template.get_output(context);
        let link_configuration_binding = args.link_configuration.get_output(context);
        let resource_types_binding = args.resource_types.get_output(context);
        let sink_identifier_binding = args.sink_identifier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:oam/link:Link".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labelTemplate".into(),
                    value: &label_template_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkConfiguration".into(),
                    value: &link_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceTypes".into(),
                    value: &resource_types_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sinkIdentifier".into(),
                    value: &sink_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkResult {
            arn: o.get_field("arn"),
            label: o.get_field("label"),
            label_template: o.get_field("labelTemplate"),
            link_configuration: o.get_field("linkConfiguration"),
            link_id: o.get_field("linkId"),
            resource_types: o.get_field("resourceTypes"),
            sink_arn: o.get_field("sinkArn"),
            sink_identifier: o.get_field("sinkIdentifier"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
