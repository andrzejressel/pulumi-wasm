/// Provides a CloudWatch Logs destination resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testDestination = log_destination::create(
///         "testDestination",
///         LogDestinationArgs::builder()
///             .name("test_destination")
///             .role_arn("${iamForCloudwatch.arn}")
///             .target_arn("${kinesisForCloudwatch.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Logs destinations using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logDestination:LogDestination test_destination test_destination
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_destination {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogDestinationArgs {
        /// A name for the log destination.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of an IAM role that grants Amazon CloudWatch Logs permissions to put data into the target.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the target Amazon Kinesis stream resource for the destination.
        #[builder(into)]
        pub target_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LogDestinationResult {
        /// The Amazon Resource Name (ARN) specifying the log destination.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A name for the log destination.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of an IAM role that grants Amazon CloudWatch Logs permissions to put data into the target.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ARN of the target Amazon Kinesis stream resource for the destination.
        pub target_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LogDestinationArgs,
    ) -> LogDestinationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_arn_binding = args.target_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/logDestination:LogDestination".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetArn".into(),
                    value: &target_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LogDestinationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetArn"),
            ),
        }
    }
}
