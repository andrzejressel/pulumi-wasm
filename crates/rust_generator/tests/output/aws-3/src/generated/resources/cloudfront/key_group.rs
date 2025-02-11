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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyGroupArgs {
        /// A comment to describe the key group..
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of the identifiers of the public keys in the key group.
        #[builder(into)]
        pub items: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A name to identify the key group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeyGroupResult {
        /// A comment to describe the key group..
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier for this version of the key group.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// A list of the identifiers of the public keys in the key group.
        pub items: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A name to identify the key group.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyGroupArgs,
    ) -> KeyGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let comment_binding = args.comment.get_output(context);
        let items_binding = args.items.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/keyGroup:KeyGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "items".into(),
                    value: &items_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        KeyGroupResult {
            comment: o.get_field("comment"),
            etag: o.get_field("etag"),
            items: o.get_field("items"),
            name: o.get_field("name"),
        }
    }
}
