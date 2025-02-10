/// A Cloud Firestore Database.
///
/// If you wish to use Firestore with App Engine, use the
/// `gcp.appengine.Application`
/// resource instead. If you were previously using the `gcp.appengine.Application` resource exclusively for managing a Firestore database
/// and would like to use the `gcp.firestore.Database` resource instead, please follow the instructions
/// [here](https://cloud.google.com/firestore/docs/app-engine-requirement).
///
///
/// To get more information about Database, see:
///
/// * [API documentation](https://cloud.google.com/firestore/docs/reference/rest/v1/projects.databases)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/firestore/docs/)
///
/// ## Example Usage
///
/// ### Firestore Default Database
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .location_id("nam5")
///             .name("(default)")
///             .project("my-project-name")
///             .type_("FIRESTORE_NATIVE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firestore Database
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .app_engine_integration_mode("DISABLED")
///             .concurrency_mode("OPTIMISTIC")
///             .delete_protection_state("DELETE_PROTECTION_ENABLED")
///             .deletion_policy("DELETE")
///             .location_id("nam5")
///             .name("database-id")
///             .point_in_time_recovery_enablement("POINT_IN_TIME_RECOVERY_ENABLED")
///             .project("my-project-name")
///             .type_("FIRESTORE_NATIVE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firestore Cmek Database
///
///
/// ```yaml
/// resources:
///   database:
///     type: gcp:firestore:Database
///     properties:
///       project: my-project-name
///       name: cmek-database-id
///       locationId: nam5
///       type: FIRESTORE_NATIVE
///       concurrencyMode: OPTIMISTIC
///       appEngineIntegrationMode: DISABLED
///       pointInTimeRecoveryEnablement: POINT_IN_TIME_RECOVERY_ENABLED
///       deleteProtectionState: DELETE_PROTECTION_ENABLED
///       deletionPolicy: DELETE
///       cmekConfig:
///         kmsKeyName: ${cryptoKey.id}
///     options:
///       dependsOn:
///         - ${firestoreCmekKeyuser}
///   cryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: kms-key
///       keyRing: ${keyRing.id}
///       purpose: ENCRYPT_DECRYPT
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: kms-key-ring
///       location: us
///   firestoreCmekKeyuser:
///     type: gcp:kms:CryptoKeyIAMBinding
///     name: firestore_cmek_keyuser
///     properties:
///       cryptoKeyId: ${cryptoKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       members:
///         - serviceAccount:service-${project.number}@gcp-sa-firestore.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Firestore Default Database In Datastore Mode
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let datastoreModeDatabase = database::create(
///         "datastoreModeDatabase",
///         DatabaseArgs::builder()
///             .location_id("nam5")
///             .name("(default)")
///             .project("my-project-name")
///             .type_("DATASTORE_MODE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firestore Database In Datastore Mode
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let datastoreModeDatabase = database::create(
///         "datastoreModeDatabase",
///         DatabaseArgs::builder()
///             .app_engine_integration_mode("DISABLED")
///             .concurrency_mode("OPTIMISTIC")
///             .delete_protection_state("DELETE_PROTECTION_ENABLED")
///             .deletion_policy("DELETE")
///             .location_id("nam5")
///             .name("database-id")
///             .point_in_time_recovery_enablement("POINT_IN_TIME_RECOVERY_ENABLED")
///             .project("my-project-name")
///             .type_("DATASTORE_MODE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Firestore Cmek Database In Datastore Mode
///
///
/// ```yaml
/// resources:
///   database:
///     type: gcp:firestore:Database
///     properties:
///       project: my-project-name
///       name: cmek-database-id
///       locationId: nam5
///       type: DATASTORE_MODE
///       concurrencyMode: OPTIMISTIC
///       appEngineIntegrationMode: DISABLED
///       pointInTimeRecoveryEnablement: POINT_IN_TIME_RECOVERY_ENABLED
///       deleteProtectionState: DELETE_PROTECTION_ENABLED
///       deletionPolicy: DELETE
///       cmekConfig:
///         kmsKeyName: ${cryptoKey.id}
///     options:
///       dependsOn:
///         - ${firestoreCmekKeyuser}
///   cryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: kms-key
///       keyRing: ${keyRing.id}
///       purpose: ENCRYPT_DECRYPT
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: kms-key-ring
///       location: us
///   firestoreCmekKeyuser:
///     type: gcp:kms:CryptoKeyIAMBinding
///     name: firestore_cmek_keyuser
///     properties:
///       cryptoKeyId: ${cryptoKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       members:
///         - serviceAccount:service-${project.number}@gcp-sa-firestore.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Database can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/databases/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Database can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:firestore/database:Database default projects/{{project}}/databases/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firestore/database:Database default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:firestore/database:Database default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// The App Engine integration mode to use for this database.
        /// Possible values are: `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub app_engine_integration_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The CMEK (Customer Managed Encryption Key) configuration for a Firestore
        /// database. If not present, the database is secured by the default Google
        /// encryption key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cmek_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::firestore::DatabaseCmekConfig>,
        >,
        /// The concurrency control mode to use for this database.
        /// Possible values are: `OPTIMISTIC`, `PESSIMISTIC`, `OPTIMISTIC_WITH_ENTITY_GROUPS`.
        #[builder(into, default)]
        pub concurrency_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub delete_protection_state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the database. Available locations are listed at
        /// https://cloud.google.com/firestore/docs/locations.
        #[builder(into)]
        pub location_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID to use for the database, which will become the final
        /// component of the database's resource name. This value should be 4-63
        /// characters. Valid characters are /[a-z][0-9]-/ with first character
        /// a letter and the last a letter or a number. Must not be
        /// UUID-like /[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}/.
        /// "(default)" database id is also valid.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to enable the PITR feature on this database.
        /// If `POINT_IN_TIME_RECOVERY_ENABLED` is selected, reads are supported on selected versions of the data from within the past 7 days.
        /// versionRetentionPeriod and earliestVersionTime can be used to determine the supported versions. These include reads against any timestamp within the past hour
        /// and reads against 1-minute snapshots beyond 1 hour and within 7 days.
        /// If `POINT_IN_TIME_RECOVERY_DISABLED` is selected, reads are supported on any version of the data from within the past 1 hour.
        /// Default value is `POINT_IN_TIME_RECOVERY_DISABLED`.
        /// Possible values are: `POINT_IN_TIME_RECOVERY_ENABLED`, `POINT_IN_TIME_RECOVERY_DISABLED`.
        #[builder(into, default)]
        pub point_in_time_recovery_enablement: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the database.
        /// See https://cloud.google.com/datastore/docs/firestore-or-datastore
        /// for information about how to choose.
        /// Possible values are: `FIRESTORE_NATIVE`, `DATASTORE_MODE`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// The App Engine integration mode to use for this database.
        /// Possible values are: `ENABLED`, `DISABLED`.
        pub app_engine_integration_mode: pulumi_gestalt_rust::Output<String>,
        /// The CMEK (Customer Managed Encryption Key) configuration for a Firestore
        /// database. If not present, the database is secured by the default Google
        /// encryption key.
        /// Structure is documented below.
        pub cmek_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::firestore::DatabaseCmekConfig>,
        >,
        /// The concurrency control mode to use for this database.
        /// Possible values are: `OPTIMISTIC`, `PESSIMISTIC`, `OPTIMISTIC_WITH_ENTITY_GROUPS`.
        pub concurrency_mode: pulumi_gestalt_rust::Output<String>,
        /// Output only. The timestamp at which this database was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub delete_protection_state: pulumi_gestalt_rust::Output<String>,
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. The earliest timestamp at which older versions of the data can be read from the database. See versionRetentionPeriod above; this field is populated with now - versionRetentionPeriod.
        /// This value is continuously updated, and becomes stale the moment it is queried. If you are using this value to recover data, make sure to account for the time from the moment when the value is queried to the moment when you initiate the recovery.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub earliest_version_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. This checksum is computed by the server based on the value of other fields,
        /// and may be sent on update and delete requests to ensure the client has an
        /// up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Output only. The keyPrefix for this database.
        /// This keyPrefix is used, in combination with the project id ("~") to construct the application id
        /// that is returned from the Cloud Datastore APIs in Google App Engine first generation runtimes.
        /// This value may be empty in which case the appid to use for URL-encoded keys is the project_id (eg: foo instead of v~foo).
        pub key_prefix: pulumi_gestalt_rust::Output<String>,
        /// The location of the database. Available locations are listed at
        /// https://cloud.google.com/firestore/docs/locations.
        pub location_id: pulumi_gestalt_rust::Output<String>,
        /// The ID to use for the database, which will become the final
        /// component of the database's resource name. This value should be 4-63
        /// characters. Valid characters are /[a-z][0-9]-/ with first character
        /// a letter and the last a letter or a number. Must not be
        /// UUID-like /[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}/.
        /// "(default)" database id is also valid.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable the PITR feature on this database.
        /// If `POINT_IN_TIME_RECOVERY_ENABLED` is selected, reads are supported on selected versions of the data from within the past 7 days.
        /// versionRetentionPeriod and earliestVersionTime can be used to determine the supported versions. These include reads against any timestamp within the past hour
        /// and reads against 1-minute snapshots beyond 1 hour and within 7 days.
        /// If `POINT_IN_TIME_RECOVERY_DISABLED` is selected, reads are supported on any version of the data from within the past 1 hour.
        /// Default value is `POINT_IN_TIME_RECOVERY_DISABLED`.
        /// Possible values are: `POINT_IN_TIME_RECOVERY_ENABLED`, `POINT_IN_TIME_RECOVERY_DISABLED`.
        pub point_in_time_recovery_enablement: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The type of the database.
        /// See https://cloud.google.com/datastore/docs/firestore-or-datastore
        /// for information about how to choose.
        /// Possible values are: `FIRESTORE_NATIVE`, `DATASTORE_MODE`.
        ///
        ///
        /// - - -
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Output only. The system-generated UUID4 for this Database.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. The timestamp at which this database was most recently updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. The period during which past versions of data are retained in the database.
        /// Any read or query can specify a readTime within this window, and will read the state of the database at that time.
        /// If the PITR feature is enabled, the retention period is 7 days. Otherwise, the retention period is 1 hour.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        pub version_retention_period: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DatabaseArgs,
    ) -> DatabaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_engine_integration_mode_binding = args
            .app_engine_integration_mode
            .get_output(context);
        let cmek_config_binding = args.cmek_config.get_output(context);
        let concurrency_mode_binding = args.concurrency_mode.get_output(context);
        let delete_protection_state_binding = args
            .delete_protection_state
            .get_output(context);
        let deletion_policy_binding = args.deletion_policy.get_output(context);
        let location_id_binding = args.location_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let point_in_time_recovery_enablement_binding = args
            .point_in_time_recovery_enablement
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:firestore/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appEngineIntegrationMode".into(),
                    value: app_engine_integration_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cmekConfig".into(),
                    value: cmek_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "concurrencyMode".into(),
                    value: concurrency_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteProtectionState".into(),
                    value: delete_protection_state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionPolicy".into(),
                    value: deletion_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locationId".into(),
                    value: location_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pointInTimeRecoveryEnablement".into(),
                    value: point_in_time_recovery_enablement_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatabaseResult {
            app_engine_integration_mode: o.get_field("appEngineIntegrationMode"),
            cmek_config: o.get_field("cmekConfig"),
            concurrency_mode: o.get_field("concurrencyMode"),
            create_time: o.get_field("createTime"),
            delete_protection_state: o.get_field("deleteProtectionState"),
            deletion_policy: o.get_field("deletionPolicy"),
            earliest_version_time: o.get_field("earliestVersionTime"),
            etag: o.get_field("etag"),
            key_prefix: o.get_field("keyPrefix"),
            location_id: o.get_field("locationId"),
            name: o.get_field("name"),
            point_in_time_recovery_enablement: o
                .get_field("pointInTimeRecoveryEnablement"),
            project: o.get_field("project"),
            type_: o.get_field("type"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            version_retention_period: o.get_field("versionRetentionPeriod"),
        }
    }
}
