pub mod get_document {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDocumentArgs {
        /// The format of the document. Valid values: `JSON`, `TEXT`, `YAML`.
        #[builder(into, default)]
        pub document_format: pulumi_wasm_rust::Output<Option<String>>,
        /// The document version.
        #[builder(into, default)]
        pub document_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the document.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDocumentResult {
        /// ARN of the document. If the document is an AWS managed document, this value will be set to the name of the document instead.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The content for the SSM document in JSON or YAML format.
        pub content: pulumi_wasm_rust::Output<String>,
        pub document_format: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of the document.
        pub document_type: pulumi_wasm_rust::Output<String>,
        pub document_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDocumentArgs) -> GetDocumentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let document_format_binding = args.document_format.get_inner();
        let document_version_binding = args.document_version.get_inner();
        let name_binding = args.name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "content".into(),
                },
                register_interface::ResultField {
                    name: "documentFormat".into(),
                },
                register_interface::ResultField {
                    name: "documentType".into(),
                },
                register_interface::ResultField {
                    name: "documentVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDocumentResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
            ),
            document_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentFormat").unwrap(),
            ),
            document_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentType").unwrap(),
            ),
            document_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
