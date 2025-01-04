/// A DicomStore is a datastore inside a Healthcare dataset that conforms to the DICOM
/// (https://www.dicomstandard.org/about/) standard for Healthcare information exchange
///
///
/// To get more information about DicomStore, see:
///
/// * [API documentation](https://cloud.google.com/healthcare/docs/reference/rest/v1/projects.locations.datasets.dicomStores)
/// * How-to Guides
///     * [Creating a DICOM store](https://cloud.google.com/healthcare/docs/how-tos/dicom)
///
/// ## Example Usage
///
/// ### Healthcare Dicom Store Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:healthcare:DicomStore
///     properties:
///       name: example-dicom-store
///       dataset: ${dataset.id}
///       notificationConfig:
///         pubsubTopic: ${topic.id}
///       labels:
///         label1: labelvalue1
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: dicom-notifications
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example-dataset
///       location: us-central1
/// ```
/// ### Healthcare Dicom Store Bq Stream
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:healthcare:DicomStore
///     properties:
///       name: example-dicom-store
///       dataset: ${dataset.id}
///       notificationConfig:
///         pubsubTopic: ${topic.id}
///         sendForBulkImport: true
///       labels:
///         label1: labelvalue1
///       streamConfigs:
///         - bigqueryDestination:
///             tableUri: bq://${bqDataset.project}.${bqDataset.datasetId}.${bqTable.tableId}
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: dicom-notifications
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example-dataset
///       location: us-central1
///   bqDataset:
///     type: gcp:bigquery:Dataset
///     name: bq_dataset
///     properties:
///       datasetId: dicom_bq_ds
///       friendlyName: test
///       description: This is a test description
///       location: US
///       deleteContentsOnDestroy: true
///   bqTable:
///     type: gcp:bigquery:Table
///     name: bq_table
///     properties:
///       deletionProtection: false
///       datasetId: ${bqDataset.datasetId}
///       tableId: dicom_bq_tb
/// ```
///
/// ## Import
///
/// DicomStore can be imported using any of these accepted formats:
///
/// * `{{dataset}}/dicomStores/{{name}}`
///
/// * `{{dataset}}/{{name}}`
///
/// When using the `pulumi import` command, DicomStore can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:healthcare/dicomStore:DicomStore default {{dataset}}/dicomStores/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:healthcare/dicomStore:DicomStore default {{dataset}}/{{name}}
/// ```
///
pub mod dicom_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DicomStoreArgs {
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        ///
        ///
        /// - - -
        #[builder(into)]
        pub dataset: pulumi_wasm_rust::Output<String>,
        /// User-supplied key-value pairs used to organize DICOM stores.
        /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must
        /// conform to the following PCRE regular expression: [\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}
        /// Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128
        /// bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}
        /// No more than 64 labels can be associated with a given store.
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the DicomStore.
        /// ** Changing this property may recreate the Dicom store (removing all data) **
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub notification_config: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::DicomStoreNotificationConfig>,
        >,
        /// To enable streaming to BigQuery, configure the streamConfigs object in your DICOM store.
        /// streamConfigs is an array, so you can specify multiple BigQuery destinations. You can stream metadata from a single DICOM store to up to five BigQuery tables in a BigQuery dataset.
        /// Structure is documented below.
        #[builder(into, default)]
        pub stream_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::healthcare::DicomStoreStreamConfig>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DicomStoreResult {
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        ///
        ///
        /// - - -
        pub dataset: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// User-supplied key-value pairs used to organize DICOM stores.
        /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must
        /// conform to the following PCRE regular expression: [\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}
        /// Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128
        /// bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}
        /// No more than 64 labels can be associated with a given store.
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the DicomStore.
        /// ** Changing this property may recreate the Dicom store (removing all data) **
        pub name: pulumi_wasm_rust::Output<String>,
        /// A nested object resource.
        /// Structure is documented below.
        pub notification_config: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::DicomStoreNotificationConfig>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The fully qualified name of this dataset
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// To enable streaming to BigQuery, configure the streamConfigs object in your DICOM store.
        /// streamConfigs is an array, so you can specify multiple BigQuery destinations. You can stream metadata from a single DICOM store to up to five BigQuery tables in a BigQuery dataset.
        /// Structure is documented below.
        pub stream_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::healthcare::DicomStoreStreamConfig>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DicomStoreArgs) -> DicomStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dataset_binding = args.dataset.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let notification_config_binding = args.notification_config.get_inner();
        let stream_configs_binding = args.stream_configs.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:healthcare/dicomStore:DicomStore".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationConfig".into(),
                    value: &notification_config_binding,
                },
                register_interface::ObjectField {
                    name: "streamConfigs".into(),
                    value: &stream_configs_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataset".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationConfig".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "streamConfigs".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DicomStoreResult {
            dataset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataset").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationConfig").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            stream_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamConfigs").unwrap(),
            ),
        }
    }
}
