/// Creates a new bucket in Google cloud storage service (GCS).
/// Once a bucket has been created, its location can't be changed.
///
/// For more information see
/// [the official documentation](https://cloud.google.com/storage/docs/overview)
/// and
/// [API](https://cloud.google.com/storage/docs/json_api/v1/buckets).
///
/// **Note**: If the project id is not set on the resource or in the provider block it will be dynamically
/// determined which will require enabling the compute api.
///
///
/// ## Example Usage
///
/// ### Creating A Private Bucket In Standard Storage, In The EU Region. Bucket Configured As Static Website And CORS Configurations
///
/// ```yaml
/// resources:
///   static-site:
///     type: gcp:storage:Bucket
///     properties:
///       name: image-store.com
///       location: EU
///       forceDestroy: true
///       uniformBucketLevelAccess: true
///       website:
///         mainPageSuffix: index.html
///         notFoundPage: 404.html
///       cors:
///         - origins:
///             - http://image-store.com
///           methods:
///             - GET
///             - HEAD
///             - PUT
///             - POST
///             - DELETE
///           responseHeaders:
///             - '*'
///           maxAgeSeconds: 3600
/// ```
///
/// ### Life Cycle Settings For Storage Bucket Objects
///
/// ```yaml
/// resources:
///   auto-expire:
///     type: gcp:storage:Bucket
///     properties:
///       name: auto-expiring-bucket
///       location: US
///       forceDestroy: true
///       lifecycleRules:
///         - condition:
///             age: 3
///           action:
///             type: Delete
///         - condition:
///             age: 1
///           action:
///             type: AbortIncompleteMultipartUpload
/// ```
///
/// ### Life Cycle Settings For Storage Bucket Objects With `Send_age_if_zero` Disabled
/// When creating a life cycle condition that does not also include an `age` field, a default `age` of 0 will be set. Set the `send_age_if_zero` flag to `false` to prevent this and avoid any potentially unintended interactions.
///
/// ```yaml
/// resources:
///   no-age-enabled:
///     type: gcp:storage:Bucket
///     properties:
///       name: no-age-enabled-bucket
///       location: US
///       forceDestroy: true
///       lifecycleRules:
///         - action:
///             type: Delete
///           condition:
///             daysSinceNoncurrentTime: 3
///             sendAgeIfZero: false
/// ```
///
/// ### Enabling Public Access Prevention
///
/// ```yaml
/// resources:
///   auto-expire:
///     type: gcp:storage:Bucket
///     properties:
///       name: no-public-access-bucket
///       location: US
///       forceDestroy: true
///       publicAccessPrevention: enforced
/// ```
///
/// ## Import
///
/// Storage buckets can be imported using the `name` or  `project/name`. If the project is not
///
/// passed to the import command it will be inferred from the provider block or environment variables.
///
/// If it cannot be inferred it will be queried from the Compute API (this will fail if the API is
///
/// not enabled).
///
/// * `{{project_id}}/{{bucket}}`
///
/// * `{{bucket}}`
///
/// When using the `pulumi import` command, Storage buckets can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/bucket:Bucket default {{bucket}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:storage/bucket:Bucket default {{project_id}}/{{bucket}}
/// ```
///
/// `false` in state. If you've set it to `true` in config, run `pulumi up` to
///
/// update the value set in state. If you delete this resource before updating the
///
/// value, objects in the bucket will not be destroyed.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketArgs {
        /// The bucket's [Autoclass](https://cloud.google.com/storage/docs/autoclass) configuration.  Structure is documented below.
        #[builder(into, default)]
        pub autoclass: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketAutoclass>,
        >,
        /// The bucket's [Cross-Origin Resource Sharing (CORS)](https://www.w3.org/TR/cors/) configuration. Multiple blocks of this type are permitted. Structure is documented below.
        #[builder(into, default)]
        pub cors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::storage::BucketCor>>,
        >,
        /// The bucket's custom location configuration, which specifies the individual regions that comprise a dual-region bucket. If the bucket is designated a single or multi-region, the parameters are empty. Structure is documented below.
        #[builder(into, default)]
        pub custom_placement_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketCustomPlacementConfig>,
        >,
        /// Whether or not to automatically apply an eventBasedHold to new objects added to the bucket.
        #[builder(into, default)]
        pub default_event_based_hold: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Enables [object retention](https://cloud.google.com/storage/docs/object-lock) on a storage bucket.
        #[builder(into, default)]
        pub enable_object_retention: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The bucket's encryption configuration. Structure is documented below.
        #[builder(into, default)]
        pub encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketEncryption>,
        >,
        /// When deleting a bucket, this
        /// boolean option will delete all contained objects. If you try to delete a
        /// bucket that contains objects, the provider will fail that run.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The bucket's hierarchical namespace policy, which defines the bucket capability to handle folders in logical structure. Structure is documented below. To use this configuration, `uniform_bucket_level_access` must be enabled on bucket.
        #[builder(into, default)]
        pub hierarchical_namespace: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketHierarchicalNamespace>,
        >,
        /// A map of key/value label pairs to assign to the bucket.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The bucket's [Lifecycle Rules](https://cloud.google.com/storage/docs/lifecycle#configuration) configuration. Multiple blocks of this type are permitted. Structure is documented below.
        #[builder(into, default)]
        pub lifecycle_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::storage::BucketLifecycleRule>>,
        >,
        /// The [GCS location](https://cloud.google.com/storage/docs/bucket-locations).
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The bucket's [Access & Storage Logs](https://cloud.google.com/storage/docs/access-logs) configuration. Structure is documented below.
        #[builder(into, default)]
        pub logging: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketLogging>,
        >,
        /// The name of the bucket.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Prevents public access to a bucket. Acceptable values are "inherited" or "enforced". If "inherited", the bucket uses [public access prevention](https://cloud.google.com/storage/docs/public-access-prevention). only if the bucket is subject to the public access prevention organization policy constraint. Defaults to "inherited".
        #[builder(into, default)]
        pub public_access_prevention: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables [Requester Pays](https://cloud.google.com/storage/docs/requester-pays) on a storage bucket.
        #[builder(into, default)]
        pub requester_pays: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration of the bucket's data retention policy for how long objects in the bucket should be retained. Structure is documented below.
        #[builder(into, default)]
        pub retention_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketRetentionPolicy>,
        >,
        /// The recovery point objective for cross-region replication of the bucket. Applicable only for dual and multi-region buckets. `"DEFAULT"` sets default replication. `"ASYNC_TURBO"` value enables turbo replication, valid for dual-region buckets only. See [Turbo Replication](https://cloud.google.com/storage/docs/managing-turbo-replication) for more information. If rpo is not specified at bucket creation, it defaults to `"DEFAULT"` for dual and multi-region buckets. **NOTE** If used with single-region bucket, It will throw an error.
        #[builder(into, default)]
        pub rpo: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The bucket's soft delete policy, which defines the period of time that soft-deleted objects will be retained, and cannot
        /// be permanently deleted. If it is not provided, by default Google Cloud Storage sets this to default soft delete policy
        #[builder(into, default)]
        pub soft_delete_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketSoftDeletePolicy>,
        >,
        /// The [Storage Class](https://cloud.google.com/storage/docs/storage-classes) of the new bucket. Supported values include: `STANDARD`, `MULTI_REGIONAL`, `REGIONAL`, `NEARLINE`, `COLDLINE`, `ARCHIVE`.
        #[builder(into, default)]
        pub storage_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables [Uniform bucket-level access](https://cloud.google.com/storage/docs/uniform-bucket-level-access) access to a bucket.
        #[builder(into, default)]
        pub uniform_bucket_level_access: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The bucket's [Versioning](https://cloud.google.com/storage/docs/object-versioning) configuration.  Structure is documented below.
        #[builder(into, default)]
        pub versioning: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketVersioning>,
        >,
        /// Configuration if the bucket acts as a website. Structure is documented below.
        #[builder(into, default)]
        pub website: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::BucketWebsite>,
        >,
    }
    #[allow(dead_code)]
    pub struct BucketResult {
        /// The bucket's [Autoclass](https://cloud.google.com/storage/docs/autoclass) configuration.  Structure is documented below.
        pub autoclass: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::BucketAutoclass>,
        >,
        /// The bucket's [Cross-Origin Resource Sharing (CORS)](https://www.w3.org/TR/cors/) configuration. Multiple blocks of this type are permitted. Structure is documented below.
        pub cors: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::storage::BucketCor>>,
        >,
        /// The bucket's custom location configuration, which specifies the individual regions that comprise a dual-region bucket. If the bucket is designated a single or multi-region, the parameters are empty. Structure is documented below.
        pub custom_placement_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::BucketCustomPlacementConfig>,
        >,
        /// Whether or not to automatically apply an eventBasedHold to new objects added to the bucket.
        pub default_event_based_hold: pulumi_gestalt_rust::Output<Option<bool>>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enables [object retention](https://cloud.google.com/storage/docs/object-lock) on a storage bucket.
        pub enable_object_retention: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The bucket's encryption configuration. Structure is documented below.
        pub encryption: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::BucketEncryption>,
        >,
        /// When deleting a bucket, this
        /// boolean option will delete all contained objects. If you try to delete a
        /// bucket that contains objects, the provider will fail that run.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The bucket's hierarchical namespace policy, which defines the bucket capability to handle folders in logical structure. Structure is documented below. To use this configuration, `uniform_bucket_level_access` must be enabled on bucket.
        pub hierarchical_namespace: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::BucketHierarchicalNamespace>,
        >,
        /// A map of key/value label pairs to assign to the bucket.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The bucket's [Lifecycle Rules](https://cloud.google.com/storage/docs/lifecycle#configuration) configuration. Multiple blocks of this type are permitted. Structure is documented below.
        pub lifecycle_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::storage::BucketLifecycleRule>>,
        >,
        /// The [GCS location](https://cloud.google.com/storage/docs/bucket-locations).
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The bucket's [Access & Storage Logs](https://cloud.google.com/storage/docs/access-logs) configuration. Structure is documented below.
        pub logging: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::BucketLogging>,
        >,
        /// The name of the bucket.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The project number of the project in which the resource belongs.
        pub project_number: pulumi_gestalt_rust::Output<i32>,
        /// Prevents public access to a bucket. Acceptable values are "inherited" or "enforced". If "inherited", the bucket uses [public access prevention](https://cloud.google.com/storage/docs/public-access-prevention). only if the bucket is subject to the public access prevention organization policy constraint. Defaults to "inherited".
        pub public_access_prevention: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enables [Requester Pays](https://cloud.google.com/storage/docs/requester-pays) on a storage bucket.
        pub requester_pays: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration of the bucket's data retention policy for how long objects in the bucket should be retained. Structure is documented below.
        pub retention_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::BucketRetentionPolicy>,
        >,
        /// The recovery point objective for cross-region replication of the bucket. Applicable only for dual and multi-region buckets. `"DEFAULT"` sets default replication. `"ASYNC_TURBO"` value enables turbo replication, valid for dual-region buckets only. See [Turbo Replication](https://cloud.google.com/storage/docs/managing-turbo-replication) for more information. If rpo is not specified at bucket creation, it defaults to `"DEFAULT"` for dual and multi-region buckets. **NOTE** If used with single-region bucket, It will throw an error.
        pub rpo: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The bucket's soft delete policy, which defines the period of time that soft-deleted objects will be retained, and cannot
        /// be permanently deleted. If it is not provided, by default Google Cloud Storage sets this to default soft delete policy
        pub soft_delete_policy: pulumi_gestalt_rust::Output<
            super::super::types::storage::BucketSoftDeletePolicy,
        >,
        /// The [Storage Class](https://cloud.google.com/storage/docs/storage-classes) of the new bucket. Supported values include: `STANDARD`, `MULTI_REGIONAL`, `REGIONAL`, `NEARLINE`, `COLDLINE`, `ARCHIVE`.
        pub storage_class: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enables [Uniform bucket-level access](https://cloud.google.com/storage/docs/uniform-bucket-level-access) access to a bucket.
        pub uniform_bucket_level_access: pulumi_gestalt_rust::Output<bool>,
        /// The base URL of the bucket, in the format `gs://<bucket-name>`.
        pub url: pulumi_gestalt_rust::Output<String>,
        /// The bucket's [Versioning](https://cloud.google.com/storage/docs/object-versioning) configuration.  Structure is documented below.
        pub versioning: pulumi_gestalt_rust::Output<
            super::super::types::storage::BucketVersioning,
        >,
        /// Configuration if the bucket acts as a website. Structure is documented below.
        pub website: pulumi_gestalt_rust::Output<
            super::super::types::storage::BucketWebsite,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketArgs,
    ) -> BucketResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoclass_binding = args.autoclass.get_output(context);
        let cors_binding = args.cors.get_output(context);
        let custom_placement_config_binding = args
            .custom_placement_config
            .get_output(context);
        let default_event_based_hold_binding = args
            .default_event_based_hold
            .get_output(context);
        let enable_object_retention_binding = args
            .enable_object_retention
            .get_output(context);
        let encryption_binding = args.encryption.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let hierarchical_namespace_binding = args
            .hierarchical_namespace
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let lifecycle_rules_binding = args.lifecycle_rules.get_output(context);
        let location_binding = args.location.get_output(context);
        let logging_binding = args.logging.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let public_access_prevention_binding = args
            .public_access_prevention
            .get_output(context);
        let requester_pays_binding = args.requester_pays.get_output(context);
        let retention_policy_binding = args.retention_policy.get_output(context);
        let rpo_binding = args.rpo.get_output(context);
        let soft_delete_policy_binding = args.soft_delete_policy.get_output(context);
        let storage_class_binding = args.storage_class.get_output(context);
        let uniform_bucket_level_access_binding = args
            .uniform_bucket_level_access
            .get_output(context);
        let versioning_binding = args.versioning.get_output(context);
        let website_binding = args.website.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:storage/bucket:Bucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoclass".into(),
                    value: autoclass_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cors".into(),
                    value: cors_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customPlacementConfig".into(),
                    value: custom_placement_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultEventBasedHold".into(),
                    value: default_event_based_hold_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableObjectRetention".into(),
                    value: enable_object_retention_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryption".into(),
                    value: encryption_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hierarchicalNamespace".into(),
                    value: hierarchical_namespace_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lifecycleRules".into(),
                    value: lifecycle_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logging".into(),
                    value: logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicAccessPrevention".into(),
                    value: public_access_prevention_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requesterPays".into(),
                    value: requester_pays_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionPolicy".into(),
                    value: retention_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rpo".into(),
                    value: rpo_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "softDeletePolicy".into(),
                    value: soft_delete_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageClass".into(),
                    value: storage_class_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uniformBucketLevelAccess".into(),
                    value: uniform_bucket_level_access_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versioning".into(),
                    value: versioning_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "website".into(),
                    value: website_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketResult {
            autoclass: o.get_field("autoclass"),
            cors: o.get_field("cors"),
            custom_placement_config: o.get_field("customPlacementConfig"),
            default_event_based_hold: o.get_field("defaultEventBasedHold"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_object_retention: o.get_field("enableObjectRetention"),
            encryption: o.get_field("encryption"),
            force_destroy: o.get_field("forceDestroy"),
            hierarchical_namespace: o.get_field("hierarchicalNamespace"),
            labels: o.get_field("labels"),
            lifecycle_rules: o.get_field("lifecycleRules"),
            location: o.get_field("location"),
            logging: o.get_field("logging"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            project_number: o.get_field("projectNumber"),
            public_access_prevention: o.get_field("publicAccessPrevention"),
            pulumi_labels: o.get_field("pulumiLabels"),
            requester_pays: o.get_field("requesterPays"),
            retention_policy: o.get_field("retentionPolicy"),
            rpo: o.get_field("rpo"),
            self_link: o.get_field("selfLink"),
            soft_delete_policy: o.get_field("softDeletePolicy"),
            storage_class: o.get_field("storageClass"),
            uniform_bucket_level_access: o.get_field("uniformBucketLevelAccess"),
            url: o.get_field("url"),
            versioning: o.get_field("versioning"),
            website: o.get_field("website"),
        }
    }
}
