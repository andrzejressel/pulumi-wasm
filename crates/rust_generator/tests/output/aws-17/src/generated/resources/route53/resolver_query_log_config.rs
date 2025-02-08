/// Provides a Route 53 Resolver query logging configuration resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53:ResolverQueryLogConfig
///     properties:
///       name: example
///       destinationArn: ${exampleAwsS3Bucket.arn}
///       tags:
///         Environment: Prod
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import  Route 53 Resolver query logging configurations using the Route 53 Resolver query logging configuration ID. For example:
///
/// ```sh
/// $ pulumi import aws:route53/resolverQueryLogConfig:ResolverQueryLogConfig example rqlc-92edc3b1838248bf
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resolver_query_log_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResolverQueryLogConfigArgs {
        /// The ARN of the resource that you want Route 53 Resolver to send query logs.
        /// You can send query logs to an S3 bucket, a CloudWatch Logs log group, or a Kinesis Data Firehose delivery stream.
        #[builder(into)]
        pub destination_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Route 53 Resolver query logging configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResolverQueryLogConfigResult {
        /// The ARN (Amazon Resource Name) of the Route 53 Resolver query logging configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the resource that you want Route 53 Resolver to send query logs.
        /// You can send query logs to an S3 bucket, a CloudWatch Logs log group, or a Kinesis Data Firehose delivery stream.
        pub destination_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the Route 53 Resolver query logging configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID of the account that created the query logging configuration.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// An indication of whether the query logging configuration is shared with other AWS accounts, or was shared with the current account by another AWS account.
        /// Sharing is configured through AWS Resource Access Manager (AWS RAM).
        /// Values are `NOT_SHARED`, `SHARED_BY_ME` or `SHARED_WITH_ME`
        pub share_status: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResolverQueryLogConfigArgs,
    ) -> ResolverQueryLogConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let destination_arn_binding = args
            .destination_arn
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/resolverQueryLogConfig:ResolverQueryLogConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationArn".into(),
                    value: &destination_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResolverQueryLogConfigResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            destination_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationArn"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            share_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shareStatus"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
