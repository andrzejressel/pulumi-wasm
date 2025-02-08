#[allow(clippy::doc_lazy_continuation)]
pub mod get_document {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDocumentArgs {
        /// The format of the document. Valid values: `JSON`, `TEXT`, `YAML`.
        #[builder(into, default)]
        pub document_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The document version.
        #[builder(into, default)]
        pub document_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the document.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDocumentResult {
        /// ARN of the document. If the document is an AWS managed document, this value will be set to the name of the document instead.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The content for the SSM document in JSON or YAML format.
        pub content: pulumi_gestalt_rust::Output<String>,
        pub document_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of the document.
        pub document_type: pulumi_gestalt_rust::Output<String>,
        pub document_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDocumentArgs,
    ) -> GetDocumentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let document_format_binding = args
            .document_format
            .get_output(context)
            .get_inner();
        let document_version_binding = args
            .document_version
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssm/getDocument:getDocument".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "documentFormat".into(),
                    value: &document_format_binding,
                },
                register_interface::ObjectField {
                    name: "documentVersion".into(),
                    value: &document_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDocumentResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            document_format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("documentFormat"),
            ),
            document_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("documentType"),
            ),
            document_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("documentVersion"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
