/// Describes a BigQuery linked dataset
///
///
/// To get more information about LinkedDataset, see:
///
/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/locations.buckets.links)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/logging/docs/apis)
///
/// ## Example Usage
///
/// ### Logging Linked Dataset Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let loggingLinkedDataset = project_bucket_config::create(
///         "loggingLinkedDataset",
///         ProjectBucketConfigArgs::builder()
///             .bucket_id("my-bucket")
///             .enable_analytics(true)
///             .location("global")
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let loggingLinkedDatasetLinkedDataset = linked_dataset::create(
///         "loggingLinkedDatasetLinkedDataset",
///         LinkedDatasetArgs::builder()
///             .bucket("${loggingLinkedDataset.id}")
///             .description("Linked dataset test")
///             .link_id("mylink")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Logging Linked Dataset All Params
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let loggingLinkedDataset = project_bucket_config::create(
///         "loggingLinkedDataset",
///         ProjectBucketConfigArgs::builder()
///             .bucket_id("my-bucket")
///             .enable_analytics(true)
///             .location("global")
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let loggingLinkedDatasetLinkedDataset = linked_dataset::create(
///         "loggingLinkedDatasetLinkedDataset",
///         LinkedDatasetArgs::builder()
///             .bucket("my-bucket")
///             .description("Linked dataset test")
///             .link_id("mylink")
///             .location("global")
///             .parent("projects/my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// LinkedDataset can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/buckets/{{bucket}}/links/{{link_id}}`
///
/// When using the `pulumi import` command, LinkedDataset can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/linkedDataset:LinkedDataset default {{parent}}/locations/{{location}}/buckets/{{bucket}}/links/{{link_id}}
/// ```
///
pub mod linked_dataset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedDatasetArgs {
        /// The information of a BigQuery Dataset. When a link is created, a BigQuery dataset is created along
        /// with it, in the same project as the LogBucket it's linked to. This dataset will also have BigQuery
        /// Views corresponding to the LogViews in the bucket.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bigquery_datasets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::logging::LinkedDatasetBigqueryDataset>>,
        >,
        /// The bucket to which the linked dataset is attached.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Describes this link. The maximum length of the description is 8000 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The id of the linked dataset.
        #[builder(into)]
        pub link_id: pulumi_wasm_rust::Output<String>,
        /// The location of the linked dataset.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The parent of the linked dataset.
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LinkedDatasetResult {
        /// The information of a BigQuery Dataset. When a link is created, a BigQuery dataset is created along
        /// with it, in the same project as the LogBucket it's linked to. This dataset will also have BigQuery
        /// Views corresponding to the LogViews in the bucket.
        /// Structure is documented below.
        pub bigquery_datasets: pulumi_wasm_rust::Output<
            Vec<super::super::types::logging::LinkedDatasetBigqueryDataset>,
        >,
        /// The bucket to which the linked dataset is attached.
        ///
        ///
        /// - - -
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Output only. The creation timestamp of the link. A timestamp in RFC3339 UTC "Zulu" format,
        /// with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z"
        /// and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Describes this link. The maximum length of the description is 8000 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Output only. The linked dataset lifecycle state.
        pub lifecycle_state: pulumi_wasm_rust::Output<String>,
        /// The id of the linked dataset.
        pub link_id: pulumi_wasm_rust::Output<String>,
        /// The location of the linked dataset.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the linked dataset. The name can have up to 100 characters. A valid link id
        /// (at the end of the link name) must only have alphanumeric characters and underscores within it.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent of the linked dataset.
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LinkedDatasetArgs) -> LinkedDatasetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bigquery_datasets_binding = args.bigquery_datasets.get_inner();
        let bucket_binding = args.bucket.get_inner();
        let description_binding = args.description.get_inner();
        let link_id_binding = args.link_id.get_inner();
        let location_binding = args.location.get_inner();
        let parent_binding = args.parent.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/linkedDataset:LinkedDataset".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bigqueryDatasets".into(),
                    value: &bigquery_datasets_binding,
                },
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "linkId".into(),
                    value: &link_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bigqueryDatasets".into(),
                },
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "lifecycleState".into(),
                },
                register_interface::ResultField {
                    name: "linkId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkedDatasetResult {
            bigquery_datasets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bigqueryDatasets").unwrap(),
            ),
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            lifecycle_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecycleState").unwrap(),
            ),
            link_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linkId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
        }
    }
}
