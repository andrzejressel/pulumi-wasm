/// Attaches Principal to AWS IoT Thing.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iot:Thing
///     properties:
///       name: example
///   cert:
///     type: aws:iot:Certificate
///     properties:
///       csr:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: csr.pem
///           return: result
///       active: true
///   att:
///     type: aws:iot:ThingPrincipalAttachment
///     properties:
///       principal: ${cert.arn}
///       thing: ${example.name}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod thing_principal_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThingPrincipalAttachmentArgs {
        /// The AWS IoT Certificate ARN or Amazon Cognito Identity ID.
        #[builder(into)]
        pub principal: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the thing.
        #[builder(into)]
        pub thing: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ThingPrincipalAttachmentResult {
        /// The AWS IoT Certificate ARN or Amazon Cognito Identity ID.
        pub principal: pulumi_gestalt_rust::Output<String>,
        /// The name of the thing.
        pub thing: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ThingPrincipalAttachmentArgs,
    ) -> ThingPrincipalAttachmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let principal_binding = args.principal.get_output(context).get_inner();
        let thing_binding = args.thing.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/thingPrincipalAttachment:ThingPrincipalAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "principal".into(),
                    value: &principal_binding,
                },
                register_interface::ObjectField {
                    name: "thing".into(),
                    value: &thing_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ThingPrincipalAttachmentResult {
            principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principal"),
            ),
            thing: pulumi_gestalt_rust::__private::into_domain(o.extract_field("thing")),
        }
    }
}
