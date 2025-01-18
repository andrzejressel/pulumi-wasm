/// Creates a new Amazon Redshift Serverless Snapshot.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotArgs {
        /// The namespace to create a snapshot for.
        #[builder(into)]
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// How long to retain the created snapshot. Default value is `-1`.
        #[builder(into, default)]
        pub retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the snapshot.
        #[builder(into)]
        pub snapshot_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotResult {
        /// All of the Amazon Web Services accounts that have access to restore a snapshot to a provisioned cluster.
        pub accounts_with_provisioned_restore_accesses: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// All of the Amazon Web Services accounts that have access to restore a snapshot to a namespace.
        pub accounts_with_restore_accesses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The username of the database within a snapshot.
        pub admin_username: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the snapshot.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the KMS key used to encrypt the snapshot.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the namespace the snapshot was created from.
        pub namespace_arn: pulumi_wasm_rust::Output<String>,
        /// The namespace to create a snapshot for.
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// The owner Amazon Web Services; account of the snapshot.
        pub owner_account: pulumi_wasm_rust::Output<String>,
        /// How long to retain the created snapshot. Default value is `-1`.
        pub retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the snapshot.
        pub snapshot_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SnapshotArgs) -> SnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let namespace_name_binding = args.namespace_name.get_inner();
        let retention_period_binding = args.retention_period.get_inner();
        let snapshot_name_binding = args.snapshot_name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountsWithProvisionedRestoreAccesses".into(),
                },
                register_interface::ResultField {
                    name: "accountsWithRestoreAccesses".into(),
                },
                register_interface::ResultField {
                    name: "adminUsername".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "namespaceArn".into(),
                },
                register_interface::ResultField {
                    name: "namespaceName".into(),
                },
                register_interface::ResultField {
                    name: "ownerAccount".into(),
                },
                register_interface::ResultField {
                    name: "retentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "snapshotName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SnapshotResult {
            accounts_with_provisioned_restore_accesses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountsWithProvisionedRestoreAccesses").unwrap(),
            ),
            accounts_with_restore_accesses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountsWithRestoreAccesses").unwrap(),
            ),
            admin_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminUsername").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            namespace_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceArn").unwrap(),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceName").unwrap(),
            ),
            owner_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAccount").unwrap(),
            ),
            retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPeriod").unwrap(),
            ),
            snapshot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotName").unwrap(),
            ),
        }
    }
}
