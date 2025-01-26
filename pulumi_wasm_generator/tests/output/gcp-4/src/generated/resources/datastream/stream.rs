/// A resource representing streaming data from a source to a destination.
///
///
/// To get more information about Stream, see:
///
/// * [API documentation](https://cloud.google.com/datastream/docs/reference/rest/v1/projects.locations.streams)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/datastream/docs/create-a-stream)
///
/// ## Example Usage
///
/// ### Datastream Stream Full
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-instance
///       databaseVersion: MYSQL_8_0
///       region: us-central1
///       settings:
///         tier: db-f1-micro
///         backupConfiguration:
///           enabled: true
///           binaryLogEnabled: true
///         ipConfiguration:
///           authorizedNetworks:
///             - value: 34.71.242.81
///             - value: 34.72.28.29
///             - value: 34.67.6.157
///             - value: 34.67.234.134
///             - value: 34.72.239.218
///       deletionProtection: true
///   db:
///     type: gcp:sql:Database
///     properties:
///       instance: ${instance.name}
///       name: db
///   pwd:
///     type: random:RandomPassword
///     properties:
///       length: 16
///       special: false
///   user:
///     type: gcp:sql:User
///     properties:
///       name: user
///       instance: ${instance.name}
///       host: '%'
///       password: ${pwd.result}
///   sourceConnectionProfile:
///     type: gcp:datastream:ConnectionProfile
///     name: source_connection_profile
///     properties:
///       displayName: Source connection profile
///       location: us-central1
///       connectionProfileId: source-profile
///       mysqlProfile:
///         hostname: ${instance.publicIpAddress}
///         username: ${user.name}
///         password: ${user.password}
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: my-bucket
///       location: US
///       uniformBucketLevelAccess: true
///   viewer:
///     type: gcp:storage:BucketIAMMember
///     properties:
///       bucket: ${bucket.name}
///       role: roles/storage.objectViewer
///       member: serviceAccount:service-${project.number}@gcp-sa-datastream.iam.gserviceaccount.com
///   creator:
///     type: gcp:storage:BucketIAMMember
///     properties:
///       bucket: ${bucket.name}
///       role: roles/storage.objectCreator
///       member: serviceAccount:service-${project.number}@gcp-sa-datastream.iam.gserviceaccount.com
///   reader:
///     type: gcp:storage:BucketIAMMember
///     properties:
///       bucket: ${bucket.name}
///       role: roles/storage.legacyBucketReader
///       member: serviceAccount:service-${project.number}@gcp-sa-datastream.iam.gserviceaccount.com
///   keyUser:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: key_user
///     properties:
///       cryptoKeyId: kms-name
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-datastream.iam.gserviceaccount.com
///   destinationConnectionProfile:
///     type: gcp:datastream:ConnectionProfile
///     name: destination_connection_profile
///     properties:
///       displayName: Connection profile
///       location: us-central1
///       connectionProfileId: destination-profile
///       gcsProfile:
///         bucket: ${bucket.name}
///         rootPath: /path
///   default:
///     type: gcp:datastream:Stream
///     properties:
///       streamId: my-stream
///       desiredState: NOT_STARTED
///       location: us-central1
///       displayName: my stream
///       labels:
///         key: value
///       sourceConfig:
///         sourceConnectionProfile: ${sourceConnectionProfile.id}
///         mysqlSourceConfig:
///           includeObjects:
///             mysqlDatabases:
///               - database: my-database
///                 mysqlTables:
///                   - table: includedTable
///                     mysqlColumns:
///                       - column: includedColumn
///                         dataType: VARCHAR
///                         collation: utf8mb4
///                         primaryKey: false
///                         nullable: false
///                         ordinalPosition: 0
///                   - table: includedTable_2
///           excludeObjects:
///             mysqlDatabases:
///               - database: my-database
///                 mysqlTables:
///                   - table: excludedTable
///                     mysqlColumns:
///                       - column: excludedColumn
///                         dataType: VARCHAR
///                         collation: utf8mb4
///                         primaryKey: false
///                         nullable: false
///                         ordinalPosition: 0
///           maxConcurrentCdcTasks: 5
///       destinationConfig:
///         destinationConnectionProfile: ${destinationConnectionProfile.id}
///         gcsDestinationConfig:
///           path: mydata
///           fileRotationMb: 200
///           fileRotationInterval: 60s
///           jsonFileFormat:
///             schemaFileFormat: NO_SCHEMA_FILE
///             compression: GZIP
///       backfillAll:
///         mysqlExcludedObjects:
///           mysqlDatabases:
///             - database: my-database
///               mysqlTables:
///                 - table: excludedTable
///                   mysqlColumns:
///                     - column: excludedColumn
///                       dataType: VARCHAR
///                       collation: utf8mb4
///                       primaryKey: false
///                       nullable: false
///                       ordinalPosition: 0
///       customerManagedEncryptionKey: kms-name
///     options:
///       dependsOn:
///         - ${keyUser}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Datastream Stream Postgresql
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = stream::create(
///         "default",
///         StreamArgs::builder()
///             .backfill_all(
///                 StreamBackfillAll::builder()
///                     .postgresqlExcludedObjects(
///                         StreamBackfillAllPostgresqlExcludedObjects::builder()
///                             .postgresqlSchemas(
///                                 vec![
///                                     StreamBackfillAllPostgresqlExcludedObjectsPostgresqlSchema::builder()
///                                     .postgresqlTables(vec![StreamBackfillAllPostgresqlExcludedObjectsPostgresqlSchemaPostgresqlTable::builder()
///                                     .postgresqlColumns(vec![StreamBackfillAllPostgresqlExcludedObjectsPostgresqlSchemaPostgresqlTablePostgresqlColumn::builder()
///                                     .column("column").build_struct(),]).table("table")
///                                     .build_struct(),]).schema("schema").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .desired_state("RUNNING")
///             .destination_config(
///                 StreamDestinationConfig::builder()
///                     .bigqueryDestinationConfig(
///                         StreamDestinationConfigBigqueryDestinationConfig::builder()
///                             .dataFreshness("900s")
///                             .sourceHierarchyDatasets(
///                                 StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasets::builder()
///                                     .datasetTemplate(
///                                         StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasetsDatasetTemplate::builder()
///                                             .location("us-central1")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .destinationConnectionProfile("${destination.id}")
///                     .build_struct(),
///             )
///             .display_name("Postgres to BigQuery")
///             .location("us-central1")
///             .source_config(
///                 StreamSourceConfig::builder()
///                     .postgresqlSourceConfig(
///                         StreamSourceConfigPostgresqlSourceConfig::builder()
///                             .excludeObjects(
///                                 StreamSourceConfigPostgresqlSourceConfigExcludeObjects::builder()
///                                     .postgresqlSchemas(
///                                         vec![
///                                             StreamSourceConfigPostgresqlSourceConfigExcludeObjectsPostgresqlSchema::builder()
///                                             .postgresqlTables(vec![StreamSourceConfigPostgresqlSourceConfigExcludeObjectsPostgresqlSchemaPostgresqlTable::builder()
///                                             .postgresqlColumns(vec![StreamSourceConfigPostgresqlSourceConfigExcludeObjectsPostgresqlSchemaPostgresqlTablePostgresqlColumn::builder()
///                                             .column("column").build_struct(),]).table("table")
///                                             .build_struct(),]).schema("schema").build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .includeObjects(
///                                 StreamSourceConfigPostgresqlSourceConfigIncludeObjects::builder()
///                                     .postgresqlSchemas(
///                                         vec![
///                                             StreamSourceConfigPostgresqlSourceConfigIncludeObjectsPostgresqlSchema::builder()
///                                             .postgresqlTables(vec![StreamSourceConfigPostgresqlSourceConfigIncludeObjectsPostgresqlSchemaPostgresqlTable::builder()
///                                             .postgresqlColumns(vec![StreamSourceConfigPostgresqlSourceConfigIncludeObjectsPostgresqlSchemaPostgresqlTablePostgresqlColumn::builder()
///                                             .column("column").build_struct(),]).table("table")
///                                             .build_struct(),]).schema("schema").build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .maxConcurrentBackfillTasks(12)
///                             .publication("publication")
///                             .replicationSlot("replication_slot")
///                             .build_struct(),
///                     )
///                     .sourceConnectionProfile("${source.id}")
///                     .build_struct(),
///             )
///             .stream_id("my-stream")
///             .build_struct(),
///     );
///     let destination = connection_profile::create(
///         "destination",
///         ConnectionProfileArgs::builder()
///             .bigquery_profile(ConnectionProfileBigqueryProfile::builder().build_struct())
///             .connection_profile_id("destination-profile")
///             .display_name("BigQuery Destination")
///             .location("us-central1")
///             .build_struct(),
///     );
///     let source = connection_profile::create(
///         "source",
///         ConnectionProfileArgs::builder()
///             .connection_profile_id("source-profile")
///             .display_name("Postgresql Source")
///             .location("us-central1")
///             .postgresql_profile(
///                 ConnectionProfilePostgresqlProfile::builder()
///                     .database("postgres")
///                     .hostname("hostname")
///                     .password("pass")
///                     .port(5432)
///                     .username("user")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Datastream Stream Oracle
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let destination = connection_profile::create(
///         "destination",
///         ConnectionProfileArgs::builder()
///             .bigquery_profile(ConnectionProfileBigqueryProfile::builder().build_struct())
///             .connection_profile_id("destination-profile")
///             .display_name("BigQuery Destination")
///             .location("us-central1")
///             .build_struct(),
///     );
///     let source = connection_profile::create(
///         "source",
///         ConnectionProfileArgs::builder()
///             .connection_profile_id("source-profile")
///             .display_name("Oracle Source")
///             .location("us-central1")
///             .oracle_profile(
///                 ConnectionProfileOracleProfile::builder()
///                     .databaseService("ORCL")
///                     .hostname("hostname")
///                     .password("pass")
///                     .port(1521)
///                     .username("user")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let stream5 = stream::create(
///         "stream5",
///         StreamArgs::builder()
///             .backfill_all(
///                 StreamBackfillAll::builder()
///                     .oracleExcludedObjects(
///                         StreamBackfillAllOracleExcludedObjects::builder()
///                             .oracleSchemas(
///                                 vec![
///                                     StreamBackfillAllOracleExcludedObjectsOracleSchema::builder()
///                                     .oracleTables(vec![StreamBackfillAllOracleExcludedObjectsOracleSchemaOracleTable::builder()
///                                     .oracleColumns(vec![StreamBackfillAllOracleExcludedObjectsOracleSchemaOracleTableOracleColumn::builder()
///                                     .column("column").build_struct(),]).table("table")
///                                     .build_struct(),]).schema("schema").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .desired_state("RUNNING")
///             .destination_config(
///                 StreamDestinationConfig::builder()
///                     .bigqueryDestinationConfig(
///                         StreamDestinationConfigBigqueryDestinationConfig::builder()
///                             .dataFreshness("900s")
///                             .sourceHierarchyDatasets(
///                                 StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasets::builder()
///                                     .datasetTemplate(
///                                         StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasetsDatasetTemplate::builder()
///                                             .location("us-central1")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .destinationConnectionProfile("${destination.id}")
///                     .build_struct(),
///             )
///             .display_name("Oracle to BigQuery")
///             .location("us-central1")
///             .source_config(
///                 StreamSourceConfig::builder()
///                     .oracleSourceConfig(
///                         StreamSourceConfigOracleSourceConfig::builder()
///                             .dropLargeObjects(
///                                 StreamSourceConfigOracleSourceConfigDropLargeObjects::builder()
///                                     .build_struct(),
///                             )
///                             .excludeObjects(
///                                 StreamSourceConfigOracleSourceConfigExcludeObjects::builder()
///                                     .oracleSchemas(
///                                         vec![
///                                             StreamSourceConfigOracleSourceConfigExcludeObjectsOracleSchema::builder()
///                                             .oracleTables(vec![StreamSourceConfigOracleSourceConfigExcludeObjectsOracleSchemaOracleTable::builder()
///                                             .oracleColumns(vec![StreamSourceConfigOracleSourceConfigExcludeObjectsOracleSchemaOracleTableOracleColumn::builder()
///                                             .column("column").build_struct(),]).table("table")
///                                             .build_struct(),]).schema("schema").build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .includeObjects(
///                                 StreamSourceConfigOracleSourceConfigIncludeObjects::builder()
///                                     .oracleSchemas(
///                                         vec![
///                                             StreamSourceConfigOracleSourceConfigIncludeObjectsOracleSchema::builder()
///                                             .oracleTables(vec![StreamSourceConfigOracleSourceConfigIncludeObjectsOracleSchemaOracleTable::builder()
///                                             .oracleColumns(vec![StreamSourceConfigOracleSourceConfigIncludeObjectsOracleSchemaOracleTableOracleColumn::builder()
///                                             .column("column").build_struct(),]).table("table")
///                                             .build_struct(),]).schema("schema").build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .maxConcurrentBackfillTasks(12)
///                             .maxConcurrentCdcTasks(8)
///                             .build_struct(),
///                     )
///                     .sourceConnectionProfile("${source.id}")
///                     .build_struct(),
///             )
///             .stream_id("my-stream")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Datastream Stream Sql Server
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let db = database::create(
///         "db",
///         DatabaseArgs::builder().instance("${instance.name}").name("db").build_struct(),
///     );
///     let default = stream::create(
///         "default",
///         StreamArgs::builder()
///             .backfill_none(StreamBackfillNone::builder().build_struct())
///             .destination_config(
///                 StreamDestinationConfig::builder()
///                     .bigqueryDestinationConfig(
///                         StreamDestinationConfigBigqueryDestinationConfig::builder()
///                             .dataFreshness("900s")
///                             .sourceHierarchyDatasets(
///                                 StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasets::builder()
///                                     .datasetTemplate(
///                                         StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasetsDatasetTemplate::builder()
///                                             .location("us-central1")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .destinationConnectionProfile("${destination.id}")
///                     .build_struct(),
///             )
///             .display_name("SQL Server to BigQuery")
///             .location("us-central1")
///             .source_config(
///                 StreamSourceConfig::builder()
///                     .sourceConnectionProfile("${source.id}")
///                     .sqlServerSourceConfig(
///                         StreamSourceConfigSqlServerSourceConfig::builder()
///                             .includeObjects(
///                                 StreamSourceConfigSqlServerSourceConfigIncludeObjects::builder()
///                                     .schemas(
///                                         vec![
///                                             StreamSourceConfigSqlServerSourceConfigIncludeObjectsSchema::builder()
///                                             .schema("schema")
///                                             .tables(vec![StreamSourceConfigSqlServerSourceConfigIncludeObjectsSchemaTable::builder()
///                                             .table("table").build_struct(),]).build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .transactionLogs(
///                                 StreamSourceConfigSqlServerSourceConfigTransactionLogs::builder()
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .stream_id("stream")
///             .build_struct(),
///     );
///     let destination = connection_profile::create(
///         "destination",
///         ConnectionProfileArgs::builder()
///             .bigquery_profile(ConnectionProfileBigqueryProfile::builder().build_struct())
///             .connection_profile_id("destination-profile")
///             .display_name("BigQuery Destination")
///             .location("us-central1")
///             .build_struct(),
///     );
///     let instance = database_instance::create(
///         "instance",
///         DatabaseInstanceArgs::builder()
///             .database_version("SQLSERVER_2019_STANDARD")
///             .deletion_protection(true)
///             .name("sql-server")
///             .region("us-central1")
///             .root_password("root-password")
///             .settings(
///                 DatabaseInstanceSettings::builder()
///                     .ipConfiguration(
///                         DatabaseInstanceSettingsIpConfiguration::builder()
///                             .authorizedNetworks(
///                                 vec![
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.71.242.81").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.72.28.29").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.67.6.157").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.67.234.134").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.72.239.218").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .tier("db-custom-2-4096")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let source = connection_profile::create(
///         "source",
///         ConnectionProfileArgs::builder()
///             .connection_profile_id("source-profile")
///             .display_name("SQL Server Source")
///             .location("us-central1")
///             .sql_server_profile(
///                 ConnectionProfileSqlServerProfile::builder()
///                     .database("${db.name}")
///                     .hostname("${instance.publicIpAddress}")
///                     .password("${user.password}")
///                     .port(1433)
///                     .username("${user.name}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let user = user::create(
///         "user",
///         UserArgs::builder()
///             .instance("${instance.name}")
///             .name("user")
///             .password("password")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Datastream Stream Sql Server Change Tables
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let db = database::create(
///         "db",
///         DatabaseArgs::builder().instance("${instance.name}").name("db").build_struct(),
///     );
///     let default = stream::create(
///         "default",
///         StreamArgs::builder()
///             .backfill_none(StreamBackfillNone::builder().build_struct())
///             .destination_config(
///                 StreamDestinationConfig::builder()
///                     .bigqueryDestinationConfig(
///                         StreamDestinationConfigBigqueryDestinationConfig::builder()
///                             .dataFreshness("900s")
///                             .sourceHierarchyDatasets(
///                                 StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasets::builder()
///                                     .datasetTemplate(
///                                         StreamDestinationConfigBigqueryDestinationConfigSourceHierarchyDatasetsDatasetTemplate::builder()
///                                             .location("us-central1")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .destinationConnectionProfile("${destination.id}")
///                     .build_struct(),
///             )
///             .display_name("SQL Server to BigQuery")
///             .location("us-central1")
///             .source_config(
///                 StreamSourceConfig::builder()
///                     .sourceConnectionProfile("${source.id}")
///                     .sqlServerSourceConfig(
///                         StreamSourceConfigSqlServerSourceConfig::builder()
///                             .changeTables(
///                                 StreamSourceConfigSqlServerSourceConfigChangeTables::builder()
///                                     .build_struct(),
///                             )
///                             .includeObjects(
///                                 StreamSourceConfigSqlServerSourceConfigIncludeObjects::builder()
///                                     .schemas(
///                                         vec![
///                                             StreamSourceConfigSqlServerSourceConfigIncludeObjectsSchema::builder()
///                                             .schema("schema")
///                                             .tables(vec![StreamSourceConfigSqlServerSourceConfigIncludeObjectsSchemaTable::builder()
///                                             .table("table").build_struct(),]).build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .stream_id("stream")
///             .build_struct(),
///     );
///     let destination = connection_profile::create(
///         "destination",
///         ConnectionProfileArgs::builder()
///             .bigquery_profile(ConnectionProfileBigqueryProfile::builder().build_struct())
///             .connection_profile_id("destination-profile")
///             .display_name("BigQuery Destination")
///             .location("us-central1")
///             .build_struct(),
///     );
///     let instance = database_instance::create(
///         "instance",
///         DatabaseInstanceArgs::builder()
///             .database_version("SQLSERVER_2019_STANDARD")
///             .deletion_protection(true)
///             .name("sql-server")
///             .region("us-central1")
///             .root_password("root-password")
///             .settings(
///                 DatabaseInstanceSettings::builder()
///                     .ipConfiguration(
///                         DatabaseInstanceSettingsIpConfiguration::builder()
///                             .authorizedNetworks(
///                                 vec![
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.71.242.81").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.72.28.29").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.67.6.157").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.67.234.134").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.72.239.218").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .tier("db-custom-2-4096")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let source = connection_profile::create(
///         "source",
///         ConnectionProfileArgs::builder()
///             .connection_profile_id("source-profile")
///             .display_name("SQL Server Source")
///             .location("us-central1")
///             .sql_server_profile(
///                 ConnectionProfileSqlServerProfile::builder()
///                     .database("${db.name}")
///                     .hostname("${instance.publicIpAddress}")
///                     .password("${user.password}")
///                     .port(1433)
///                     .username("${user.name}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let user = user::create(
///         "user",
///         UserArgs::builder()
///             .instance("${instance.name}")
///             .name("user")
///             .password("password")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Datastream Stream Postgresql Bigquery Dataset Id
///
///
/// ```yaml
/// resources:
///   postgres:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: postgres
///       friendlyName: postgres
///       description: Database of postgres
///       location: us-central1
///   default:
///     type: gcp:datastream:Stream
///     properties:
///       displayName: postgres to bigQuery
///       location: us-central1
///       streamId: postgres-bigquery
///       sourceConfig:
///         sourceConnectionProfile: ${sourceConnectionProfile.id}
///         mysqlSourceConfig: {}
///       destinationConfig:
///         destinationConnectionProfile: ${destinationConnectionProfile2.id}
///         bigqueryDestinationConfig:
///           dataFreshness: 900s
///           singleTargetDataset:
///             datasetId: ${postgres.id}
///       backfillAll: {}
///   destinationConnectionProfile2:
///     type: gcp:datastream:ConnectionProfile
///     name: destination_connection_profile2
///     properties:
///       displayName: Connection profile
///       location: us-central1
///       connectionProfileId: dest-profile
///       bigqueryProfile: {}
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: instance-name
///       databaseVersion: MYSQL_8_0
///       region: us-central1
///       settings:
///         tier: db-f1-micro
///         backupConfiguration:
///           enabled: true
///           binaryLogEnabled: true
///         ipConfiguration:
///           authorizedNetworks:
///             - value: 34.71.242.81
///             - value: 34.72.28.29
///             - value: 34.67.6.157
///             - value: 34.67.234.134
///             - value: 34.72.239.218
///       deletionProtection: false
///   db:
///     type: gcp:sql:Database
///     properties:
///       instance: ${instance.name}
///       name: db
///   pwd:
///     type: random:RandomPassword
///     properties:
///       length: 16
///       special: false
///   user:
///     type: gcp:sql:User
///     properties:
///       name: my-user
///       instance: ${instance.name}
///       host: '%'
///       password: ${pwd.result}
///   sourceConnectionProfile:
///     type: gcp:datastream:ConnectionProfile
///     name: source_connection_profile
///     properties:
///       displayName: Source connection profile
///       location: us-central1
///       connectionProfileId: source-profile
///       mysqlProfile:
///         hostname: ${instance.publicIpAddress}
///         username: ${user.name}
///         password: ${user.password}
/// ```
/// ### Datastream Stream Bigquery
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-instance
///       databaseVersion: MYSQL_8_0
///       region: us-central1
///       settings:
///         tier: db-f1-micro
///         backupConfiguration:
///           enabled: true
///           binaryLogEnabled: true
///         ipConfiguration:
///           authorizedNetworks:
///             - value: 34.71.242.81
///             - value: 34.72.28.29
///             - value: 34.67.6.157
///             - value: 34.67.234.134
///             - value: 34.72.239.218
///       deletionProtection: true
///   db:
///     type: gcp:sql:Database
///     properties:
///       instance: ${instance.name}
///       name: db
///   pwd:
///     type: random:RandomPassword
///     properties:
///       length: 16
///       special: false
///   user:
///     type: gcp:sql:User
///     properties:
///       name: user
///       instance: ${instance.name}
///       host: '%'
///       password: ${pwd.result}
///   sourceConnectionProfile:
///     type: gcp:datastream:ConnectionProfile
///     name: source_connection_profile
///     properties:
///       displayName: Source connection profile
///       location: us-central1
///       connectionProfileId: source-profile
///       mysqlProfile:
///         hostname: ${instance.publicIpAddress}
///         username: ${user.name}
///         password: ${user.password}
///   bigqueryKeyUser:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: bigquery_key_user
///     properties:
///       cryptoKeyId: bigquery-kms-name
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:${bqSa.email}
///   destinationConnectionProfile:
///     type: gcp:datastream:ConnectionProfile
///     name: destination_connection_profile
///     properties:
///       displayName: Connection profile
///       location: us-central1
///       connectionProfileId: destination-profile
///       bigqueryProfile: {}
///   default:
///     type: gcp:datastream:Stream
///     properties:
///       streamId: my-stream
///       location: us-central1
///       displayName: my stream
///       sourceConfig:
///         sourceConnectionProfile: ${sourceConnectionProfile.id}
///         mysqlSourceConfig: {}
///       destinationConfig:
///         destinationConnectionProfile: ${destinationConnectionProfile.id}
///         bigqueryDestinationConfig:
///           sourceHierarchyDatasets:
///             datasetTemplate:
///               location: us-central1
///               kmsKeyName: bigquery-kms-name
///       backfillNone: {}
///     options:
///       dependsOn:
///         - ${bigqueryKeyUser}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
///   bqSa:
///     fn::invoke:
///       function: gcp:bigquery:getDefaultServiceAccount
///       arguments: {}
/// ```
/// ### Datastream Stream Bigquery Append Only
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-instance
///       databaseVersion: MYSQL_8_0
///       region: us-central1
///       settings:
///         tier: db-f1-micro
///         backupConfiguration:
///           enabled: true
///           binaryLogEnabled: true
///         ipConfiguration:
///           authorizedNetworks:
///             - value: 34.71.242.81
///             - value: 34.72.28.29
///             - value: 34.67.6.157
///             - value: 34.67.234.134
///             - value: 34.72.239.218
///       deletionProtection: true
///   db:
///     type: gcp:sql:Database
///     properties:
///       instance: ${instance.name}
///       name: db
///   pwd:
///     type: random:RandomPassword
///     properties:
///       length: 16
///       special: false
///   user:
///     type: gcp:sql:User
///     properties:
///       name: user
///       instance: ${instance.name}
///       host: '%'
///       password: ${pwd.result}
///   sourceConnectionProfile:
///     type: gcp:datastream:ConnectionProfile
///     name: source_connection_profile
///     properties:
///       displayName: Source connection profile
///       location: us-central1
///       connectionProfileId: source-profile
///       mysqlProfile:
///         hostname: ${instance.publicIpAddress}
///         username: ${user.name}
///         password: ${user.password}
///   destinationConnectionProfile:
///     type: gcp:datastream:ConnectionProfile
///     name: destination_connection_profile
///     properties:
///       displayName: Connection profile
///       location: us-central1
///       connectionProfileId: destination-profile
///       bigqueryProfile: {}
///   default:
///     type: gcp:datastream:Stream
///     properties:
///       streamId: my-stream
///       location: us-central1
///       displayName: my stream
///       sourceConfig:
///         sourceConnectionProfile: ${sourceConnectionProfile.id}
///         mysqlSourceConfig: {}
///       destinationConfig:
///         destinationConnectionProfile: ${destinationConnectionProfile.id}
///         bigqueryDestinationConfig:
///           sourceHierarchyDatasets:
///             datasetTemplate:
///               location: us-central1
///           appendOnly: {}
///       backfillNone: {}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Stream can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/streams/{{stream_id}}`
///
/// * `{{project}}/{{location}}/{{stream_id}}`
///
/// * `{{location}}/{{stream_id}}`
///
/// When using the `pulumi import` command, Stream can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datastream/stream:Stream default projects/{{project}}/locations/{{location}}/streams/{{stream_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datastream/stream:Stream default {{project}}/{{location}}/{{stream_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datastream/stream:Stream default {{location}}/{{stream_id}}
/// ```
///
pub mod stream {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamArgs {
        /// Backfill strategy to automatically backfill the Stream's objects. Specific objects can be excluded.
        #[builder(into, default)]
        pub backfill_all: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::datastream::StreamBackfillAll>,
        >,
        /// Backfill strategy to disable automatic backfill for the Stream's objects.
        #[builder(into, default)]
        pub backfill_none: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::datastream::StreamBackfillNone>,
        >,
        /// Create the stream without validating it.
        #[builder(into, default)]
        pub create_without_validation: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data will be
        /// encrypted using an internal Stream-specific encryption key provisioned through KMS.
        #[builder(into, default)]
        pub customer_managed_encryption_key: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Desired state of the Stream. Set this field to 'RUNNING' to start the stream, 'NOT_STARTED' to create the stream without
        /// starting and 'PAUSED' to pause the stream from a 'RUNNING' state. Possible values: NOT_STARTED, RUNNING, PAUSED.
        /// Default: NOT_STARTED
        #[builder(into, default)]
        pub desired_state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Destination connection profile configuration.
        /// Structure is documented below.
        #[builder(into)]
        pub destination_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::datastream::StreamDestinationConfig,
        >,
        /// Display name.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Labels. **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please
        /// refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location this stream is located in.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Source connection profile configuration.
        /// Structure is documented below.
        #[builder(into)]
        pub source_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::datastream::StreamSourceConfig,
        >,
        /// The stream identifier.
        #[builder(into)]
        pub stream_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StreamResult {
        /// Backfill strategy to automatically backfill the Stream's objects. Specific objects can be excluded.
        pub backfill_all: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::StreamBackfillAll>,
        >,
        /// Backfill strategy to disable automatic backfill for the Stream's objects.
        pub backfill_none: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::StreamBackfillNone>,
        >,
        /// Create the stream without validating it.
        pub create_without_validation: pulumi_wasm_rust::Output<Option<bool>>,
        /// A reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data will be
        /// encrypted using an internal Stream-specific encryption key provisioned through KMS.
        pub customer_managed_encryption_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Desired state of the Stream. Set this field to 'RUNNING' to start the stream, 'NOT_STARTED' to create the stream without
        /// starting and 'PAUSED' to pause the stream from a 'RUNNING' state. Possible values: NOT_STARTED, RUNNING, PAUSED.
        /// Default: NOT_STARTED
        pub desired_state: pulumi_wasm_rust::Output<Option<String>>,
        /// Destination connection profile configuration.
        /// Structure is documented below.
        pub destination_config: pulumi_wasm_rust::Output<
            super::super::types::datastream::StreamDestinationConfig,
        >,
        /// Display name.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Labels. **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please
        /// refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location this stream is located in.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The stream's name.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Source connection profile configuration.
        /// Structure is documented below.
        pub source_config: pulumi_wasm_rust::Output<
            super::super::types::datastream::StreamSourceConfig,
        >,
        /// The state of the stream.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The stream identifier.
        pub stream_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StreamArgs,
    ) -> StreamResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backfill_all_binding = args.backfill_all.get_output(context).get_inner();
        let backfill_none_binding = args.backfill_none.get_output(context).get_inner();
        let create_without_validation_binding = args
            .create_without_validation
            .get_output(context)
            .get_inner();
        let customer_managed_encryption_key_binding = args
            .customer_managed_encryption_key
            .get_output(context)
            .get_inner();
        let desired_state_binding = args.desired_state.get_output(context).get_inner();
        let destination_config_binding = args
            .destination_config
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let source_config_binding = args.source_config.get_output(context).get_inner();
        let stream_id_binding = args.stream_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:datastream/stream:Stream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backfillAll".into(),
                    value: &backfill_all_binding,
                },
                register_interface::ObjectField {
                    name: "backfillNone".into(),
                    value: &backfill_none_binding,
                },
                register_interface::ObjectField {
                    name: "createWithoutValidation".into(),
                    value: &create_without_validation_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedEncryptionKey".into(),
                    value: &customer_managed_encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "desiredState".into(),
                    value: &desired_state_binding,
                },
                register_interface::ObjectField {
                    name: "destinationConfig".into(),
                    value: &destination_config_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
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
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "sourceConfig".into(),
                    value: &source_config_binding,
                },
                register_interface::ObjectField {
                    name: "streamId".into(),
                    value: &stream_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StreamResult {
            backfill_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backfillAll"),
            ),
            backfill_none: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backfillNone"),
            ),
            create_without_validation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createWithoutValidation"),
            ),
            customer_managed_encryption_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerManagedEncryptionKey"),
            ),
            desired_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("desiredState"),
            ),
            destination_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinationConfig"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            source_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceConfig"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            stream_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("streamId"),
            ),
        }
    }
}
