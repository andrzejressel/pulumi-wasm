/// Manages an AWS CloudFront Origin Access Control, which is used by CloudFront Distributions with an Amazon S3 bucket as the origin.
///
/// Read more about Origin Access Control in the [CloudFront Developer Guide](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html).
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
///     let example = origin_access_control::create(
///         "example",
///         OriginAccessControlArgs::builder()
///             .description("Example Policy")
///             .name("example")
///             .origin_access_control_origin_type("s3")
///             .signing_behavior("always")
///             .signing_protocol("sigv4")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront Origin Access Control using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/originAccessControl:OriginAccessControl example E327GJI25M56DG
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod origin_access_control {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OriginAccessControlArgs {
        /// The description of the Origin Access Control. Defaults to "Managed by Pulumi" if omitted.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A name that identifies the Origin Access Control.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of origin that this Origin Access Control is for. Valid values are `lambda`, `mediapackagev2`, `mediastore`, and `s3`.
        #[builder(into)]
        pub origin_access_control_origin_type: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// Specifies which requests CloudFront signs. Specify `always` for the most common use case. Allowed values: `always`, `never`, and `no-override`.
        #[builder(into)]
        pub signing_behavior: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Determines how CloudFront signs (authenticates) requests. The only valid value is `sigv4`.
        #[builder(into)]
        pub signing_protocol: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OriginAccessControlResult {
        /// The description of the Origin Access Control. Defaults to "Managed by Pulumi" if omitted.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The current version of this Origin Access Control.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// A name that identifies the Origin Access Control.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The type of origin that this Origin Access Control is for. Valid values are `lambda`, `mediapackagev2`, `mediastore`, and `s3`.
        pub origin_access_control_origin_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies which requests CloudFront signs. Specify `always` for the most common use case. Allowed values: `always`, `never`, and `no-override`.
        pub signing_behavior: pulumi_gestalt_rust::Output<String>,
        /// Determines how CloudFront signs (authenticates) requests. The only valid value is `sigv4`.
        pub signing_protocol: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OriginAccessControlArgs,
    ) -> OriginAccessControlResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let origin_access_control_origin_type_binding = args
            .origin_access_control_origin_type
            .get_output(context);
        let signing_behavior_binding = args.signing_behavior.get_output(context);
        let signing_protocol_binding = args.signing_protocol.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/originAccessControl:OriginAccessControl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originAccessControlOriginType".into(),
                    value: &origin_access_control_origin_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signingBehavior".into(),
                    value: &signing_behavior_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signingProtocol".into(),
                    value: &signing_protocol_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OriginAccessControlResult {
            description: o.get_field("description"),
            etag: o.get_field("etag"),
            name: o.get_field("name"),
            origin_access_control_origin_type: o
                .get_field("originAccessControlOriginType"),
            signing_behavior: o.get_field("signingBehavior"),
            signing_protocol: o.get_field("signingProtocol"),
        }
    }
}
