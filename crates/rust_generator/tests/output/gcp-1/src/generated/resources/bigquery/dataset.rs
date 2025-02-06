/// ## Example Usage
///
/// ### Bigquery Dataset Basic
///
///
/// ```yaml
/// resources:
///   dataset:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: example_dataset
///       friendlyName: test
///       description: This is a test description
///       location: EU
///       defaultTableExpirationMs: 3.6e+06
///       labels:
///         env: default
///       accesses:
///         - role: OWNER
///           userByEmail: ${bqowner.email}
///         - role: READER
///           domain: hashicorp.com
///   bqowner:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: bqowner
/// ```
/// ### Bigquery Dataset Cmek
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cryptoKey = crypto_key::create(
///         "cryptoKey",
///         CryptoKeyArgs::builder()
///             .key_ring("${keyRing.id}")
///             .name("example-key")
///             .build_struct(),
///     );
///     let dataset = dataset::create(
///         "dataset",
///         DatasetArgs::builder()
///             .dataset_id("example_dataset")
///             .default_encryption_configuration(
///                 DatasetDefaultEncryptionConfiguration::builder()
///                     .kmsKeyName("${cryptoKey.id}")
///                     .build_struct(),
///             )
///             .default_table_expiration_ms(3600000)
///             .description("This is a test description")
///             .friendly_name("test")
///             .location("US")
///             .build_struct(),
///     );
///     let keyRing = key_ring::create(
///         "keyRing",
///         KeyRingArgs::builder().location("us").name("example-keyring").build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Dataset Authorized Dataset
///
///
/// ```yaml
/// resources:
///   public:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: public
///       friendlyName: test
///       description: This dataset is public
///       location: EU
///       defaultTableExpirationMs: 3.6e+06
///       labels:
///         env: default
///       accesses:
///         - role: OWNER
///           userByEmail: ${bqowner.email}
///         - role: READER
///           domain: hashicorp.com
///   dataset:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: private
///       friendlyName: test
///       description: This dataset is private
///       location: EU
///       defaultTableExpirationMs: 3.6e+06
///       labels:
///         env: default
///       accesses:
///         - role: OWNER
///           userByEmail: ${bqowner.email}
///         - role: READER
///           domain: hashicorp.com
///         - dataset:
///             dataset:
///               projectId: ${public.project}
///               datasetId: ${public.datasetId}
///             targetTypes:
///               - VIEWS
///   bqowner:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: bqowner
/// ```
/// ### Bigquery Dataset Authorized Routine
///
///
/// ```yaml
/// resources:
///   public:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: public_dataset
///       description: This dataset is public
///   publicRoutine:
///     type: gcp:bigquery:Routine
///     name: public
///     properties:
///       datasetId: ${public.datasetId}
///       routineId: public_routine
///       routineType: TABLE_VALUED_FUNCTION
///       language: SQL
///       definitionBody: |
///         SELECT 1 + value AS value
///       arguments:
///         - name: value
///           argumentKind: FIXED_TYPE
///           dataType:
///             fn::toJSON:
///               typeKind: INT64
///       returnTableType:
///         fn::toJSON:
///           columns:
///             - name: value
///               type:
///                 typeKind: INT64
///   private:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: private_dataset
///       description: This dataset is private
///       accesses:
///         - role: OWNER
///           userByEmail: my@service-account.com
///         - routine:
///             projectId: ${publicRoutine.project}
///             datasetId: ${publicRoutine.datasetId}
///             routineId: ${publicRoutine.routineId}
/// ```
/// ### Bigquery Dataset External Reference Aws
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dataset = dataset::create(
///         "dataset",
///         DatasetArgs::builder()
///             .dataset_id("example_dataset")
///             .description("This is a test description")
///             .external_dataset_reference(
///                 DatasetExternalDatasetReference::builder()
///                     .connection(
///                         "projects/project/locations/aws-us-east-1/connections/connection",
///                     )
///                     .externalSource(
///                         "aws-glue://arn:aws:glue:us-east-1:999999999999:database/database",
///                     )
///                     .build_struct(),
///             )
///             .friendly_name("test")
///             .location("aws-us-east-1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Dataset External Catalog Dataset Options
///
///
/// ```yaml
/// resources:
///   dataset:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: example_dataset
///       friendlyName: test
///       description: This is a test description
///       location: US
///       externalCatalogDatasetOptions:
///         parameters:
///           dataset_owner: test_dataset_owner
///         defaultStorageLocationUri: gs://test_dataset/tables
/// ```
///
/// ## Import
///
/// Dataset can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/datasets/{{dataset_id}}`
///
/// * `{{project}}/{{dataset_id}}`
///
/// * `{{dataset_id}}`
///
/// When using the `pulumi import` command, Dataset can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/dataset:Dataset default projects/{{project}}/datasets/{{dataset_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/dataset:Dataset default {{project}}/{{dataset_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/dataset:Dataset default {{dataset_id}}
/// ```
///
pub mod dataset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetArgs {
        /// An array of objects that define dataset access for one or more entities.
        /// Structure is documented below.
        #[builder(into, default)]
        pub accesses: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::bigquery::DatasetAccess>>,
        >,
        /// A unique ID for this dataset, without the project name. The ID
        /// must contain only letters (a-z, A-Z), numbers (0-9), or
        /// underscores (_). The maximum length is 1,024 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub dataset_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Defines the default collation specification of future tables created
        /// in the dataset. If a table is created in this dataset without table-level
        /// default collation, then the table inherits the dataset default collation,
        /// which is applied to the string fields that do not have explicit collation
        /// specified. A change to this field affects only tables created afterwards,
        /// and does not alter the existing tables.
        /// The following values are supported:
        /// - 'und:ci': undetermined locale, case insensitive.
        /// - '': empty string. Default to case-sensitive behavior.
        #[builder(into, default)]
        pub default_collation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The default encryption key for all tables in the dataset. Once this property is set,
        /// all newly-created partitioned tables in the dataset will have encryption key set to
        /// this value, unless table creation request (or query) overrides the key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub default_encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::DatasetDefaultEncryptionConfiguration>,
        >,
        /// The default partition expiration for all partitioned tables in
        /// the dataset, in milliseconds.
        /// Once this property is set, all newly-created partitioned tables in
        /// the dataset will have an `expirationMs` property in the `timePartitioning`
        /// settings set to this value, and changing the value will only
        /// affect new tables, not existing ones. The storage in a partition will
        /// have an expiration time of its partition time plus this value.
        /// Setting this property overrides the use of `defaultTableExpirationMs`
        /// for partitioned tables: only one of `defaultTableExpirationMs` and
        /// `defaultPartitionExpirationMs` will be used for any new partitioned
        /// table. If you provide an explicit `timePartitioning.expirationMs` when
        /// creating or updating a partitioned table, that value takes precedence
        /// over the default partition expiration time indicated by this property.
        #[builder(into, default)]
        pub default_partition_expiration_ms: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The default lifetime of all tables in the dataset, in milliseconds.
        /// The minimum value is 3600000 milliseconds (one hour).
        /// Once this property is set, all newly-created tables in the dataset
        /// will have an `expirationTime` property set to the creation time plus
        /// the value in this property, and changing the value will only affect
        /// new tables, not existing ones. When the `expirationTime` for a given
        /// table is reached, that table will be deleted automatically.
        /// If a table's `expirationTime` is modified or removed before the
        /// table expires, or if you provide an explicit `expirationTime` when
        /// creating a table, that value takes precedence over the default
        /// expiration time indicated by this property.
        #[builder(into, default)]
        pub default_table_expiration_ms: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// If set to `true`, delete all the tables in the
        /// dataset when destroying the resource; otherwise,
        /// destroying the resource will fail if tables are present.
        #[builder(into, default)]
        pub delete_contents_on_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A user-friendly description of the dataset
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Options defining open source compatible datasets living in the BigQuery catalog. Contains
        /// metadata of open source database, schema or namespace represented by the current dataset.
        /// Structure is documented below.
        #[builder(into, default)]
        pub external_catalog_dataset_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::DatasetExternalCatalogDatasetOptions>,
        >,
        /// Information about the external metadata storage where the dataset is defined.
        /// Structure is documented below.
        #[builder(into, default)]
        pub external_dataset_reference: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::DatasetExternalDatasetReference>,
        >,
        /// A descriptive name for the dataset
        #[builder(into, default)]
        pub friendly_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// TRUE if the dataset and its table names are case-insensitive, otherwise FALSE.
        /// By default, this is FALSE, which means the dataset and its table names are
        /// case-sensitive. This field does not affect routine references.
        #[builder(into, default)]
        pub is_case_insensitive: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The labels associated with this dataset. You can use these to
        /// organize and group your datasets.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The geographic location where the dataset should reside.
        /// See [official docs](https://cloud.google.com/bigquery/docs/dataset-locations).
        /// There are two types of locations, regional or multi-regional. A regional
        /// location is a specific geographic place, such as Tokyo, and a multi-regional
        /// location is a large geographic area, such as the United States, that
        /// contains at least two geographic places.
        /// The default value is multi-regional location `US`.
        /// Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days).
        #[builder(into, default)]
        pub max_time_travel_hours: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The tags attached to this table. Tag keys are globally unique. Tag key is expected to be
        /// in the namespaced format, for example "123456789012/environment" where 123456789012 is the
        /// ID of the parent organization or project resource for this tag key. Tag value is expected
        /// to be the short name, for example "Production". See [Tag definitions](https://www.terraform.io/iam/docs/tags-access-control#definitions)
        /// for more details.
        #[builder(into, default)]
        pub resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the storage billing model for the dataset.
        /// Set this flag value to LOGICAL to use logical bytes for storage billing,
        /// or to PHYSICAL to use physical bytes instead.
        /// LOGICAL is the default if this flag isn't specified.
        #[builder(into, default)]
        pub storage_billing_model: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatasetResult {
        /// An array of objects that define dataset access for one or more entities.
        /// Structure is documented below.
        pub accesses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::bigquery::DatasetAccess>,
        >,
        /// The time when this dataset was created, in milliseconds since the
        /// epoch.
        pub creation_time: pulumi_gestalt_rust::Output<i32>,
        /// A unique ID for this dataset, without the project name. The ID
        /// must contain only letters (a-z, A-Z), numbers (0-9), or
        /// underscores (_). The maximum length is 1,024 characters.
        ///
        ///
        /// - - -
        pub dataset_id: pulumi_gestalt_rust::Output<String>,
        /// Defines the default collation specification of future tables created
        /// in the dataset. If a table is created in this dataset without table-level
        /// default collation, then the table inherits the dataset default collation,
        /// which is applied to the string fields that do not have explicit collation
        /// specified. A change to this field affects only tables created afterwards,
        /// and does not alter the existing tables.
        /// The following values are supported:
        /// - 'und:ci': undetermined locale, case insensitive.
        /// - '': empty string. Default to case-sensitive behavior.
        pub default_collation: pulumi_gestalt_rust::Output<String>,
        /// The default encryption key for all tables in the dataset. Once this property is set,
        /// all newly-created partitioned tables in the dataset will have encryption key set to
        /// this value, unless table creation request (or query) overrides the key.
        /// Structure is documented below.
        pub default_encryption_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::DatasetDefaultEncryptionConfiguration>,
        >,
        /// The default partition expiration for all partitioned tables in
        /// the dataset, in milliseconds.
        /// Once this property is set, all newly-created partitioned tables in
        /// the dataset will have an `expirationMs` property in the `timePartitioning`
        /// settings set to this value, and changing the value will only
        /// affect new tables, not existing ones. The storage in a partition will
        /// have an expiration time of its partition time plus this value.
        /// Setting this property overrides the use of `defaultTableExpirationMs`
        /// for partitioned tables: only one of `defaultTableExpirationMs` and
        /// `defaultPartitionExpirationMs` will be used for any new partitioned
        /// table. If you provide an explicit `timePartitioning.expirationMs` when
        /// creating or updating a partitioned table, that value takes precedence
        /// over the default partition expiration time indicated by this property.
        pub default_partition_expiration_ms: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The default lifetime of all tables in the dataset, in milliseconds.
        /// The minimum value is 3600000 milliseconds (one hour).
        /// Once this property is set, all newly-created tables in the dataset
        /// will have an `expirationTime` property set to the creation time plus
        /// the value in this property, and changing the value will only affect
        /// new tables, not existing ones. When the `expirationTime` for a given
        /// table is reached, that table will be deleted automatically.
        /// If a table's `expirationTime` is modified or removed before the
        /// table expires, or if you provide an explicit `expirationTime` when
        /// creating a table, that value takes precedence over the default
        /// expiration time indicated by this property.
        pub default_table_expiration_ms: pulumi_gestalt_rust::Output<Option<i32>>,
        /// If set to `true`, delete all the tables in the
        /// dataset when destroying the resource; otherwise,
        /// destroying the resource will fail if tables are present.
        pub delete_contents_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A user-friendly description of the dataset
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A hash of the resource.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Options defining open source compatible datasets living in the BigQuery catalog. Contains
        /// metadata of open source database, schema or namespace represented by the current dataset.
        /// Structure is documented below.
        pub external_catalog_dataset_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::DatasetExternalCatalogDatasetOptions>,
        >,
        /// Information about the external metadata storage where the dataset is defined.
        /// Structure is documented below.
        pub external_dataset_reference: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::DatasetExternalDatasetReference>,
        >,
        /// A descriptive name for the dataset
        pub friendly_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// TRUE if the dataset and its table names are case-insensitive, otherwise FALSE.
        /// By default, this is FALSE, which means the dataset and its table names are
        /// case-sensitive. This field does not affect routine references.
        pub is_case_insensitive: pulumi_gestalt_rust::Output<bool>,
        /// The labels associated with this dataset. You can use these to
        /// organize and group your datasets.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The date when this dataset or any of its tables was last modified, in
        /// milliseconds since the epoch.
        pub last_modified_time: pulumi_gestalt_rust::Output<i32>,
        /// The geographic location where the dataset should reside.
        /// See [official docs](https://cloud.google.com/bigquery/docs/dataset-locations).
        /// There are two types of locations, regional or multi-regional. A regional
        /// location is a specific geographic place, such as Tokyo, and a multi-regional
        /// location is a large geographic area, such as the United States, that
        /// contains at least two geographic places.
        /// The default value is multi-regional location `US`.
        /// Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Defines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days).
        pub max_time_travel_hours: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The tags attached to this table. Tag keys are globally unique. Tag key is expected to be
        /// in the namespaced format, for example "123456789012/environment" where 123456789012 is the
        /// ID of the parent organization or project resource for this tag key. Tag value is expected
        /// to be the short name, for example "Production". See [Tag definitions](https://www.terraform.io/iam/docs/tags-access-control#definitions)
        /// for more details.
        pub resource_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Specifies the storage billing model for the dataset.
        /// Set this flag value to LOGICAL to use logical bytes for storage billing,
        /// or to PHYSICAL to use physical bytes instead.
        /// LOGICAL is the default if this flag isn't specified.
        pub storage_billing_model: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DatasetArgs,
    ) -> DatasetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accesses_binding = args.accesses.get_output(context).get_inner();
        let dataset_id_binding = args.dataset_id.get_output(context).get_inner();
        let default_collation_binding = args
            .default_collation
            .get_output(context)
            .get_inner();
        let default_encryption_configuration_binding = args
            .default_encryption_configuration
            .get_output(context)
            .get_inner();
        let default_partition_expiration_ms_binding = args
            .default_partition_expiration_ms
            .get_output(context)
            .get_inner();
        let default_table_expiration_ms_binding = args
            .default_table_expiration_ms
            .get_output(context)
            .get_inner();
        let delete_contents_on_destroy_binding = args
            .delete_contents_on_destroy
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let external_catalog_dataset_options_binding = args
            .external_catalog_dataset_options
            .get_output(context)
            .get_inner();
        let external_dataset_reference_binding = args
            .external_dataset_reference
            .get_output(context)
            .get_inner();
        let friendly_name_binding = args.friendly_name.get_output(context).get_inner();
        let is_case_insensitive_binding = args
            .is_case_insensitive
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let max_time_travel_hours_binding = args
            .max_time_travel_hours
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let resource_tags_binding = args.resource_tags.get_output(context).get_inner();
        let storage_billing_model_binding = args
            .storage_billing_model
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigquery/dataset:Dataset".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accesses".into(),
                    value: &accesses_binding,
                },
                register_interface::ObjectField {
                    name: "datasetId".into(),
                    value: &dataset_id_binding,
                },
                register_interface::ObjectField {
                    name: "defaultCollation".into(),
                    value: &default_collation_binding,
                },
                register_interface::ObjectField {
                    name: "defaultEncryptionConfiguration".into(),
                    value: &default_encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "defaultPartitionExpirationMs".into(),
                    value: &default_partition_expiration_ms_binding,
                },
                register_interface::ObjectField {
                    name: "defaultTableExpirationMs".into(),
                    value: &default_table_expiration_ms_binding,
                },
                register_interface::ObjectField {
                    name: "deleteContentsOnDestroy".into(),
                    value: &delete_contents_on_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "externalCatalogDatasetOptions".into(),
                    value: &external_catalog_dataset_options_binding,
                },
                register_interface::ObjectField {
                    name: "externalDatasetReference".into(),
                    value: &external_dataset_reference_binding,
                },
                register_interface::ObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding,
                },
                register_interface::ObjectField {
                    name: "isCaseInsensitive".into(),
                    value: &is_case_insensitive_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maxTimeTravelHours".into(),
                    value: &max_time_travel_hours_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTags".into(),
                    value: &resource_tags_binding,
                },
                register_interface::ObjectField {
                    name: "storageBillingModel".into(),
                    value: &storage_billing_model_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DatasetResult {
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
            default_encryption_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultEncryptionConfiguration"),
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
            external_dataset_reference: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalDatasetReference"),
            ),
            friendly_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("friendlyName"),
            ),
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
