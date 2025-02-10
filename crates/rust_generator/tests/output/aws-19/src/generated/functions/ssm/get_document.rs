#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetDocumentArgs,
    ) -> GetDocumentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let document_format_binding = args.document_format.get_output(context);
        let document_version_binding = args.document_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssm/getDocument:getDocument".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "documentFormat".into(),
                    value: document_format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "documentVersion".into(),
                    value: document_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDocumentResult {
            arn: o.get_field("arn"),
            content: o.get_field("content"),
            document_format: o.get_field("documentFormat"),
            document_type: o.get_field("documentType"),
            document_version: o.get_field("documentVersion"),
            id: o.get_field("id"),
            name: o.get_field("name"),
        }
    }
}
