/// Manages an Azure HTTP Dataset inside an Azure Data Factory.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleDatasetHttp = dataset_http::create(
///         "exampleDatasetHttp",
///         DatasetHttpArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .linked_service_name("${exampleLinkedServiceWeb.name}")
///             .name("example")
///             .relative_url("http://www.bing.com")
///             .request_body("foo=bar")
///             .request_method("POST")
///             .build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLinkedServiceWeb = linked_service_web::create(
///         "exampleLinkedServiceWeb",
///         LinkedServiceWebArgs::builder()
///             .authentication_type("Anonymous")
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .url("https://www.bing.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/datasetHttp:DatasetHttp example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/datasets/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dataset_http {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetHttpArgs {
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported arguments are specific to HTTP Dataset:
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Dataset.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        #[builder(into)]
        pub linked_service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Dataset.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The relative URL based on the URL in the HTTP Linked Service.
        #[builder(into, default)]
        pub relative_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The body for the HTTP request.
        #[builder(into, default)]
        pub request_body: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The HTTP method for the HTTP request. (e.g. GET, POST)
        #[builder(into, default)]
        pub request_method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `schema_column` block as defined below.
        #[builder(into, default)]
        pub schema_columns: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datafactory::DatasetHttpSchemaColumn>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatasetHttpResult {
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported arguments are specific to HTTP Dataset:
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Dataset.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        pub folder: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        pub linked_service_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Dataset.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The relative URL based on the URL in the HTTP Linked Service.
        pub relative_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The body for the HTTP request.
        pub request_body: pulumi_gestalt_rust::Output<Option<String>>,
        /// The HTTP method for the HTTP request. (e.g. GET, POST)
        pub request_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `schema_column` block as defined below.
        pub schema_columns: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datafactory::DatasetHttpSchemaColumn>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DatasetHttpArgs,
    ) -> DatasetHttpResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding_1 = args
            .additional_properties
            .get_output(context);
        let additional_properties_binding = additional_properties_binding_1.get_inner();
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let data_factory_id_binding_1 = args.data_factory_id.get_output(context);
        let data_factory_id_binding = data_factory_id_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let folder_binding_1 = args.folder.get_output(context);
        let folder_binding = folder_binding_1.get_inner();
        let linked_service_name_binding_1 = args.linked_service_name.get_output(context);
        let linked_service_name_binding = linked_service_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let relative_url_binding_1 = args.relative_url.get_output(context);
        let relative_url_binding = relative_url_binding_1.get_inner();
        let request_body_binding_1 = args.request_body.get_output(context);
        let request_body_binding = request_body_binding_1.get_inner();
        let request_method_binding_1 = args.request_method.get_output(context);
        let request_method_binding = request_method_binding_1.get_inner();
        let schema_columns_binding_1 = args.schema_columns.get_output(context);
        let schema_columns_binding = schema_columns_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/datasetHttp:DatasetHttp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalProperties".into(),
                    value: &additional_properties_binding,
                },
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "folder".into(),
                    value: &folder_binding,
                },
                register_interface::ObjectField {
                    name: "linkedServiceName".into(),
                    value: &linked_service_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "relativeUrl".into(),
                    value: &relative_url_binding,
                },
                register_interface::ObjectField {
                    name: "requestBody".into(),
                    value: &request_body_binding,
                },
                register_interface::ObjectField {
                    name: "requestMethod".into(),
                    value: &request_method_binding,
                },
                register_interface::ObjectField {
                    name: "schemaColumns".into(),
                    value: &schema_columns_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DatasetHttpResult {
            additional_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalProperties"),
            ),
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            data_factory_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataFactoryId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            folder: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folder"),
            ),
            linked_service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkedServiceName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            relative_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("relativeUrl"),
            ),
            request_body: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requestBody"),
            ),
            request_method: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requestMethod"),
            ),
            schema_columns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schemaColumns"),
            ),
        }
    }
}
