/// Creates a replica of an existing EFS file system in the same or another region. Creating this resource causes the source EFS file system to be replicated to a new read-only destination EFS file system (unless using the `destination.file_system_id` attribute). Deleting this resource will cause the replication from source to destination to stop and the destination file system will no longer be read only.
///
/// > **NOTE:** Deleting this resource does **not** delete the destination file system that was created.
///
/// ## Example Usage
///
/// Will create a replica using regional storage in us-west-2 that will be encrypted by the default EFS KMS key `/aws/elasticfilesystem`.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = file_system::create(
///         "example",
///         FileSystemArgs::builder().build_struct(),
///     );
///     let exampleReplicationConfiguration = replication_configuration::create(
///         "exampleReplicationConfiguration",
///         ReplicationConfigurationArgs::builder()
///             .destination(
///                 ReplicationConfigurationDestination::builder()
///                     .region("us-west-2")
///                     .build_struct(),
///             )
///             .source_file_system_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Replica will be created as One Zone storage in the us-west-2b Availability Zone and encrypted with the specified KMS key.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = file_system::create(
///         "example",
///         FileSystemArgs::builder().build_struct(),
///     );
///     let exampleReplicationConfiguration = replication_configuration::create(
///         "exampleReplicationConfiguration",
///         ReplicationConfigurationArgs::builder()
///             .destination(
///                 ReplicationConfigurationDestination::builder()
///                     .availabilityZoneName("us-west-2b")
///                     .kmsKeyId("1234abcd-12ab-34cd-56ef-1234567890ab")
///                     .build_struct(),
///             )
///             .source_file_system_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Will create a replica and set the existing file system with id `fs-1234567890` in us-west-2 as destination.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = file_system::create(
///         "example",
///         FileSystemArgs::builder().build_struct(),
///     );
///     let exampleReplicationConfiguration = replication_configuration::create(
///         "exampleReplicationConfiguration",
///         ReplicationConfigurationArgs::builder()
///             .destination(
///                 ReplicationConfigurationDestination::builder()
///                     .fileSystemId("fs-1234567890")
///                     .region("us-west-2")
///                     .build_struct(),
///             )
///             .source_file_system_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EFS Replication Configurations using the file system ID of either the source or destination file system. When importing, the `availability_zone_name` and `kms_key_id` attributes must __not__ be set in the configuration. The AWS API does not return these values when querying the replication configuration and their presence will therefore show as a diff in a subsequent plan. For example:
///
/// ```sh
/// $ pulumi import aws:efs/replicationConfiguration:ReplicationConfiguration example fs-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replication_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationConfigurationArgs {
        /// A destination configuration block (documented below).
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::efs::ReplicationConfigurationDestination,
        >,
        /// The ID of the file system that is to be replicated.
        #[builder(into)]
        pub source_file_system_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReplicationConfigurationResult {
        /// When the replication configuration was created.
        /// * `destination[0].file_system_id` - The fs ID of the replica.
        /// * `destination[0].status` - The status of the replication.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// A destination configuration block (documented below).
        pub destination: pulumi_gestalt_rust::Output<
            super::super::types::efs::ReplicationConfigurationDestination,
        >,
        /// The Amazon Resource Name (ARN) of the original source Amazon EFS file system in the replication configuration.
        pub original_source_file_system_arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the current source file system in the replication configuration.
        pub source_file_system_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the file system that is to be replicated.
        pub source_file_system_id: pulumi_gestalt_rust::Output<String>,
        /// The AWS Region in which the source Amazon EFS file system is located.
        pub source_file_system_region: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReplicationConfigurationArgs,
    ) -> ReplicationConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_binding = args.destination.get_output(context);
        let source_file_system_id_binding = args
            .source_file_system_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:efs/replicationConfiguration:ReplicationConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: destination_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceFileSystemId".into(),
                    value: source_file_system_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReplicationConfigurationResult {
            creation_time: o.get_field("creationTime"),
            destination: o.get_field("destination"),
            original_source_file_system_arn: o.get_field("originalSourceFileSystemArn"),
            source_file_system_arn: o.get_field("sourceFileSystemArn"),
            source_file_system_id: o.get_field("sourceFileSystemId"),
            source_file_system_region: o.get_field("sourceFileSystemRegion"),
        }
    }
}
