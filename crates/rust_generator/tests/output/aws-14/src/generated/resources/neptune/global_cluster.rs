/// Manages a Neptune Global Cluster. A global cluster consists of one primary region and up to five read-only secondary regions. You issue write operations directly to the primary cluster in the primary region and Amazon Neptune automatically replicates the data to the secondary regions using dedicated infrastructure.
///
/// More information about Neptune Global Clusters can be found in the [Neptune User Guide](https://docs.aws.amazon.com/neptune/latest/userguide/neptune-global-database.html).
///
/// ## Example Usage
///
/// ### New Neptune Global Cluster
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = global_cluster::create(
///         "example",
///         GlobalClusterArgs::builder()
///             .engine("neptune")
///             .engine_version("1.2.0.0")
///             .global_cluster_identifier("global-test")
///             .build_struct(),
///     );
///     let primary = cluster::create(
///         "primary",
///         ClusterArgs::builder()
///             .cluster_identifier("test-primary-cluster")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .global_cluster_identifier("${example.id}")
///             .neptune_subnet_group_name("default")
///             .build_struct(),
///     );
///     let primaryClusterInstance = cluster_instance::create(
///         "primaryClusterInstance",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${primary.id}")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .identifier("test-primary-cluster-instance")
///             .instance_class("db.r5.large")
///             .neptune_subnet_group_name("default")
///             .build_struct(),
///     );
///     let secondary = cluster::create(
///         "secondary",
///         ClusterArgs::builder()
///             .cluster_identifier("test-secondary-cluster")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .global_cluster_identifier("${example.id}")
///             .neptune_subnet_group_name("default")
///             .build_struct(),
///     );
///     let secondaryClusterInstance = cluster_instance::create(
///         "secondaryClusterInstance",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${secondary.id}")
///             .engine("${example.engine}")
///             .engine_version("${example.engineVersion}")
///             .identifier("test-secondary-cluster-instance")
///             .instance_class("db.r5.large")
///             .neptune_subnet_group_name("default")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### New Global Cluster From Existing DB Cluster
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// Using `pulumi import`, import `aws_neptune_global_cluster` using the Global Cluster identifier. For example:
///
/// ```sh
/// $ pulumi import aws:neptune/globalCluster:GlobalCluster example example
/// ```
/// Certain resource arguments, like `source_db_cluster_identifier`, do not have an API method for reading the information after creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod global_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalClusterArgs {
        /// If the Global Cluster should have deletion protection enabled. The database can't be deleted when this value is set to `true`. The default is `false`.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the database engine to be used for this DB cluster. The provider will only perform drift detection if a configuration value is provided. Current Valid values: `neptune`. Conflicts with `source_db_cluster_identifier`.
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Engine version of the global database. Upgrading the engine version will result in all cluster members being immediately updated and will.
        /// * **NOTE:** Upgrading major versions is not supported.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The global cluster identifier.
        #[builder(into)]
        pub global_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amazon Resource Name (ARN) to use as the primary DB Cluster of the Global Cluster on creation. The provider cannot perform drift detection of this value.
        #[builder(into, default)]
        pub source_db_cluster_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies whether the DB cluster is encrypted. The default is `false` unless `source_db_cluster_identifier` is specified and encrypted. The provider will only perform drift detection if a configuration value is provided.
        #[builder(into, default)]
        pub storage_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GlobalClusterResult {
        /// Global Cluster Amazon Resource Name (ARN)
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// If the Global Cluster should have deletion protection enabled. The database can't be deleted when this value is set to `true`. The default is `false`.
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the database engine to be used for this DB cluster. The provider will only perform drift detection if a configuration value is provided. Current Valid values: `neptune`. Conflicts with `source_db_cluster_identifier`.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Engine version of the global database. Upgrading the engine version will result in all cluster members being immediately updated and will.
        /// * **NOTE:** Upgrading major versions is not supported.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The global cluster identifier.
        pub global_cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// Set of objects containing Global Cluster members.
        pub global_cluster_members: pulumi_gestalt_rust::Output<
            Vec<super::super::types::neptune::GlobalClusterGlobalClusterMember>,
        >,
        /// AWS Region-unique, immutable identifier for the global database cluster. This identifier is found in AWS CloudTrail log entries whenever the AWS KMS key for the DB cluster is accessed.
        pub global_cluster_resource_id: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) to use as the primary DB Cluster of the Global Cluster on creation. The provider cannot perform drift detection of this value.
        pub source_db_cluster_identifier: pulumi_gestalt_rust::Output<String>,
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the DB cluster is encrypted. The default is `false` unless `source_db_cluster_identifier` is specified and encrypted. The provider will only perform drift detection if a configuration value is provided.
        pub storage_encrypted: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlobalClusterArgs,
    ) -> GlobalClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let engine_binding = args.engine.get_output(context);
        let engine_version_binding = args.engine_version.get_output(context);
        let global_cluster_identifier_binding = args
            .global_cluster_identifier
            .get_output(context);
        let source_db_cluster_identifier_binding = args
            .source_db_cluster_identifier
            .get_output(context);
        let storage_encrypted_binding = args.storage_encrypted.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:neptune/globalCluster:GlobalCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: deletion_protection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: engine_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineVersion".into(),
                    value: engine_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalClusterIdentifier".into(),
                    value: global_cluster_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceDbClusterIdentifier".into(),
                    value: source_db_cluster_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageEncrypted".into(),
                    value: storage_encrypted_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GlobalClusterResult {
            arn: o.get_field("arn"),
            deletion_protection: o.get_field("deletionProtection"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            global_cluster_identifier: o.get_field("globalClusterIdentifier"),
            global_cluster_members: o.get_field("globalClusterMembers"),
            global_cluster_resource_id: o.get_field("globalClusterResourceId"),
            source_db_cluster_identifier: o.get_field("sourceDbClusterIdentifier"),
            status: o.get_field("status"),
            storage_encrypted: o.get_field("storageEncrypted"),
        }
    }
}
