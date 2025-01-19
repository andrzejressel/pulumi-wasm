/// A managed metastore service that serves metadata queries.
///
///
/// To get more information about Service, see:
///
/// * [API documentation](https://cloud.google.com/dataproc-metastore/docs/reference/rest/v1/projects.locations.services)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dataproc-metastore/docs/overview)
///
/// ## Example Usage
///
/// ### Dataproc Metastore Service Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:dataproc:MetastoreService
///     properties:
///       serviceId: metastore-srv
///       location: us-central1
///       port: 9080
///       tier: DEVELOPER
///       maintenanceWindow:
///         hourOfDay: 2
///         dayOfWeek: SUNDAY
///       hiveMetastoreConfig:
///         version: 2.3.6
///       labels:
///         env: test
/// ```
/// ### Dataproc Metastore Service Deletion Protection
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:dataproc:MetastoreService
///     properties:
///       serviceId: metastore-srv
///       location: us-central1
///       port: 9080
///       tier: DEVELOPER
///       deletionProtection: true
///       maintenanceWindow:
///         hourOfDay: 2
///         dayOfWeek: SUNDAY
///       hiveMetastoreConfig:
///         version: 2.3.6
///       labels:
///         env: test
/// ```
/// ### Dataproc Metastore Service Cmek Example
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cryptoKey = crypto_key::create(
///         "cryptoKey",
///         CryptoKeyArgs::builder()
///             .key_ring("${keyRing.id}")
///             .name("example-key")
///             .purpose("ENCRYPT_DECRYPT")
///             .build_struct(),
///     );
///     let default = metastore_service::create(
///         "default",
///         MetastoreServiceArgs::builder()
///             .encryption_config(
///                 MetastoreServiceEncryptionConfig::builder()
///                     .kmsKey("${cryptoKey.id}")
///                     .build_struct(),
///             )
///             .hive_metastore_config(
///                 MetastoreServiceHiveMetastoreConfig::builder()
///                     .version("3.1.2")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .service_id("example-service")
///             .build_struct(),
///     );
///     let keyRing = key_ring::create(
///         "keyRing",
///         KeyRingArgs::builder()
///             .location("us-central1")
///             .name("example-keyring")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataproc Metastore Service Private Service Connect
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = metastore_service::create(
///         "default",
///         MetastoreServiceArgs::builder()
///             .hive_metastore_config(
///                 MetastoreServiceHiveMetastoreConfig::builder()
///                     .version("3.1.2")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .network_config(
///                 MetastoreServiceNetworkConfig::builder()
///                     .consumers(
///                         vec![
///                             MetastoreServiceNetworkConfigConsumer::builder()
///                             .subnetwork("${subnet.id}").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .service_id("metastore-srv")
///             .tier("DEVELOPER")
///             .build_struct(),
///     );
///     let net = network::create(
///         "net",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .build_struct(),
///     );
///     let subnet = subnetwork::create(
///         "subnet",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/22")
///             .name("my-subnetwork")
///             .network("${net.id}")
///             .private_ip_google_access(true)
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataproc Metastore Service Private Service Connect Custom Routes
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = metastore_service::create(
///         "default",
///         MetastoreServiceArgs::builder()
///             .hive_metastore_config(
///                 MetastoreServiceHiveMetastoreConfig::builder()
///                     .version("3.1.2")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .network_config(
///                 MetastoreServiceNetworkConfig::builder()
///                     .consumers(
///                         vec![
///                             MetastoreServiceNetworkConfigConsumer::builder()
///                             .subnetwork("${subnet.id}").build_struct(),
///                         ],
///                     )
///                     .customRoutesEnabled(true)
///                     .build_struct(),
///             )
///             .service_id("metastore-srv")
///             .build_struct(),
///     );
///     let net = network::create(
///         "net",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .build_struct(),
///     );
///     let subnet = subnetwork::create(
///         "subnet",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/22")
///             .name("my-subnetwork")
///             .network("${net.id}")
///             .private_ip_google_access(true)
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataproc Metastore Service Dpms2
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dpms2 = metastore_service::create(
///         "dpms2",
///         MetastoreServiceArgs::builder()
///             .database_type("SPANNER")
///             .hive_metastore_config(
///                 MetastoreServiceHiveMetastoreConfig::builder()
///                     .version("3.1.2")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .scaling_config(
///                 MetastoreServiceScalingConfig::builder()
///                     .instanceSize("EXTRA_SMALL")
///                     .build_struct(),
///             )
///             .service_id("ms-dpms2")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataproc Metastore Service Dpms2 Scaling Factor
///
///
/// ```yaml
/// resources:
///   dpms2ScalingFactor:
///     type: gcp:dataproc:MetastoreService
///     name: dpms2_scaling_factor
///     properties:
///       serviceId: ms-dpms2sf
///       location: us-central1
///       databaseType: SPANNER
///       hiveMetastoreConfig:
///         version: 3.1.2
///       scalingConfig:
///         scalingFactor: '2'
/// ```
/// ### Dataproc Metastore Service Scheduled Backup
///
///
/// ```yaml
/// resources:
///   backup:
///     type: gcp:dataproc:MetastoreService
///     properties:
///       serviceId: backup
///       location: us-central1
///       port: 9080
///       tier: DEVELOPER
///       maintenanceWindow:
///         hourOfDay: 2
///         dayOfWeek: SUNDAY
///       hiveMetastoreConfig:
///         version: 2.3.6
///       scheduledBackup:
///         enabled: true
///         cronSchedule: 0 0 * * *
///         timeZone: UTC
///         backupLocation: gs://${bucket.name}
///       labels:
///         env: test
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: backup
///       location: us-central1
/// ```
/// ### Dataproc Metastore Service Autoscaling Max Scaling Factor
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testResource = metastore_service::create(
///         "testResource",
///         MetastoreServiceArgs::builder()
///             .database_type("SPANNER")
///             .hive_metastore_config(
///                 MetastoreServiceHiveMetastoreConfig::builder()
///                     .version("3.1.2")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .scaling_config(
///                 MetastoreServiceScalingConfig::builder()
///                     .autoscalingConfig(
///                         MetastoreServiceScalingConfigAutoscalingConfig::builder()
///                             .autoscalingEnabled(true)
///                             .limitConfig(
///                                 MetastoreServiceScalingConfigAutoscalingConfigLimitConfig::builder()
///                                     .maxScalingFactor(1)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .service_id("test-service")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataproc Metastore Service Autoscaling Min And Max Scaling Factor
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testResource = metastore_service::create(
///         "testResource",
///         MetastoreServiceArgs::builder()
///             .database_type("SPANNER")
///             .hive_metastore_config(
///                 MetastoreServiceHiveMetastoreConfig::builder()
///                     .version("3.1.2")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .scaling_config(
///                 MetastoreServiceScalingConfig::builder()
///                     .autoscalingConfig(
///                         MetastoreServiceScalingConfigAutoscalingConfig::builder()
///                             .autoscalingEnabled(true)
///                             .limitConfig(
///                                 MetastoreServiceScalingConfigAutoscalingConfigLimitConfig::builder()
///                                     .maxScalingFactor(1)
///                                     .minScalingFactor(0.1)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .service_id("test-service")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataproc Metastore Service Autoscaling Min Scaling Factor
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testResource = metastore_service::create(
///         "testResource",
///         MetastoreServiceArgs::builder()
///             .database_type("SPANNER")
///             .hive_metastore_config(
///                 MetastoreServiceHiveMetastoreConfig::builder()
///                     .version("3.1.2")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .scaling_config(
///                 MetastoreServiceScalingConfig::builder()
///                     .autoscalingConfig(
///                         MetastoreServiceScalingConfigAutoscalingConfig::builder()
///                             .autoscalingEnabled(true)
///                             .limitConfig(
///                                 MetastoreServiceScalingConfigAutoscalingConfigLimitConfig::builder()
///                                     .minScalingFactor(0.1)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .service_id("test-service")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataproc Metastore Service Autoscaling No Limit Config
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testResource = metastore_service::create(
///         "testResource",
///         MetastoreServiceArgs::builder()
///             .database_type("SPANNER")
///             .hive_metastore_config(
///                 MetastoreServiceHiveMetastoreConfig::builder()
///                     .version("3.1.2")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .scaling_config(
///                 MetastoreServiceScalingConfig::builder()
///                     .autoscalingConfig(
///                         MetastoreServiceScalingConfigAutoscalingConfig::builder()
///                             .autoscalingEnabled(true)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .service_id("test-service")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Service can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/services/{{service_id}}`
///
/// * `{{project}}/{{location}}/{{service_id}}`
///
/// * `{{location}}/{{service_id}}`
///
/// When using the `pulumi import` command, Service can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataproc/metastoreService:MetastoreService default projects/{{project}}/locations/{{location}}/services/{{service_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/metastoreService:MetastoreService default {{project}}/{{location}}/{{service_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/metastoreService:MetastoreService default {{location}}/{{service_id}}
/// ```
///
pub mod metastore_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetastoreServiceArgs {
        /// The database type that the Metastore service stores its data.
        /// Default value is `MYSQL`.
        /// Possible values are: `MYSQL`, `SPANNER`.
        #[builder(into, default)]
        pub database_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates if the dataproc metastore should be protected against accidental deletions.
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Information used to configure the Dataproc Metastore service to encrypt
        /// customer data at rest.
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceEncryptionConfig>,
        >,
        /// Configuration information specific to running Hive metastore software as the metastore service.
        /// Structure is documented below.
        #[builder(into, default)]
        pub hive_metastore_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceHiveMetastoreConfig>,
        >,
        /// User-defined labels for the metastore service.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the metastore service should reside.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The one hour maintenance window of the metastore service.
        /// This specifies when the service can be restarted for maintenance purposes in UTC time.
        /// Maintenance window is not needed for services with the `SPANNER` database type.
        /// Structure is documented below.
        #[builder(into, default)]
        pub maintenance_window: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceMaintenanceWindow>,
        >,
        /// The setting that defines how metastore metadata should be integrated with external services and systems.
        /// Structure is documented below.
        #[builder(into, default)]
        pub metadata_integration: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceMetadataIntegration>,
        >,
        /// The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:
        /// "projects/{projectNumber}/global/networks/{network_id}".
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::Output<Option<String>>,
        /// The configuration specifying the network settings for the Dataproc Metastore service.
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceNetworkConfig>,
        >,
        /// The TCP port at which the metastore service is reached. Default: 9083.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The release channel of the service. If unspecified, defaults to `STABLE`.
        /// Default value is `STABLE`.
        /// Possible values are: `CANARY`, `STABLE`.
        #[builder(into, default)]
        pub release_channel: pulumi_wasm_rust::Output<Option<String>>,
        /// Represents the scaling configuration of a metastore service.
        /// Structure is documented below.
        #[builder(into, default)]
        pub scaling_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceScalingConfig>,
        >,
        /// The configuration of scheduled backup for the metastore service.
        /// Structure is documented below.
        #[builder(into, default)]
        pub scheduled_backup: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceScheduledBackup>,
        >,
        /// The ID of the metastore service. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),
        /// and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between
        /// 3 and 63 characters.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_id: pulumi_wasm_rust::Output<String>,
        /// The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON.
        /// Structure is documented below.
        #[builder(into, default)]
        pub telemetry_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceTelemetryConfig>,
        >,
        /// The tier of the service.
        /// Possible values are: `DEVELOPER`, `ENTERPRISE`.
        #[builder(into, default)]
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MetastoreServiceResult {
        /// A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored.
        pub artifact_gcs_uri: pulumi_wasm_rust::Output<String>,
        /// The database type that the Metastore service stores its data.
        /// Default value is `MYSQL`.
        /// Possible values are: `MYSQL`, `SPANNER`.
        pub database_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Indicates if the dataproc metastore should be protected against accidental deletions.
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Information used to configure the Dataproc Metastore service to encrypt
        /// customer data at rest.
        /// Structure is documented below.
        pub encryption_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceEncryptionConfig>,
        >,
        /// The URI of the endpoint used to access the metastore service.
        pub endpoint_uri: pulumi_wasm_rust::Output<String>,
        /// Configuration information specific to running Hive metastore software as the metastore service.
        /// Structure is documented below.
        pub hive_metastore_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceHiveMetastoreConfig>,
        >,
        /// User-defined labels for the metastore service.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the metastore service should reside.
        /// The default value is `global`.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The one hour maintenance window of the metastore service.
        /// This specifies when the service can be restarted for maintenance purposes in UTC time.
        /// Maintenance window is not needed for services with the `SPANNER` database type.
        /// Structure is documented below.
        pub maintenance_window: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceMaintenanceWindow>,
        >,
        /// The setting that defines how metastore metadata should be integrated with external services and systems.
        /// Structure is documented below.
        pub metadata_integration: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceMetadataIntegration>,
        >,
        /// The relative resource name of the metastore service.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:
        /// "projects/{projectNumber}/global/networks/{network_id}".
        pub network: pulumi_wasm_rust::Output<String>,
        /// The configuration specifying the network settings for the Dataproc Metastore service.
        /// Structure is documented below.
        pub network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceNetworkConfig>,
        >,
        /// The TCP port at which the metastore service is reached. Default: 9083.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The release channel of the service. If unspecified, defaults to `STABLE`.
        /// Default value is `STABLE`.
        /// Possible values are: `CANARY`, `STABLE`.
        pub release_channel: pulumi_wasm_rust::Output<Option<String>>,
        /// Represents the scaling configuration of a metastore service.
        /// Structure is documented below.
        pub scaling_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceScalingConfig>,
        >,
        /// The configuration of scheduled backup for the metastore service.
        /// Structure is documented below.
        pub scheduled_backup: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::MetastoreServiceScheduledBackup>,
        >,
        /// The ID of the metastore service. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),
        /// and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between
        /// 3 and 63 characters.
        ///
        ///
        /// - - -
        pub service_id: pulumi_wasm_rust::Output<String>,
        /// The current state of the metastore service.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Additional information about the current state of the metastore service, if available.
        pub state_message: pulumi_wasm_rust::Output<String>,
        /// The configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON.
        /// Structure is documented below.
        pub telemetry_config: pulumi_wasm_rust::Output<
            super::super::types::dataproc::MetastoreServiceTelemetryConfig,
        >,
        /// The tier of the service.
        /// Possible values are: `DEVELOPER`, `ENTERPRISE`.
        pub tier: pulumi_wasm_rust::Output<String>,
        /// The globally unique resource identifier of the metastore service.
        pub uid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MetastoreServiceArgs) -> MetastoreServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let database_type_binding = args.database_type.get_inner();
        let deletion_protection_binding = args.deletion_protection.get_inner();
        let encryption_config_binding = args.encryption_config.get_inner();
        let hive_metastore_config_binding = args.hive_metastore_config.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let maintenance_window_binding = args.maintenance_window.get_inner();
        let metadata_integration_binding = args.metadata_integration.get_inner();
        let network_binding = args.network.get_inner();
        let network_config_binding = args.network_config.get_inner();
        let port_binding = args.port.get_inner();
        let project_binding = args.project.get_inner();
        let release_channel_binding = args.release_channel.get_inner();
        let scaling_config_binding = args.scaling_config.get_inner();
        let scheduled_backup_binding = args.scheduled_backup.get_inner();
        let service_id_binding = args.service_id.get_inner();
        let telemetry_config_binding = args.telemetry_config.get_inner();
        let tier_binding = args.tier.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/metastoreService:MetastoreService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "databaseType".into(),
                    value: &database_type_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfig".into(),
                    value: &encryption_config_binding,
                },
                register_interface::ObjectField {
                    name: "hiveMetastoreConfig".into(),
                    value: &hive_metastore_config_binding,
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
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "metadataIntegration".into(),
                    value: &metadata_integration_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfig".into(),
                    value: &network_config_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "releaseChannel".into(),
                    value: &release_channel_binding,
                },
                register_interface::ObjectField {
                    name: "scalingConfig".into(),
                    value: &scaling_config_binding,
                },
                register_interface::ObjectField {
                    name: "scheduledBackup".into(),
                    value: &scheduled_backup_binding,
                },
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
                },
                register_interface::ObjectField {
                    name: "telemetryConfig".into(),
                    value: &telemetry_config_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "artifactGcsUri".into(),
                },
                register_interface::ResultField {
                    name: "databaseType".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfig".into(),
                },
                register_interface::ResultField {
                    name: "endpointUri".into(),
                },
                register_interface::ResultField {
                    name: "hiveMetastoreConfig".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "metadataIntegration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "networkConfig".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "releaseChannel".into(),
                },
                register_interface::ResultField {
                    name: "scalingConfig".into(),
                },
                register_interface::ResultField {
                    name: "scheduledBackup".into(),
                },
                register_interface::ResultField {
                    name: "serviceId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateMessage".into(),
                },
                register_interface::ResultField {
                    name: "telemetryConfig".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MetastoreServiceResult {
            artifact_gcs_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("artifactGcsUri").unwrap(),
            ),
            database_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseType").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            encryption_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfig").unwrap(),
            ),
            endpoint_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointUri").unwrap(),
            ),
            hive_metastore_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hiveMetastoreConfig").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceWindow").unwrap(),
            ),
            metadata_integration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadataIntegration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            network_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfig").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            release_channel: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseChannel").unwrap(),
            ),
            scaling_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalingConfig").unwrap(),
            ),
            scheduled_backup: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduledBackup").unwrap(),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateMessage").unwrap(),
            ),
            telemetry_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("telemetryConfig").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(hashmap.remove("uid").unwrap()),
        }
    }
}
