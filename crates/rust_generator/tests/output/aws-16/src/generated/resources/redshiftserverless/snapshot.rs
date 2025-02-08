/// Creates a new Amazon Redshift Serverless Snapshot.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = snapshot::create(
///         "example",
///         SnapshotArgs::builder()
///             .namespace_name("${exampleAwsRedshiftserverlessWorkgroup.namespaceName}")
///             .snapshot_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Serverless Snapshots using the `snapshot_name`. For example:
///
/// ```sh
/// $ pulumi import aws:redshiftserverless/snapshot:Snapshot example example
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// The namespace to create a snapshot for.
        #[builder(into)]
        pub namespace_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// How long to retain the created snapshot. Default value is `-1`.
        #[builder(into, default)]
        pub retention_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the snapshot.
        #[builder(into)]
        pub snapshot_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// All of the Amazon Web Services accounts that have access to restore a snapshot to a provisioned cluster.
        pub accounts_with_provisioned_restore_accesses: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// All of the Amazon Web Services accounts that have access to restore a snapshot to a namespace.
        pub accounts_with_restore_accesses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The username of the database within a snapshot.
        pub admin_username: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the snapshot.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the KMS key used to encrypt the snapshot.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the namespace the snapshot was created from.
        pub namespace_arn: pulumi_gestalt_rust::Output<String>,
        /// The namespace to create a snapshot for.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The owner Amazon Web Services; account of the snapshot.
        pub owner_account: pulumi_gestalt_rust::Output<String>,
        /// How long to retain the created snapshot. Default value is `-1`.
        pub retention_period: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the snapshot.
        pub snapshot_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SnapshotArgs,
    ) -> SnapshotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let namespace_name_binding = args.namespace_name.get_output(context).get_inner();
        let retention_period_binding = args
            .retention_period
            .get_output(context)
            .get_inner();
        let snapshot_name_binding = args.snapshot_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshiftserverless/snapshot:Snapshot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "namespaceName".into(),
                    value: &namespace_name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionPeriod".into(),
                    value: &retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotName".into(),
                    value: &snapshot_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SnapshotResult {
            accounts_with_provisioned_restore_accesses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountsWithProvisionedRestoreAccesses"),
            ),
            accounts_with_restore_accesses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountsWithRestoreAccesses"),
            ),
            admin_username: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("adminUsername"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            namespace_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespaceArn"),
            ),
            namespace_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespaceName"),
            ),
            owner_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerAccount"),
            ),
            retention_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retentionPeriod"),
            ),
            snapshot_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotName"),
            ),
        }
    }
}
