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
pub mod thing_principal_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThingPrincipalAttachmentArgs {
        /// The AWS IoT Certificate ARN or Amazon Cognito Identity ID.
        #[builder(into)]
        pub principal: pulumi_wasm_rust::Output<String>,
        /// The name of the thing.
        #[builder(into)]
        pub thing: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ThingPrincipalAttachmentResult {
        /// The AWS IoT Certificate ARN or Amazon Cognito Identity ID.
        pub principal: pulumi_wasm_rust::Output<String>,
        /// The name of the thing.
        pub thing: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ThingPrincipalAttachmentArgs,
    ) -> ThingPrincipalAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let principal_binding = args.principal.get_inner();
        let thing_binding = args.thing.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/thingPrincipalAttachment:ThingPrincipalAttachment".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "principal".into(),
                },
                register_interface::ResultField {
                    name: "thing".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ThingPrincipalAttachmentResult {
            principal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principal").unwrap(),
            ),
            thing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thing").unwrap(),
            ),
        }
    }
}
