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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dataset_delimited_text {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetDelimitedTextArgs {
        /// A map of additional properties to associate with the Data Factory Dataset.
        ///
        /// The following supported locations for a Delimited Text Dataset (exactly one of them must be set):
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// An `azure_blob_fs_location` block as defined below.
        #[builder(into, default)]
        pub azure_blob_fs_location: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextAzureBlobFsLocation,
            >,
        >,
        /// An `azure_blob_storage_location` block as defined below.
        #[builder(into, default)]
        pub azure_blob_storage_location: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextAzureBlobStorageLocation,
            >,
        >,
        /// The column delimiter. Defaults to `,`.
        #[builder(into, default)]
        pub column_delimiter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The compression codec used to read/write text files. Valid values are `None`, `bzip2`, `gzip`, `deflate`, `ZipDeflate`, `TarGzip`, `Tar`, `snappy` and `lz4`. Please note these values are case sensitive.
        #[builder(into, default)]
        pub compression_codec: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The compression ratio for the Data Factory Dataset. Valid values are `Fastest` or `Optimal`. Please note these values are case sensitive.
        #[builder(into, default)]
        pub compression_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Dataset.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The encoding format for the file.
        #[builder(into, default)]
        pub encoding: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The escape character. Defaults to `\`.
        #[builder(into, default)]
        pub escape_character: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When used as input, treat the first row of data as headers. When used as output, write the headers into the output as the first row of data. Defaults to `false`.
        #[builder(into, default)]
        pub first_row_as_header: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `http_server_location` block as defined below.
        ///
        /// The following supported arguments are specific to Delimited Text Dataset:
        #[builder(into, default)]
        pub http_server_location: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextHttpServerLocation,
            >,
        >,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        #[builder(into)]
        pub linked_service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The null value string. Defaults to `""`.
        #[builder(into, default)]
        pub null_value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Dataset.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The quote character. Defaults to `"`.
        #[builder(into, default)]
        pub quote_character: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The row delimiter. Defaults to any of the following values on read: `\r\n`, `\r`, `\n`, and `\n` or `\r\n` on write by mapping data flow and Copy activity respectively.
        #[builder(into, default)]
        pub row_delimiter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `schema_column` block as defined below.
        #[builder(into, default)]
        pub schema_columns: pulumi_gestalt_rust::InputOrOutput<
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
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Dataset.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An `azure_blob_fs_location` block as defined below.
        pub azure_blob_fs_location: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextAzureBlobFsLocation,
            >,
        >,
        /// An `azure_blob_storage_location` block as defined below.
        pub azure_blob_storage_location: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextAzureBlobStorageLocation,
            >,
        >,
        /// The column delimiter. Defaults to `,`.
        pub column_delimiter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The compression codec used to read/write text files. Valid values are `None`, `bzip2`, `gzip`, `deflate`, `ZipDeflate`, `TarGzip`, `Tar`, `snappy` and `lz4`. Please note these values are case sensitive.
        pub compression_codec: pulumi_gestalt_rust::Output<Option<String>>,
        /// The compression ratio for the Data Factory Dataset. Valid values are `Fastest` or `Optimal`. Please note these values are case sensitive.
        pub compression_level: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Dataset.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The encoding format for the file.
        pub encoding: pulumi_gestalt_rust::Output<Option<String>>,
        /// The escape character. Defaults to `\`.
        pub escape_character: pulumi_gestalt_rust::Output<Option<String>>,
        /// When used as input, treat the first row of data as headers. When used as output, write the headers into the output as the first row of data. Defaults to `false`.
        pub first_row_as_header: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        pub folder: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `http_server_location` block as defined below.
        ///
        /// The following supported arguments are specific to Delimited Text Dataset:
        pub http_server_location: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::DatasetDelimitedTextHttpServerLocation,
            >,
        >,
        /// The Data Factory Linked Service name in which to associate the Dataset with.
        pub linked_service_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Data Factory Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The null value string. Defaults to `""`.
        pub null_value: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of parameters to associate with the Data Factory Dataset.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The quote character. Defaults to `"`.
        pub quote_character: pulumi_gestalt_rust::Output<Option<String>>,
        /// The row delimiter. Defaults to any of the following values on read: `\r\n`, `\r`, `\n`, and `\n` or `\r\n` on write by mapping data flow and Copy activity respectively.
        pub row_delimiter: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `schema_column` block as defined below.
        pub schema_columns: pulumi_gestalt_rust::Output<
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatasetDelimitedTextArgs,
    ) -> DatasetDelimitedTextResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let additional_properties_binding = args
            .additional_properties
            .get_output(context);
        let annotations_binding = args.annotations.get_output(context);
        let azure_blob_fs_location_binding = args
            .azure_blob_fs_location
            .get_output(context);
        let azure_blob_storage_location_binding = args
            .azure_blob_storage_location
            .get_output(context);
        let column_delimiter_binding = args.column_delimiter.get_output(context);
        let compression_codec_binding = args.compression_codec.get_output(context);
        let compression_level_binding = args.compression_level.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let encoding_binding = args.encoding.get_output(context);
        let escape_character_binding = args.escape_character.get_output(context);
        let first_row_as_header_binding = args.first_row_as_header.get_output(context);
        let folder_binding = args.folder.get_output(context);
        let http_server_location_binding = args.http_server_location.get_output(context);
        let linked_service_name_binding = args.linked_service_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let null_value_binding = args.null_value.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let quote_character_binding = args.quote_character.get_output(context);
        let row_delimiter_binding = args.row_delimiter.get_output(context);
        let schema_columns_binding = args.schema_columns.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/datasetDelimitedText:DatasetDelimitedText".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "additionalProperties".into(),
                    value: &additional_properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureBlobFsLocation".into(),
                    value: &azure_blob_fs_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureBlobStorageLocation".into(),
                    value: &azure_blob_storage_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "columnDelimiter".into(),
                    value: &column_delimiter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compressionCodec".into(),
                    value: &compression_codec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compressionLevel".into(),
                    value: &compression_level_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encoding".into(),
                    value: &encoding_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "escapeCharacter".into(),
                    value: &escape_character_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firstRowAsHeader".into(),
                    value: &first_row_as_header_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: &folder_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpServerLocation".into(),
                    value: &http_server_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "linkedServiceName".into(),
                    value: &linked_service_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nullValue".into(),
                    value: &null_value_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quoteCharacter".into(),
                    value: &quote_character_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rowDelimiter".into(),
                    value: &row_delimiter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schemaColumns".into(),
                    value: &schema_columns_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatasetDelimitedTextResult {
            additional_properties: o.get_field("additionalProperties"),
            annotations: o.get_field("annotations"),
            azure_blob_fs_location: o.get_field("azureBlobFsLocation"),
            azure_blob_storage_location: o.get_field("azureBlobStorageLocation"),
            column_delimiter: o.get_field("columnDelimiter"),
            compression_codec: o.get_field("compressionCodec"),
            compression_level: o.get_field("compressionLevel"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            encoding: o.get_field("encoding"),
            escape_character: o.get_field("escapeCharacter"),
            first_row_as_header: o.get_field("firstRowAsHeader"),
            folder: o.get_field("folder"),
            http_server_location: o.get_field("httpServerLocation"),
            linked_service_name: o.get_field("linkedServiceName"),
            name: o.get_field("name"),
            null_value: o.get_field("nullValue"),
            parameters: o.get_field("parameters"),
            quote_character: o.get_field("quoteCharacter"),
            row_delimiter: o.get_field("rowDelimiter"),
            schema_columns: o.get_field("schemaColumns"),
        }
    }
}
