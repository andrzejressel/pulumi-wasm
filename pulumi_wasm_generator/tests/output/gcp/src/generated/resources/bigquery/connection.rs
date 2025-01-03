/// A connection allows BigQuery connections to external data sources..
///
///
/// To get more information about Connection, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/bigqueryconnection/rest/v1/projects.locations.connections/create)
/// * How-to Guides
///     * [Cloud SQL federated queries](https://cloud.google.com/bigquery/docs/cloud-sql-federated-queries)
///
///
///
/// ## Example Usage
///
/// ### Bigquery Connection Cloud Resource
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let connection = connection::create(
///         "connection",
///         ConnectionArgs::builder()
///             .cloud_resource(ConnectionCloudResource::builder().build_struct())
///             .connection_id("my-connection")
///             .description("a riveting description")
///             .friendly_name("👋")
///             .location("US")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Connection Basic
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-database-instance
///       databaseVersion: POSTGRES_11
///       region: us-central1
///       settings:
///         tier: db-f1-micro
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
///       password: ${pwd.result}
///   connection:
///     type: gcp:bigquery:Connection
///     properties:
///       friendlyName: "\U0001F44B"
///       description: a riveting description
///       location: US
///       cloudSql:
///         instanceId: ${instance.connectionName}
///         database: ${db.name}
///         type: POSTGRES
///         credential:
///           username: ${user.name}
///           password: ${user.password}
/// ```
/// ### Bigquery Connection Full
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-database-instance
///       databaseVersion: POSTGRES_11
///       region: us-central1
///       settings:
///         tier: db-f1-micro
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
///       password: ${pwd.result}
///   connection:
///     type: gcp:bigquery:Connection
///     properties:
///       connectionId: my-connection
///       location: US
///       friendlyName: "\U0001F44B"
///       description: a riveting description
///       cloudSql:
///         instanceId: ${instance.connectionName}
///         database: ${db.name}
///         type: POSTGRES
///         credential:
///           username: ${user.name}
///           password: ${user.password}
/// ```
/// ### Bigquery Connection Aws
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let connection = connection::create(
///         "connection",
///         ConnectionArgs::builder()
///             .aws(
///                 ConnectionAws::builder()
///                     .accessRole(
///                         ConnectionAwsAccessRole::builder()
///                             .iamRoleId("arn:aws:iam::999999999999:role/omnirole")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .connection_id("my-connection")
///             .description("a riveting description")
///             .friendly_name("👋")
///             .location("aws-us-east-1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Connection Azure
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let connection = connection::create(
///         "connection",
///         ConnectionArgs::builder()
///             .azure(
///                 ConnectionAzure::builder()
///                     .customerTenantId("customer-tenant-id")
///                     .federatedApplicationClientId("b43eeeee-eeee-eeee-eeee-a480155501ce")
///                     .build_struct(),
///             )
///             .connection_id("my-connection")
///             .description("a riveting description")
///             .friendly_name("👋")
///             .location("azure-eastus2")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Connection Cloudspanner
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let connection = connection::create(
///         "connection",
///         ConnectionArgs::builder()
///             .cloud_spanner(
///                 ConnectionCloudSpanner::builder()
///                     .database("projects/project/instances/instance/databases/database")
///                     .databaseRole("database_role")
///                     .build_struct(),
///             )
///             .connection_id("my-connection")
///             .description("a riveting description")
///             .friendly_name("👋")
///             .location("US")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Connection Cloudspanner Databoost
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let connection = connection::create(
///         "connection",
///         ConnectionArgs::builder()
///             .cloud_spanner(
///                 ConnectionCloudSpanner::builder()
///                     .database("projects/project/instances/instance/databases/database")
///                     .maxParallelism(100)
///                     .useDataBoost(true)
///                     .useParallelism(true)
///                     .build_struct(),
///             )
///             .connection_id("my-connection")
///             .description("a riveting description")
///             .friendly_name("👋")
///             .location("US")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Connection Spark
///
///
/// ```yaml
/// resources:
///   connection:
///     type: gcp:bigquery:Connection
///     properties:
///       connectionId: my-connection
///       location: US
///       friendlyName: "\U0001F44B"
///       description: a riveting description
///       spark:
///         sparkHistoryServerConfig:
///           dataprocCluster: ${basic.id}
///   basic:
///     type: gcp:dataproc:Cluster
///     properties:
///       name: my-connection
///       region: us-central1
///       clusterConfig:
///         softwareConfig:
///           overrideProperties:
///             dataproc:dataproc.allow.zero.workers: 'true'
///         masterConfig:
///           numInstances: 1
///           machineType: e2-standard-2
///           diskConfig:
///             bootDiskSizeGb: 35
/// ```
/// ### Bigquery Connection Sql With Cmek
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-database-instance
///       region: us-central1
///       databaseVersion: POSTGRES_11
///       settings:
///         tier: db-f1-micro
///       deletionProtection: true
///   db:
///     type: gcp:sql:Database
///     properties:
///       instance: ${instance.name}
///       name: db
///   user:
///     type: gcp:sql:User
///     properties:
///       name: user
///       instance: ${instance.name}
///       password: tf-test-my-password_77884
///   keySaUser:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: key_sa_user
///     properties:
///       cryptoKeyId: projects/project/locations/us-central1/keyRings/us-central1/cryptoKeys/bq-key
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:${bqSa.email}
///   bq-connection-cmek:
///     type: gcp:bigquery:Connection
///     properties:
///       friendlyName: "\U0001F44B"
///       description: a riveting description
///       location: US
///       kmsKeyName: projects/project/locations/us-central1/keyRings/us-central1/cryptoKeys/bq-key
///       cloudSql:
///         instanceId: ${instance.connectionName}
///         database: ${db.name}
///         type: POSTGRES
///         credential:
///           username: ${user.name}
///           password: ${user.password}
///     options:
///       dependsOn:
///         - ${keySaUser}
/// variables:
///   bqSa:
///     fn::invoke:
///       function: gcp:bigquery:getDefaultServiceAccount
///       arguments: {}
/// ```
///
/// ## Import
///
/// Connection can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/connections/{{connection_id}}`
///
/// * `{{project}}/{{location}}/{{connection_id}}`
///
/// * `{{location}}/{{connection_id}}`
///
/// When using the `pulumi import` command, Connection can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/connection:Connection default projects/{{project}}/locations/{{location}}/connections/{{connection_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/connection:Connection default {{project}}/{{location}}/{{connection_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/connection:Connection default {{location}}/{{connection_id}}
/// ```
///
pub mod connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// Connection properties specific to Amazon Web Services.
        /// Structure is documented below.
        #[builder(into, default)]
        pub aws: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionAws>,
        >,
        /// Container for connection properties specific to Azure.
        /// Structure is documented below.
        #[builder(into, default)]
        pub azure: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionAzure>,
        >,
        /// Container for connection properties for delegation of access to GCP resources.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cloud_resource: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionCloudResource>,
        >,
        /// Connection properties specific to Cloud Spanner
        /// Structure is documented below.
        #[builder(into, default)]
        pub cloud_spanner: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionCloudSpanner>,
        >,
        /// Connection properties specific to the Cloud SQL.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cloud_sql: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionCloudSql>,
        >,
        /// Optional connection id that should be assigned to the created connection.
        #[builder(into, default)]
        pub connection_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A descriptive description for the connection
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A descriptive name for the connection
        #[builder(into, default)]
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. The Cloud KMS key that is used for encryption.
        /// Example: projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]
        #[builder(into, default)]
        pub kms_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The geographic location where the connection should reside.
        /// Cloud SQL instance must be in the same location as the connection
        /// with following exceptions: Cloud SQL us-central1 maps to BigQuery US, Cloud SQL europe-west1 maps to BigQuery EU.
        /// Examples: US, EU, asia-northeast1, us-central1, europe-west1.
        /// Spanner Connections same as spanner region
        /// AWS allowed regions are aws-us-east-1
        /// Azure allowed regions are azure-eastus2
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Container for connection properties to execute stored procedures for Apache Spark. resources.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spark: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionSpark>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// Connection properties specific to Amazon Web Services.
        /// Structure is documented below.
        pub aws: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionAws>,
        >,
        /// Container for connection properties specific to Azure.
        /// Structure is documented below.
        pub azure: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionAzure>,
        >,
        /// Container for connection properties for delegation of access to GCP resources.
        /// Structure is documented below.
        pub cloud_resource: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionCloudResource>,
        >,
        /// Connection properties specific to Cloud Spanner
        /// Structure is documented below.
        pub cloud_spanner: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionCloudSpanner>,
        >,
        /// Connection properties specific to the Cloud SQL.
        /// Structure is documented below.
        pub cloud_sql: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionCloudSql>,
        >,
        /// Optional connection id that should be assigned to the created connection.
        pub connection_id: pulumi_wasm_rust::Output<String>,
        /// A descriptive description for the connection
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A descriptive name for the connection
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// True if the connection has credential assigned.
        pub has_credential: pulumi_wasm_rust::Output<bool>,
        /// Optional. The Cloud KMS key that is used for encryption.
        /// Example: projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]
        pub kms_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The geographic location where the connection should reside.
        /// Cloud SQL instance must be in the same location as the connection
        /// with following exceptions: Cloud SQL us-central1 maps to BigQuery US, Cloud SQL europe-west1 maps to BigQuery EU.
        /// Examples: US, EU, asia-northeast1, us-central1, europe-west1.
        /// Spanner Connections same as spanner region
        /// AWS allowed regions are aws-us-east-1
        /// Azure allowed regions are azure-eastus2
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of the connection in the form of:
        /// "projects/{project_id}/locations/{location_id}/connections/{connectionId}"
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Container for connection properties to execute stored procedures for Apache Spark. resources.
        /// Structure is documented below.
        pub spark: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ConnectionSpark>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectionArgs) -> ConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_binding = args.aws.get_inner();
        let azure_binding = args.azure.get_inner();
        let cloud_resource_binding = args.cloud_resource.get_inner();
        let cloud_spanner_binding = args.cloud_spanner.get_inner();
        let cloud_sql_binding = args.cloud_sql.get_inner();
        let connection_id_binding = args.connection_id.get_inner();
        let description_binding = args.description.get_inner();
        let friendly_name_binding = args.friendly_name.get_inner();
        let kms_key_name_binding = args.kms_key_name.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let spark_binding = args.spark.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigquery/connection:Connection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aws".into(),
                    value: &aws_binding,
                },
                register_interface::ObjectField {
                    name: "azure".into(),
                    value: &azure_binding,
                },
                register_interface::ObjectField {
                    name: "cloudResource".into(),
                    value: &cloud_resource_binding,
                },
                register_interface::ObjectField {
                    name: "cloudSpanner".into(),
                    value: &cloud_spanner_binding,
                },
                register_interface::ObjectField {
                    name: "cloudSql".into(),
                    value: &cloud_sql_binding,
                },
                register_interface::ObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyName".into(),
                    value: &kms_key_name_binding,
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
                    name: "spark".into(),
                    value: &spark_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aws".into(),
                },
                register_interface::ResultField {
                    name: "azure".into(),
                },
                register_interface::ResultField {
                    name: "cloudResource".into(),
                },
                register_interface::ResultField {
                    name: "cloudSpanner".into(),
                },
                register_interface::ResultField {
                    name: "cloudSql".into(),
                },
                register_interface::ResultField {
                    name: "connectionId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "friendlyName".into(),
                },
                register_interface::ResultField {
                    name: "hasCredential".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "spark".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectionResult {
            aws: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aws").unwrap(),
            ),
            azure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azure").unwrap(),
            ),
            cloud_resource: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudResource").unwrap(),
            ),
            cloud_spanner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudSpanner").unwrap(),
            ),
            cloud_sql: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudSql").unwrap(),
            ),
            connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            friendly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("friendlyName").unwrap(),
            ),
            has_credential: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasCredential").unwrap(),
            ),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            spark: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spark").unwrap(),
            ),
        }
    }
}
