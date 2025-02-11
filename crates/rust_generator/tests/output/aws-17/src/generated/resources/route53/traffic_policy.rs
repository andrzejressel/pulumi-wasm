/// Manages a Route53 Traffic Policy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = traffic_policy::create(
///         "example",
///         TrafficPolicyArgs::builder()
///             .comment("example comment")
///             .document(
///                 "{\n  \"AWSPolicyFormatVersion\": \"2015-10-01\",\n  \"RecordType\": \"A\",\n  \"Endpoints\": {\n    \"endpoint-start-NkPh\": {\n      \"Type\": \"value\",\n      \"Value\": \"10.0.0.2\"\n    }\n  },\n  \"StartEndpoint\": \"endpoint-start-NkPh\"\n}",
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Traffic Policy using the `id` and `version`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/trafficPolicy:TrafficPolicy example 01a52019-d16f-422a-ae72-c306d2b6df7e/1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod traffic_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrafficPolicyArgs {
        /// Comment for the traffic policy.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Policy document. This is a JSON formatted string. For more information about building Route53 traffic policy documents, see the [AWS Route53 Traffic Policy document format](https://docs.aws.amazon.com/Route53/latest/APIReference/api-policies-traffic-policy-document-format.html)
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub document: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the traffic policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TrafficPolicyResult {
        /// Comment for the traffic policy.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Policy document. This is a JSON formatted string. For more information about building Route53 traffic policy documents, see the [AWS Route53 Traffic Policy document format](https://docs.aws.amazon.com/Route53/latest/APIReference/api-policies-traffic-policy-document-format.html)
        ///
        /// The following arguments are optional:
        pub document: pulumi_gestalt_rust::Output<String>,
        /// Name of the traffic policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// DNS type of the resource record sets that Amazon Route 53 creates when you use a traffic policy to create a traffic policy instance.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Version number of the traffic policy. This value is automatically incremented by AWS after each update of this resource.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrafficPolicyArgs,
    ) -> TrafficPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let comment_binding = args.comment.get_output(context);
        let document_binding = args.document.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/trafficPolicy:TrafficPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "document".into(),
                    value: &document_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrafficPolicyResult {
            comment: o.get_field("comment"),
            document: o.get_field("document"),
            name: o.get_field("name"),
            type_: o.get_field("type"),
            version: o.get_field("version"),
        }
    }
}
