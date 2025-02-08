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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AccessLogSubscriptionArgs,
    ) -> AccessLogSubscriptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let destination_arn_binding = args
            .destination_arn
            .get_output(context)
            .get_inner();
        let resource_identifier_binding = args
            .resource_identifier
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpclattice/accessLogSubscription:AccessLogSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationArn".into(),
                    value: &destination_arn_binding,
                },
                register_interface::ObjectField {
                    name: "resourceIdentifier".into(),
                    value: &resource_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccessLogSubscriptionResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            destination_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationArn"),
            ),
            resource_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
            resource_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceIdentifier"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
