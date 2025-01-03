/// Manages an DocumentDB Global Cluster. A global cluster consists of one primary region and up to five read-only secondary regions. You issue write operations directly to the primary cluster in the primary region and Amazon DocumentDB automatically replicates the data to the secondary regions using dedicated infrastructure.
///
/// More information about DocumentDB Global Clusters can be found in the [DocumentDB Developer Guide](https://docs.aws.amazon.com/documentdb/latest/developerguide/global-clusters.html).
///
/// ## Example Usage
///
/// ### New DocumentDB Global Cluster
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = global_cluster::create(
///         "example",
///         GlobalClusterArgs::builder()
///             .engine("docdb")
///             .engine_version("4.0.0")
///             .global_cluster_identifier("global-test")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .cluster_identifier("test-primary-cluster")
///             .db_subnet_group_name("default")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .global_cluster_identifier("${example.id}")
///             .master_password("somepass123")
///             .master_username("username")
///             .build_struct(),
///     );
///     let primaryClusterInstance = cluster_instance::create(
///         "primaryClusterInstance",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${primary.id}")
///             .engine("${example.engine}")
///             .identifier("test-primary-cluster-instance")
///             .instance_class("db.r5.large")
///             .build_struct(),
///     );
///     let secondary = cluster::create(
///         "secondary",
///         ClusterArgs::builder()
///             .cluster_identifier("test-secondary-cluster")
///             .db_subnet_group_name("default")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .global_cluster_identifier("${example.id}")
///             .build_struct(),
///     );
///     let secondaryClusterInstance = cluster_instance::create(
///         "secondaryClusterInstance",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${secondary.id}")
///             .engine("${example.engine}")
///             .identifier("test-secondary-cluster-instance")
///             .instance_class("db.r5.large")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### New Global Cluster From Existing DB Cluster
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create("example", ClusterArgs::builder().build_struct());
///     let exampleGlobalCluster = global_cluster::create(
///         "exampleGlobalCluster",
///         GlobalClusterArgs::builder()
///             .global_cluster_identifier("example")
///             .source_db_cluster_identifier("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_docdb_global_cluster` using the Global Cluster identifier. For example:
///
/// ```sh
/// $ pulumi import aws:docdb/globalCluster:GlobalCluster example example
/// ```
/// Certain resource arguments, like `source_db_cluster_identifier`, do not have an API method for reading the information after creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
pub mod global_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalClusterArgs {
        /// Name for an automatically created database on cluster creation.
        #[builder(into, default)]
        pub database_name: pulumi_wasm_rust::Output<Option<String>>,
        /// If the Global Cluster should have deletion protection enabled. The database can't be deleted when this value is set to `true`. The default is `false`.
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the database engine to be used for this DB cluster. The provider will only perform drift detection if a configuration value is provided. Current Valid values: `docdb`. Defaults to `docdb`. Conflicts with `source_db_cluster_identifier`.
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// Engine version of the global database. Upgrading the engine version will result in all cluster members being immediately updated and will.
        /// * **NOTE:** Upgrading major versions is not supported.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The global cluster identifier.
        #[builder(into)]
        pub global_cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) to use as the primary DB Cluster of the Global Cluster on creation. The provider cannot perform drift detection of this value.
        #[builder(into, default)]
        pub source_db_cluster_identifier: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the DB cluster is encrypted. The default is `false` unless `source_db_cluster_identifier` is specified and encrypted. The provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub storage_encrypted: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GlobalClusterResult {
        /// Global Cluster Amazon Resource Name (ARN)
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name for an automatically created database on cluster creation.
        pub database_name: pulumi_wasm_rust::Output<Option<String>>,
        /// If the Global Cluster should have deletion protection enabled. The database can't be deleted when this value is set to `true`. The default is `false`.
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the database engine to be used for this DB cluster. The provider will only perform drift detection if a configuration value is provided. Current Valid values: `docdb`. Defaults to `docdb`. Conflicts with `source_db_cluster_identifier`.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Engine version of the global database. Upgrading the engine version will result in all cluster members being immediately updated and will.
        /// * **NOTE:** Upgrading major versions is not supported.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The global cluster identifier.
        pub global_cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// Set of objects containing Global Cluster members.
        pub global_cluster_members: pulumi_wasm_rust::Output<
            Vec<super::super::types::docdb::GlobalClusterGlobalClusterMember>,
        >,
        /// AWS Region-unique, immutable identifier for the global database cluster. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB cluster is accessed.
        pub global_cluster_resource_id: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) to use as the primary DB Cluster of the Global Cluster on creation. The provider cannot perform drift detection of this value.
        pub source_db_cluster_identifier: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the DB cluster is encrypted. The default is `false` unless `source_db_cluster_identifier` is specified and encrypted. The provider will only perform drift detection if a configuration value is provided.
        pub storage_encrypted: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GlobalClusterArgs) -> GlobalClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let database_name_binding = args.database_name.get_inner();
        let deletion_protection_binding = args.deletion_protection.get_inner();
        let engine_binding = args.engine.get_inner();
        let engine_version_binding = args.engine_version.get_inner();
        let global_cluster_identifier_binding = args
            .global_cluster_identifier
            .get_inner();
        let source_db_cluster_identifier_binding = args
            .source_db_cluster_identifier
            .get_inner();
        let storage_encrypted_binding = args.storage_encrypted.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:docdb/globalCluster:GlobalCluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "globalClusterIdentifier".into(),
                    value: &global_cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "sourceDbClusterIdentifier".into(),
                    value: &source_db_cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "storageEncrypted".into(),
                    value: &storage_encrypted_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "globalClusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "globalClusterMembers".into(),
                },
                register_interface::ResultField {
                    name: "globalClusterResourceId".into(),
                },
                register_interface::ResultField {
                    name: "sourceDbClusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "storageEncrypted".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GlobalClusterResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            global_cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalClusterIdentifier").unwrap(),
            ),
            global_cluster_members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalClusterMembers").unwrap(),
            ),
            global_cluster_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalClusterResourceId").unwrap(),
            ),
            source_db_cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDbClusterIdentifier").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            storage_encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEncrypted").unwrap(),
            ),
        }
    }
}
