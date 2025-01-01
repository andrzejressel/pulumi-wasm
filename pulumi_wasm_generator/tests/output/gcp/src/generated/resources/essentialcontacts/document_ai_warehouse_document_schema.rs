/// A document schema is used to define document structure.
///
///
/// To get more information about DocumentSchema, see:
///
/// * [API documentation](https://cloud.google.com/document-warehouse/docs/reference/rest/v1/projects.locations.documentSchemas)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/document-warehouse/docs/manage-document-schemas)
///
/// ## Example Usage
///
/// ### Document Ai Warehouse Document Schema Text
///
///
/// ```yaml
/// resources:
///   exampleText:
///     type: gcp:essentialcontacts:DocumentAiWarehouseDocumentSchema
///     name: example_text
///     properties:
///       projectNumber: ${project.number}
///       displayName: test-property-text
///       location: us
///       documentIsFolder: false
///       propertyDefinitions:
///         - name: prop3
///           displayName: propdisp3
///           isRepeatable: false
///           isFilterable: true
///           isSearchable: true
///           isMetadata: false
///           isRequired: false
///           retrievalImportance: HIGHEST
///           schemaSources:
///             - name: dummy_source
///               processorType: dummy_processor
///           textTypeOptions: {}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// DocumentSchema can be imported using any of these accepted formats:
///
/// * `projects/{{project_number}}/locations/{{location}}/documentSchemas/{{name}}`
///
/// * `{{project_number}}/{{location}}/{{name}}`
///
/// When using the `pulumi import` command, DocumentSchema can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:essentialcontacts/documentAiWarehouseDocumentSchema:DocumentAiWarehouseDocumentSchema default projects/{{project_number}}/locations/{{location}}/documentSchemas/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:essentialcontacts/documentAiWarehouseDocumentSchema:DocumentAiWarehouseDocumentSchema default {{project_number}}/{{location}}/{{name}}
/// ```
///
pub mod document_ai_warehouse_document_schema {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DocumentAiWarehouseDocumentSchemaArgs {
        /// Name of the schema given by the user.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Tells whether the document is a folder or a typical document.
        #[builder(into, default)]
        pub document_is_folder: pulumi_wasm_rust::Output<Option<bool>>,
        /// The location of the resource.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the project.
        #[builder(into)]
        pub project_number: pulumi_wasm_rust::Output<String>,
        /// Defines the metadata for a schema property.
        /// Structure is documented below.
        #[builder(into)]
        pub property_definitions: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinition,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct DocumentAiWarehouseDocumentSchemaResult {
        /// Name of the schema given by the user.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Tells whether the document is a folder or a typical document.
        pub document_is_folder: pulumi_wasm_rust::Output<Option<bool>>,
        /// The location of the resource.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the document schema.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the project.
        pub project_number: pulumi_wasm_rust::Output<String>,
        /// Defines the metadata for a schema property.
        /// Structure is documented below.
        pub property_definitions: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinition,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DocumentAiWarehouseDocumentSchemaArgs,
    ) -> DocumentAiWarehouseDocumentSchemaResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let document_is_folder_binding = args.document_is_folder.get_inner();
        let location_binding = args.location.get_inner();
        let project_number_binding = args.project_number.get_inner();
        let property_definitions_binding = args.property_definitions.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:essentialcontacts/documentAiWarehouseDocumentSchema:DocumentAiWarehouseDocumentSchema"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "documentIsFolder".into(),
                    value: &document_is_folder_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "projectNumber".into(),
                    value: &project_number_binding,
                },
                register_interface::ObjectField {
                    name: "propertyDefinitions".into(),
                    value: &property_definitions_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "documentIsFolder".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "projectNumber".into(),
                },
                register_interface::ResultField {
                    name: "propertyDefinitions".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DocumentAiWarehouseDocumentSchemaResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            document_is_folder: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentIsFolder").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectNumber").unwrap(),
            ),
            property_definitions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("propertyDefinitions").unwrap(),
            ),
        }
    }
}
