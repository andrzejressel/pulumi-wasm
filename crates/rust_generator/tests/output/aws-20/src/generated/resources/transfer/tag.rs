/// Manages an individual Transfer Family resource tag. This resource should only be used in cases where Transfer Family resources are created outside the provider (e.g., Servers without AWS Management Console) or the tag key has the `aws:` prefix.
///
/// > **NOTE:** This tagging resource should not be combined with the resource for managing the parent resource. For example, using `aws.transfer.Server` and `aws.transfer.Tag` to manage tags of the same server will cause a perpetual difference where the `aws.transfer.Server` resource will try to remove the tag being added by the `aws.transfer.Tag` resource.
///
/// > **NOTE:** This tagging resource does not use the provider `ignore_tags` configuration.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = server::create(
///         "example",
///         ServerArgs::builder().identity_provider_type("SERVICE_MANAGED").build_struct(),
///     );
///     let hostname = tag::create(
///         "hostname",
///         TagArgs::builder()
///             .key("aws:transfer:customHostname")
///             .resource_arn("${example.arn}")
///             .value("example.com")
///             .build_struct(),
///     );
///     let zoneId = tag::create(
///         "zoneId",
///         TagArgs::builder()
///             .key("aws:transfer:route53HostedZoneId")
///             .resource_arn("${example.arn}")
///             .value("/hostedzone/MyHostedZoneId")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_transfer_tag` using the Transfer Family resource identifier and key, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:transfer/tag:Tag example arn:aws:transfer:us-east-1:123456789012:server/s-1234567890abcdef0,Name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagArgs {
        /// Tag name.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amazon Resource Name (ARN) of the Transfer Family resource to tag.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tag value.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagResult {
        /// Tag name.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Transfer Family resource to tag.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
        /// Tag value.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TagArgs,
    ) -> TagResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_binding_1 = args.key.get_output(context);
        let key_binding = key_binding_1.get_inner();
        let resource_arn_binding_1 = args.resource_arn.get_output(context);
        let resource_arn_binding = resource_arn_binding_1.get_inner();
        let value_binding_1 = args.value.get_output(context);
        let value_binding = value_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transfer/tag:Tag".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TagResult {
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            resource_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
