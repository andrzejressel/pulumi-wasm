/// A Hl7V2Store is a datastore inside a Healthcare dataset that conforms to the FHIR (https://www.hl7.org/hl7V2/STU3/)
/// standard for Healthcare information exchange
///
///
/// To get more information about Hl7V2Store, see:
///
/// * [API documentation](https://cloud.google.com/healthcare/docs/reference/rest/v1/projects.locations.datasets.hl7V2Stores)
/// * How-to Guides
///     * [Creating a HL7v2 Store](https://cloud.google.com/healthcare/docs/how-tos/hl7v2)
///
/// ## Example Usage
///
/// ### Healthcare Hl7 V2 Store Basic
///
///
/// ```yaml
/// resources:
///   store:
///     type: gcp:healthcare:Hl7Store
///     properties:
///       name: example-hl7-v2-store
///       dataset: ${dataset.id}
///       rejectDuplicateMessage: true
///       notificationConfigs:
///         - pubsubTopic: ${topic.id}
///       labels:
///         label1: labelvalue1
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: hl7-v2-notifications
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example-dataset
///       location: us-central1
/// ```
/// ### Healthcare Hl7 V2 Store Parser Config
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dataset = dataset::create(
///         "dataset",
///         DatasetArgs::builder()
///             .location("us-central1")
///             .name("example-dataset")
///             .build_struct(),
///     );
///     let store = hl_7_store::create(
///         "store",
///         Hl7StoreArgs::builder()
///             .dataset("${dataset.id}")
///             .name("example-hl7-v2-store")
///             .parser_config(
///                 Hl7StoreParserConfig::builder()
///                     .allowNullHeader(false)
///                     .schema(
///                         "{\n  \"schemas\": [{\n    \"messageSchemaConfigs\": {\n      \"ADT_A01\": {\n        \"name\": \"ADT_A01\",\n        \"minOccurs\": 1,\n        \"maxOccurs\": 1,\n        \"members\": [{\n            \"segment\": {\n              \"type\": \"MSH\",\n              \"minOccurs\": 1,\n              \"maxOccurs\": 1\n            }\n          },\n          {\n            \"segment\": {\n              \"type\": \"EVN\",\n              \"minOccurs\": 1,\n              \"maxOccurs\": 1\n            }\n          },\n          {\n            \"segment\": {\n              \"type\": \"PID\",\n              \"minOccurs\": 1,\n              \"maxOccurs\": 1\n            }\n          },\n          {\n            \"segment\": {\n              \"type\": \"ZPD\",\n              \"minOccurs\": 1,\n              \"maxOccurs\": 1\n            }\n          },\n          {\n            \"segment\": {\n              \"type\": \"OBX\"\n            }\n          },\n          {\n            \"group\": {\n              \"name\": \"PROCEDURE\",\n              \"members\": [{\n                  \"segment\": {\n                    \"type\": \"PR1\",\n                    \"minOccurs\": 1,\n                    \"maxOccurs\": 1\n                  }\n                },\n                {\n                  \"segment\": {\n                    \"type\": \"ROL\"\n                  }\n                }\n              ]\n            }\n          },\n          {\n            \"segment\": {\n              \"type\": \"PDA\",\n              \"maxOccurs\": 1\n            }\n          }\n        ]\n      }\n    }\n  }],\n  \"types\": [{\n    \"type\": [{\n        \"name\": \"ZPD\",\n        \"primitive\": \"VARIES\"\n      }\n\n    ]\n  }],\n  \"ignoreMinOccurs\": true\n}\n",
///                     )
///                     .segmentTerminator("Jw==")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Healthcare Hl7 V2 Store Unschematized
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dataset = dataset::create(
///         "dataset",
///         DatasetArgs::builder()
///             .location("us-central1")
///             .name("example-dataset")
///             .build_struct(),
///     );
///     let store = hl_7_store::create(
///         "store",
///         Hl7StoreArgs::builder()
///             .dataset("${dataset.id}")
///             .name("example-hl7-v2-store")
///             .parser_config(
///                 Hl7StoreParserConfig::builder()
///                     .allowNullHeader(false)
///                     .segmentTerminator("Jw==")
///                     .version("V2")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Hl7V2Store can be imported using any of these accepted formats:
///
/// * `{{dataset}}/hl7V2Stores/{{name}}`
///
/// * `{{dataset}}/{{name}}`
///
/// When using the `pulumi import` command, Hl7V2Store can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:healthcare/hl7Store:Hl7Store default {{dataset}}/hl7V2Stores/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:healthcare/hl7Store:Hl7Store default {{dataset}}/{{name}}
/// ```
///
pub mod hl_7_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Hl7StoreArgs {
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        ///
        ///
        /// - - -
        #[builder(into)]
        pub dataset: pulumi_wasm_rust::Output<String>,
        /// User-supplied key-value pairs used to organize HL7v2 stores.
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
        /// The resource name for the Hl7V2Store.
        /// ** Changing this property may recreate the Hl7v2 store (removing all data) **
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// (Optional, Deprecated)
        /// A nested object resource.
        /// Structure is documented below.
        ///
        /// > **Warning:** `notification_config` is deprecated and will be removed in a future major release. Use `notification_configs` instead.
        #[builder(into, default)]
        pub notification_config: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::Hl7StoreNotificationConfig>,
        >,
        /// A list of notification configs. Each configuration uses a filter to determine whether to publish a
        /// message (both Ingest & Create) on the corresponding notification destination. Only the message name
        /// is sent as part of the notification. Supplied by the client.
        /// Structure is documented below.
        #[builder(into, default)]
        pub notification_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::healthcare::Hl7StoreNotificationConfigs>>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub parser_config: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::Hl7StoreParserConfig>,
        >,
        /// Determines whether duplicate messages are allowed.
        #[builder(into, default)]
        pub reject_duplicate_message: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct Hl7StoreResult {
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
        /// User-supplied key-value pairs used to organize HL7v2 stores.
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
        /// The resource name for the Hl7V2Store.
        /// ** Changing this property may recreate the Hl7v2 store (removing all data) **
        pub name: pulumi_wasm_rust::Output<String>,
        /// (Optional, Deprecated)
        /// A nested object resource.
        /// Structure is documented below.
        ///
        /// > **Warning:** `notification_config` is deprecated and will be removed in a future major release. Use `notification_configs` instead.
        pub notification_config: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::Hl7StoreNotificationConfig>,
        >,
        /// A list of notification configs. Each configuration uses a filter to determine whether to publish a
        /// message (both Ingest & Create) on the corresponding notification destination. Only the message name
        /// is sent as part of the notification. Supplied by the client.
        /// Structure is documented below.
        pub notification_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::healthcare::Hl7StoreNotificationConfigs>>,
        >,
        /// A nested object resource.
        /// Structure is documented below.
        pub parser_config: pulumi_wasm_rust::Output<
            super::super::types::healthcare::Hl7StoreParserConfig,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Determines whether duplicate messages are allowed.
        pub reject_duplicate_message: pulumi_wasm_rust::Output<Option<bool>>,
        /// The fully qualified name of this dataset
        pub self_link: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: Hl7StoreArgs) -> Hl7StoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dataset_binding = args.dataset.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let notification_config_binding = args.notification_config.get_inner();
        let notification_configs_binding = args.notification_configs.get_inner();
        let parser_config_binding = args.parser_config.get_inner();
        let reject_duplicate_message_binding = args.reject_duplicate_message.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:healthcare/hl7Store:Hl7Store".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "notificationConfigs".into(),
                    value: &notification_configs_binding,
                },
                register_interface::ObjectField {
                    name: "parserConfig".into(),
                    value: &parser_config_binding,
                },
                register_interface::ObjectField {
                    name: "rejectDuplicateMessage".into(),
                    value: &reject_duplicate_message_binding,
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
                    name: "notificationConfigs".into(),
                },
                register_interface::ResultField {
                    name: "parserConfig".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "rejectDuplicateMessage".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        Hl7StoreResult {
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
            notification_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationConfigs").unwrap(),
            ),
            parser_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parserConfig").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            reject_duplicate_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rejectDuplicateMessage").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
        }
    }
}
