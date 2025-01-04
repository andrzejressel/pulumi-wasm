pub mod get_dataset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetArgs {
        /// The dataset ID.
        #[builder(into)]
        pub dataset_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDatasetResult {
        pub accesses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::bigquery::GetDatasetAccess>,
        >,
        pub creation_time: pulumi_wasm_rust::Output<i32>,
        pub dataset_id: pulumi_wasm_rust::Output<String>,
        pub default_collation: pulumi_wasm_rust::Output<String>,
        pub default_encryption_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::bigquery::GetDatasetDefaultEncryptionConfiguration,
            >,
        >,
        pub default_partition_expiration_ms: pulumi_wasm_rust::Output<i32>,
        pub default_table_expiration_ms: pulumi_wasm_rust::Output<i32>,
        pub delete_contents_on_destroy: pulumi_wasm_rust::Output<bool>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub external_catalog_dataset_options: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::bigquery::GetDatasetExternalCatalogDatasetOption,
            >,
        >,
        pub external_dataset_references: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::bigquery::GetDatasetExternalDatasetReference>,
        >,
        pub friendly_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub is_case_insensitive: pulumi_wasm_rust::Output<bool>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub last_modified_time: pulumi_wasm_rust::Output<i32>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub max_time_travel_hours: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub resource_tags: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub storage_billing_model: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDatasetArgs) -> GetDatasetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dataset_id_binding = args.dataset_id.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:bigquery/getDataset:getDataset".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "datasetId".into(),
                    value: &dataset_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accesses".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "datasetId".into(),
                },
                register_interface::ResultField {
                    name: "defaultCollation".into(),
                },
                register_interface::ResultField {
                    name: "defaultEncryptionConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "defaultPartitionExpirationMs".into(),
                },
                register_interface::ResultField {
                    name: "defaultTableExpirationMs".into(),
                },
                register_interface::ResultField {
                    name: "deleteContentsOnDestroy".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "externalCatalogDatasetOptions".into(),
                },
                register_interface::ResultField {
                    name: "externalDatasetReferences".into(),
                },
                register_interface::ResultField {
                    name: "friendlyName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "isCaseInsensitive".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTime".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maxTimeTravelHours".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "resourceTags".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "storageBillingModel".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDatasetResult {
            accesses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accesses").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            dataset_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("datasetId").unwrap(),
            ),
            default_collation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultCollation").unwrap(),
            ),
            default_encryption_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultEncryptionConfigurations").unwrap(),
            ),
            default_partition_expiration_ms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPartitionExpirationMs").unwrap(),
            ),
            default_table_expiration_ms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultTableExpirationMs").unwrap(),
            ),
            delete_contents_on_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteContentsOnDestroy").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            external_catalog_dataset_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalCatalogDatasetOptions").unwrap(),
            ),
            external_dataset_references: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalDatasetReferences").unwrap(),
            ),
            friendly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("friendlyName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            is_case_insensitive: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isCaseInsensitive").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            last_modified_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTime").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            max_time_travel_hours: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxTimeTravelHours").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTags").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            storage_billing_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageBillingModel").unwrap(),
            ),
        }
    }
}
