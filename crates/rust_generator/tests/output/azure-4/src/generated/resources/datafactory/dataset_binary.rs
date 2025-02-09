/// Manages a Data Factory Binary Dataset inside an Azure Data Factory.
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
///             .name("example")
///             .build_struct(),
///     );
///     let exampleDatasetBinary = dataset_binary::create(
///         "exampleDatasetBinary",
///         DatasetBinaryArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .linked_service_name("${exampleLinkedServiceSftp.name}")
///             .name("example")
///             .sftp_server_location(
///                 DatasetBinarySftpServerLocation::builder()
///                     .filename("**")
///                     .path("/test/")
///                     .build_struct(),
///             )
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
///     let exampleLinkedServiceSftp = linked_service_sftp::create(
///         "exampleLinkedServiceSftp",
///         LinkedServiceSftpArgs::builder()
///             .authentication_type("Basic")
///             .data_factory_id("${exampleFactory.id}")
///             .host("http://www.bing.com")
///             .name("example")
///             .password("bar")
///             .port(22)
///             .username("foo")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Binary Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/datasetBinary:DatasetBinary example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/datasets/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dataset_binary {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetBinaryArgs {
        /// A map of additional properties to associate with the Data Factory Binary Dataset.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Binary Dataset.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A `azure_blob_storage_location` block as defined below.
        #[builder(into, default)]
        pub azure_blob_storage_location: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::DatasetBinaryAzureBlobStorageLocation,
            >,
        >,
        /// A `compression` block as defined below.
        #[builder(into, default)]
        pub compression: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::DatasetBinaryCompression>,
        >,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Dataset.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        #[builder(into, default)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `http_server_location` block as defined below.
        #[builder(into, default)]
        pub http_server_location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::DatasetBinaryHttpServerLocation>,
        >,
        /// The Data Factory Linked Service name in which to associate the Binary Dataset with.
        #[builder(into)]
        pub linked_service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Data Factory Binary Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of parameters to associate with the Data Factory Binary Dataset.
        ///
        /// The following supported locations for a Binary Dataset. One of these should be specified:
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `sftp_server_location` block as defined below.
        #[builder(into, default)]
        pub sftp_server_location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafactory::DatasetBinarySftpServerLocation>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatasetBinaryResult {
        /// A map of additional properties to associate with the Data Factory Binary Dataset.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Binary Dataset.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A `azure_blob_storage_location` block as defined below.
        pub azure_blob_storage_location: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::DatasetBinaryAzureBlobStorageLocation,
            >,
        >,
        /// A `compression` block as defined below.
        pub compression: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::DatasetBinaryCompression>,
        >,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Dataset.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The folder that this Dataset is in. If not specified, the Dataset will appear at the root level.
        pub folder: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `http_server_location` block as defined below.
        pub http_server_location: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::DatasetBinaryHttpServerLocation>,
        >,
        /// The Data Factory Linked Service name in which to associate the Binary Dataset with.
        pub linked_service_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Data Factory Binary Dataset. Changing this forces a new resource to be created. Must be globally unique. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of parameters to associate with the Data Factory Binary Dataset.
        ///
        /// The following supported locations for a Binary Dataset. One of these should be specified:
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `sftp_server_location` block as defined below.
        pub sftp_server_location: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafactory::DatasetBinarySftpServerLocation>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DatasetBinaryArgs,
    ) -> DatasetBinaryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding_1 = args
            .additional_properties
            .get_output(context);
        let additional_properties_binding = additional_properties_binding_1.get_inner();
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let azure_blob_storage_location_binding_1 = args
            .azure_blob_storage_location
            .get_output(context);
        let azure_blob_storage_location_binding = azure_blob_storage_location_binding_1
            .get_inner();
        let compression_binding_1 = args.compression.get_output(context);
        let compression_binding = compression_binding_1.get_inner();
        let data_factory_id_binding_1 = args.data_factory_id.get_output(context);
        let data_factory_id_binding = data_factory_id_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let folder_binding_1 = args.folder.get_output(context);
        let folder_binding = folder_binding_1.get_inner();
        let http_server_location_binding_1 = args
            .http_server_location
            .get_output(context);
        let http_server_location_binding = http_server_location_binding_1.get_inner();
        let linked_service_name_binding_1 = args.linked_service_name.get_output(context);
        let linked_service_name_binding = linked_service_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let sftp_server_location_binding_1 = args
            .sftp_server_location
            .get_output(context);
        let sftp_server_location_binding = sftp_server_location_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/datasetBinary:DatasetBinary".into(),
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
                    name: "azureBlobStorageLocation".into(),
                    value: &azure_blob_storage_location_binding,
                },
                register_interface::ObjectField {
                    name: "compression".into(),
                    value: &compression_binding,
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
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "sftpServerLocation".into(),
                    value: &sftp_server_location_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DatasetBinaryResult {
            additional_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalProperties"),
            ),
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            azure_blob_storage_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureBlobStorageLocation"),
            ),
            compression: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("compression"),
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
            http_server_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpServerLocation"),
            ),
            linked_service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkedServiceName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            sftp_server_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sftpServerLocation"),
            ),
        }
    }
}
