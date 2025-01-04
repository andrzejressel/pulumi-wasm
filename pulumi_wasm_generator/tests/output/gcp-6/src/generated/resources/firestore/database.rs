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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// The App Engine integration mode to use for this database.
        /// Possible values are: `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub app_engine_integration_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The CMEK (Customer Managed Encryption Key) configuration for a Firestore
        /// database. If not present, the database is secured by the default Google
        /// encryption key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cmek_config: pulumi_wasm_rust::Output<
            Option<super::super::types::firestore::DatabaseCmekConfig>,
        >,
        /// The concurrency control mode to use for this database.
        /// Possible values are: `OPTIMISTIC`, `PESSIMISTIC`, `OPTIMISTIC_WITH_ENTITY_GROUPS`.
        #[builder(into, default)]
        pub concurrency_mode: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub delete_protection_state: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the database. Available locations are listed at
        /// https://cloud.google.com/firestore/docs/locations.
        #[builder(into)]
        pub location_id: pulumi_wasm_rust::Output<String>,
        /// The ID to use for the database, which will become the final
        /// component of the database's resource name. This value should be 4-63
        /// characters. Valid characters are /[a-z][0-9]-/ with first character
        /// a letter and the last a letter or a number. Must not be
        /// UUID-like /[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}/.
        /// "(default)" database id is also valid.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to enable the PITR feature on this database.
        /// If `POINT_IN_TIME_RECOVERY_ENABLED` is selected, reads are supported on selected versions of the data from within the past 7 days.
        /// versionRetentionPeriod and earliestVersionTime can be used to determine the supported versions. These include reads against any timestamp within the past hour
        /// and reads against 1-minute snapshots beyond 1 hour and within 7 days.
        /// If `POINT_IN_TIME_RECOVERY_DISABLED` is selected, reads are supported on any version of the data from within the past 1 hour.
        /// Default value is `POINT_IN_TIME_RECOVERY_DISABLED`.
        /// Possible values are: `POINT_IN_TIME_RECOVERY_ENABLED`, `POINT_IN_TIME_RECOVERY_DISABLED`.
        #[builder(into, default)]
        pub point_in_time_recovery_enablement: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of the database.
        /// See https://cloud.google.com/datastore/docs/firestore-or-datastore
        /// for information about how to choose.
        /// Possible values are: `FIRESTORE_NATIVE`, `DATASTORE_MODE`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// The App Engine integration mode to use for this database.
        /// Possible values are: `ENABLED`, `DISABLED`.
        pub app_engine_integration_mode: pulumi_wasm_rust::Output<String>,
        /// The CMEK (Customer Managed Encryption Key) configuration for a Firestore
        /// database. If not present, the database is secured by the default Google
        /// encryption key.
        /// Structure is documented below.
        pub cmek_config: pulumi_wasm_rust::Output<
            Option<super::super::types::firestore::DatabaseCmekConfig>,
        >,
        /// The concurrency control mode to use for this database.
        /// Possible values are: `OPTIMISTIC`, `PESSIMISTIC`, `OPTIMISTIC_WITH_ENTITY_GROUPS`.
        pub concurrency_mode: pulumi_wasm_rust::Output<String>,
        /// Output only. The timestamp at which this database was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub delete_protection_state: pulumi_wasm_rust::Output<String>,
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Output only. The earliest timestamp at which older versions of the data can be read from the database. See versionRetentionPeriod above; this field is populated with now - versionRetentionPeriod.
        /// This value is continuously updated, and becomes stale the moment it is queried. If you are using this value to recover data, make sure to account for the time from the moment when the value is queried to the moment when you initiate the recovery.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub earliest_version_time: pulumi_wasm_rust::Output<String>,
        /// Output only. This checksum is computed by the server based on the value of other fields,
        /// and may be sent on update and delete requests to ensure the client has an
        /// up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Output only. The keyPrefix for this database.
        /// This keyPrefix is used, in combination with the project id ("~") to construct the application id
        /// that is returned from the Cloud Datastore APIs in Google App Engine first generation runtimes.
        /// This value may be empty in which case the appid to use for URL-encoded keys is the project_id (eg: foo instead of v~foo).
        pub key_prefix: pulumi_wasm_rust::Output<String>,
        /// The location of the database. Available locations are listed at
        /// https://cloud.google.com/firestore/docs/locations.
        pub location_id: pulumi_wasm_rust::Output<String>,
        /// The ID to use for the database, which will become the final
        /// component of the database's resource name. This value should be 4-63
        /// characters. Valid characters are /[a-z][0-9]-/ with first character
        /// a letter and the last a letter or a number. Must not be
        /// UUID-like /[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}/.
        /// "(default)" database id is also valid.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether to enable the PITR feature on this database.
        /// If `POINT_IN_TIME_RECOVERY_ENABLED` is selected, reads are supported on selected versions of the data from within the past 7 days.
        /// versionRetentionPeriod and earliestVersionTime can be used to determine the supported versions. These include reads against any timestamp within the past hour
        /// and reads against 1-minute snapshots beyond 1 hour and within 7 days.
        /// If `POINT_IN_TIME_RECOVERY_DISABLED` is selected, reads are supported on any version of the data from within the past 1 hour.
        /// Default value is `POINT_IN_TIME_RECOVERY_DISABLED`.
        /// Possible values are: `POINT_IN_TIME_RECOVERY_ENABLED`, `POINT_IN_TIME_RECOVERY_DISABLED`.
        pub point_in_time_recovery_enablement: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The type of the database.
        /// See https://cloud.google.com/datastore/docs/firestore-or-datastore
        /// for information about how to choose.
        /// Possible values are: `FIRESTORE_NATIVE`, `DATASTORE_MODE`.
        ///
        ///
        /// - - -
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Output only. The system-generated UUID4 for this Database.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. The timestamp at which this database was most recently updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Output only. The period during which past versions of data are retained in the database.
        /// Any read or query can specify a readTime within this window, and will read the state of the database at that time.
        /// If the PITR feature is enabled, the retention period is 7 days. Otherwise, the retention period is 1 hour.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        pub version_retention_period: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DatabaseArgs) -> DatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_engine_integration_mode_binding = args
            .app_engine_integration_mode
            .get_inner();
        let cmek_config_binding = args.cmek_config.get_inner();
        let concurrency_mode_binding = args.concurrency_mode.get_inner();
        let delete_protection_state_binding = args.delete_protection_state.get_inner();
        let deletion_policy_binding = args.deletion_policy.get_inner();
        let location_id_binding = args.location_id.get_inner();
        let name_binding = args.name.get_inner();
        let point_in_time_recovery_enablement_binding = args
            .point_in_time_recovery_enablement
            .get_inner();
        let project_binding = args.project.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:firestore/database:Database".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appEngineIntegrationMode".into(),
                    value: &app_engine_integration_mode_binding,
                },
                register_interface::ObjectField {
                    name: "cmekConfig".into(),
                    value: &cmek_config_binding,
                },
                register_interface::ObjectField {
                    name: "concurrencyMode".into(),
                    value: &concurrency_mode_binding,
                },
                register_interface::ObjectField {
                    name: "deleteProtectionState".into(),
                    value: &delete_protection_state_binding,
                },
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "locationId".into(),
                    value: &location_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pointInTimeRecoveryEnablement".into(),
                    value: &point_in_time_recovery_enablement_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "appEngineIntegrationMode".into(),
                },
                register_interface::ResultField {
                    name: "cmekConfig".into(),
                },
                register_interface::ResultField {
                    name: "concurrencyMode".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deleteProtectionState".into(),
                },
                register_interface::ResultField {
                    name: "deletionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "earliestVersionTime".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "keyPrefix".into(),
                },
                register_interface::ResultField {
                    name: "locationId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pointInTimeRecoveryEnablement".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "versionRetentionPeriod".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatabaseResult {
            app_engine_integration_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appEngineIntegrationMode").unwrap(),
            ),
            cmek_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cmekConfig").unwrap(),
            ),
            concurrency_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("concurrencyMode").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            delete_protection_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteProtectionState").unwrap(),
            ),
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionPolicy").unwrap(),
            ),
            earliest_version_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("earliestVersionTime").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            key_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyPrefix").unwrap(),
            ),
            location_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locationId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            point_in_time_recovery_enablement: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pointInTimeRecoveryEnablement").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            version_retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionRetentionPeriod").unwrap(),
            ),
        }
    }
}
