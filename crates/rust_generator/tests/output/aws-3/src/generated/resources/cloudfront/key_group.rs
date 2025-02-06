/// ## Example Usage
///
/// The following example below creates a CloudFront key group.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudfront:PublicKey
///     properties:
///       comment: example public key
///       encodedKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: public_key.pem
///           return: result
///       name: example-key
///   exampleKeyGroup:
///     type: aws:cloudfront:KeyGroup
///     name: example
///     properties:
///       comment: example key group
///       items:
///         - ${example.id}
///       name: example-key-group
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront Key Group using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/keyGroup:KeyGroup example 4b4f2r1c-315d-5c2e-f093-216t50jed10f
/// ```
pub mod key_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyGroupArgs {
        /// A comment to describe the key group..
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of the identifiers of the public keys in the key group.
        #[builder(into)]
        pub items: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// A name to identify the key group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeyGroupResult {
        /// A comment to describe the key group..
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier for this version of the key group.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// A list of the identifiers of the public keys in the key group.
        pub items: pulumi_wasm_rust::Output<Vec<String>>,
        /// A name to identify the key group.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: KeyGroupArgs,
    ) -> KeyGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let comment_binding = args.comment.get_output(context).get_inner();
        let items_binding = args.items.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/keyGroup:KeyGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "items".into(),
                    value: &items_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        KeyGroupResult {
            comment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            items: pulumi_wasm_rust::__private::into_domain(o.extract_field("items")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
