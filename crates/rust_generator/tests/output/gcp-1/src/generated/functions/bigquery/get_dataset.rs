#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dataset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatasetArgs {
        /// The dataset ID.
        #[builder(into)]
        pub dataset_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDatasetResult {
        pub accesses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::bigquery::GetDatasetAccess>,
        >,
        pub creation_time: pulumi_gestalt_rust::Output<i32>,
        pub dataset_id: pulumi_gestalt_rust::Output<String>,
        pub default_collation: pulumi_gestalt_rust::Output<String>,
        pub default_encryption_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::bigquery::GetDatasetDefaultEncryptionConfiguration,
            >,
        >,
        pub default_partition_expiration_ms: pulumi_gestalt_rust::Output<i32>,
        pub default_table_expiration_ms: pulumi_gestalt_rust::Output<i32>,
        pub delete_contents_on_destroy: pulumi_gestalt_rust::Output<bool>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub external_catalog_dataset_options: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::bigquery::GetDatasetExternalCatalogDatasetOption,
            >,
        >,
        pub external_dataset_references: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::bigquery::GetDatasetExternalDatasetReference>,
        >,
        pub friendly_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub is_case_insensitive: pulumi_gestalt_rust::Output<bool>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub last_modified_time: pulumi_gestalt_rust::Output<i32>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub max_time_travel_hours: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub resource_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub storage_billing_model: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDatasetArgs,
    ) -> GetDatasetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dataset_id_binding = args.dataset_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:bigquery/getDataset:getDataset".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDatasetResult {
            accesses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accesses"),
            ),
            creation_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTime"),
            ),
            dataset_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("datasetId"),
            ),
            default_collation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultCollation"),
            ),
            default_encryption_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultEncryptionConfigurations"),
            ),
            default_partition_expiration_ms: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultPartitionExpirationMs"),
            ),
            default_table_expiration_ms: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultTableExpirationMs"),
            ),
            delete_contents_on_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteContentsOnDestroy"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            external_catalog_dataset_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalCatalogDatasetOptions"),
            ),
            external_dataset_references: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalDatasetReferences"),
            ),
            friendly_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("friendlyName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            is_case_insensitive: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isCaseInsensitive"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            last_modified_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedTime"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            max_time_travel_hours: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxTimeTravelHours"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            resource_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTags"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            storage_billing_model: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageBillingModel"),
            ),
        }
    }
}
