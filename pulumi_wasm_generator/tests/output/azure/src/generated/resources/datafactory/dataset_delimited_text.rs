/// Manages an Azure Delimited Text Dataset inside an Azure Data Factory.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFactory:
///     type: azure:datafactory:Factory
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleLinkedServiceWeb:
///     type: azure:datafactory:LinkedServiceWeb
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       authenticationType: Anonymous
///       url: https://www.bing.com
///   exampleDatasetDelimitedText:
///     type: azure:datafactory:DatasetDelimitedText
///     name: example
///     properties:
///       name: example
///       dataFactoryId: ${exampleFactory.id}
///       linkedServiceName: ${exampleLinkedServiceWeb.name}
///       httpServerLocation:
///         relativeUrl: http://www.bing.com
///         path: foo/bar/
///         filename: fizz.txt
///       columnDelimiter: ','
///       rowDelimiter: NEW
///       encoding: UTF-8
///       quoteCharacter: x
///       escapeCharacter: f
///       firstRowAsHeader: true
///       nullValue: NULL
/// ```
///
/// ## Import
///
/// Data Factory Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/datasetDelimitedText:DatasetDelimitedText example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/datasets/example
/// ```
///
pub mod dataset_delimited_text {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetDelimitedTextArgs {
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported locations for a Delimited Text Dataset (exactly one of them must be set):
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An `azure_blob_fs_location` block as defined below.
        #[builder(into, default)]
        pub azure_blob_fs_location: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextAzureBlobFsLocation,
            >,
        >,
        /// An `azure_blob_storage_location` block as defined below.
        #[builder(into, default)]
        pub azure_blob_storage_location: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextAzureBlobStorageLocation,
            >,
        >,
        /// The column delimiter. Defaults to `,`.
        #[builder(into, default)]
        pub column_delimiter: pulumi_wasm_rust::Output<Option<String>>,
        /// The compression codec used to read/write text files. Valid values are `None`, `bzip2`, `gzip`, `deflate`, `ZipDeflate`, `TarGzip`, `Tar`, `snappy` and `lz4`. Please note these values are case sensitive.
        #[builder(into, default)]
        pub compression_codec: pulumi_wasm_rust::Output<Option<String>>,
        /// The compression ratio for the Data Factory Dataset. Valid values are `Fastest` or `Optimal`. Please note these values are case sensitive.
        #[builder(into, default)]
        pub compression_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Dataset.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The encoding format for the file.
        #[builder(into, default)]
        pub encoding: pulumi_wasm_rust::Output<Option<String>>,
        /// The escape character. Defaults to `\`.
        #[builder(into, default)]
        pub escape_character: pulumi_wasm_rust::Output<Option<String>>,
        /// When used as input, treat the first row of data as headers. When used as output, write the headers into the output as the first row of data. Defaults to `false`.
        #[builder(into, default)]
        pub first_row_as_header: pulumi_wasm_rust::Output<Option<bool>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_wasm_rust::Output<Option<String>>,
        /// A `http_server_location` block as defined below.
        ///
        /// The following supported arguments are specific to Delimited Text Dataset:
        #[builder(into, default)]
        pub http_server_location: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextHttpServerLocation,
            >,
        >,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        #[builder(into)]
        pub linked_service_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The null value string. Defaults to `""`.
        #[builder(into, default)]
        pub null_value: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of parameters to associate with the Data Factory Dataset.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The quote character. Defaults to `"`.
        #[builder(into, default)]
        pub quote_character: pulumi_wasm_rust::Output<Option<String>>,
        /// The row delimiter. Defaults to any of the following values on read: `\r\n`, `\r`, `\n`, and `\n` or `\r\n` on write by mapping data flow and Copy activity respectively.
        #[builder(into, default)]
        pub row_delimiter: pulumi_wasm_rust::Output<Option<String>>,
        /// A `schema_column` block as defined below.
        #[builder(into, default)]
        pub schema_columns: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::datafactory::DatasetDelimitedTextSchemaColumn>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct DatasetDelimitedTextResult {
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported locations for a Delimited Text Dataset (exactly one of them must be set):
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An `azure_blob_fs_location` block as defined below.
        pub azure_blob_fs_location: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextAzureBlobFsLocation,
            >,
        >,
        /// An `azure_blob_storage_location` block as defined below.
        pub azure_blob_storage_location: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextAzureBlobStorageLocation,
            >,
        >,
        /// The column delimiter. Defaults to `,`.
        pub column_delimiter: pulumi_wasm_rust::Output<Option<String>>,
        /// The compression codec used to read/write text files. Valid values are `None`, `bzip2`, `gzip`, `deflate`, `ZipDeflate`, `TarGzip`, `Tar`, `snappy` and `lz4`. Please note these values are case sensitive.
        pub compression_codec: pulumi_wasm_rust::Output<Option<String>>,
        /// The compression ratio for the Data Factory Dataset. Valid values are `Fastest` or `Optimal`. Please note these values are case sensitive.
        pub compression_level: pulumi_wasm_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Dataset.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The encoding format for the file.
        pub encoding: pulumi_wasm_rust::Output<Option<String>>,
        /// The escape character. Defaults to `\`.
        pub escape_character: pulumi_wasm_rust::Output<Option<String>>,
        /// When used as input, treat the first row of data as headers. When used as output, write the headers into the output as the first row of data. Defaults to `false`.
        pub first_row_as_header: pulumi_wasm_rust::Output<Option<bool>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        pub folder: pulumi_wasm_rust::Output<Option<String>>,
        /// A `http_server_location` block as defined below.
        ///
        /// The following supported arguments are specific to Delimited Text Dataset:
        pub http_server_location: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextHttpServerLocation,
            >,
        >,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        pub linked_service_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The null value string. Defaults to `""`.
        pub null_value: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of parameters to associate with the Data Factory Dataset.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The quote character. Defaults to `"`.
        pub quote_character: pulumi_wasm_rust::Output<Option<String>>,
        /// The row delimiter. Defaults to any of the following values on read: `\r\n`, `\r`, `\n`, and `\n` or `\r\n` on write by mapping data flow and Copy activity respectively.
        pub row_delimiter: pulumi_wasm_rust::Output<Option<String>>,
        /// A `schema_column` block as defined below.
        pub schema_columns: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::datafactory::DatasetDelimitedTextSchemaColumn>,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DatasetDelimitedTextArgs,
    ) -> DatasetDelimitedTextResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding = args.additional_properties.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let azure_blob_fs_location_binding = args.azure_blob_fs_location.get_inner();
        let azure_blob_storage_location_binding = args
            .azure_blob_storage_location
            .get_inner();
        let column_delimiter_binding = args.column_delimiter.get_inner();
        let compression_codec_binding = args.compression_codec.get_inner();
        let compression_level_binding = args.compression_level.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let encoding_binding = args.encoding.get_inner();
        let escape_character_binding = args.escape_character.get_inner();
        let first_row_as_header_binding = args.first_row_as_header.get_inner();
        let folder_binding = args.folder.get_inner();
        let http_server_location_binding = args.http_server_location.get_inner();
        let linked_service_name_binding = args.linked_service_name.get_inner();
        let name_binding = args.name.get_inner();
        let null_value_binding = args.null_value.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let quote_character_binding = args.quote_character.get_inner();
        let row_delimiter_binding = args.row_delimiter.get_inner();
        let schema_columns_binding = args.schema_columns.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/datasetDelimitedText:DatasetDelimitedText".into(),
            name: name.to_string(),
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
                    name: "azureBlobFsLocation".into(),
                    value: &azure_blob_fs_location_binding,
                },
                register_interface::ObjectField {
                    name: "azureBlobStorageLocation".into(),
                    value: &azure_blob_storage_location_binding,
                },
                register_interface::ObjectField {
                    name: "columnDelimiter".into(),
                    value: &column_delimiter_binding,
                },
                register_interface::ObjectField {
                    name: "compressionCodec".into(),
                    value: &compression_codec_binding,
                },
                register_interface::ObjectField {
                    name: "compressionLevel".into(),
                    value: &compression_level_binding,
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
                    name: "encoding".into(),
                    value: &encoding_binding,
                },
                register_interface::ObjectField {
                    name: "escapeCharacter".into(),
                    value: &escape_character_binding,
                },
                register_interface::ObjectField {
                    name: "firstRowAsHeader".into(),
                    value: &first_row_as_header_binding,
                },
                register_interface::ObjectField {
                    name: "folder".into(),
                    value: &folder_binding,
                },
                register_interface::ObjectField {
                    name: "httpServerLocation".into(),
                    value: &http_server_location_binding,
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
                    name: "nullValue".into(),
                    value: &null_value_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "quoteCharacter".into(),
                    value: &quote_character_binding,
                },
                register_interface::ObjectField {
                    name: "rowDelimiter".into(),
                    value: &row_delimiter_binding,
                },
                register_interface::ObjectField {
                    name: "schemaColumns".into(),
                    value: &schema_columns_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalProperties".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "azureBlobFsLocation".into(),
                },
                register_interface::ResultField {
                    name: "azureBlobStorageLocation".into(),
                },
                register_interface::ResultField {
                    name: "columnDelimiter".into(),
                },
                register_interface::ResultField {
                    name: "compressionCodec".into(),
                },
                register_interface::ResultField {
                    name: "compressionLevel".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "encoding".into(),
                },
                register_interface::ResultField {
                    name: "escapeCharacter".into(),
                },
                register_interface::ResultField {
                    name: "firstRowAsHeader".into(),
                },
                register_interface::ResultField {
                    name: "folder".into(),
                },
                register_interface::ResultField {
                    name: "httpServerLocation".into(),
                },
                register_interface::ResultField {
                    name: "linkedServiceName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nullValue".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "quoteCharacter".into(),
                },
                register_interface::ResultField {
                    name: "rowDelimiter".into(),
                },
                register_interface::ResultField {
                    name: "schemaColumns".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatasetDelimitedTextResult {
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalProperties").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            azure_blob_fs_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureBlobFsLocation").unwrap(),
            ),
            azure_blob_storage_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureBlobStorageLocation").unwrap(),
            ),
            column_delimiter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("columnDelimiter").unwrap(),
            ),
            compression_codec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compressionCodec").unwrap(),
            ),
            compression_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compressionLevel").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            encoding: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encoding").unwrap(),
            ),
            escape_character: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("escapeCharacter").unwrap(),
            ),
            first_row_as_header: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firstRowAsHeader").unwrap(),
            ),
            folder: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folder").unwrap(),
            ),
            http_server_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpServerLocation").unwrap(),
            ),
            linked_service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkedServiceName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            null_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nullValue").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            quote_character: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quoteCharacter").unwrap(),
            ),
            row_delimiter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rowDelimiter").unwrap(),
            ),
            schema_columns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemaColumns").unwrap(),
            ),
        }
    }
}