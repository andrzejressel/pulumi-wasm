/// Resource for managing an AWS VPC Lattice Service Network or Service Access log subscription.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_log_subscription::create(
///         "example",
///         AccessLogSubscriptionArgs::builder()
///             .destination_arn("${bucket.arn}")
///             .resource_identifier("${exampleAwsVpclatticeServiceNetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Access Log Subscription using the access log subscription ID. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/accessLogSubscription:AccessLogSubscription example rft-8012925589
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_log_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessLogSubscriptionArgs {
        /// Amazon Resource Name (ARN) of the log destination.
        #[builder(into)]
        pub destination_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID or Amazon Resource Identifier (ARN) of the service network or service. You must use the ARN if the resources specified in the operation are in different accounts.
        #[builder(into)]
        pub resource_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessLogSubscriptionResult {
        /// Amazon Resource Name (ARN) of the access log subscription.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the log destination.
        pub destination_arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the service network or service.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID or Amazon Resource Identifier (ARN) of the service network or service. You must use the ARN if the resources specified in the operation are in different accounts.
        pub resource_identifier: pulumi_gestalt_rust::Output<String>,
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
        args: AccessLogSubscriptionArgs,
    ) -> AccessLogSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_arn_binding = args.destination_arn.get_output(context);
        let resource_identifier_binding = args.resource_identifier.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:vpclattice/accessLogSubscription:AccessLogSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationArn".into(),
                    value: destination_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceIdentifier".into(),
                    value: resource_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessLogSubscriptionResult {
            arn: o.get_field("arn"),
            destination_arn: o.get_field("destinationArn"),
            resource_arn: o.get_field("resourceArn"),
            resource_identifier: o.get_field("resourceIdentifier"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
