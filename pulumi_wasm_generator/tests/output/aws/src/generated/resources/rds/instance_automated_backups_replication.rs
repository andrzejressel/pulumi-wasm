/// Manage cross-region replication of automated backups to a different AWS Region. Documentation for cross-region automated backup replication can be found at:
///
/// * [Replicating automated backups to another AWS Region](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_ReplicateBackups.html)
///
/// > **Note:** This resource has to be created in the destination region.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = instance_automated_backups_replication::create(
///         "default",
///         InstanceAutomatedBackupsReplicationArgs::builder()
///             .retention_period(14)
///             .source_db_instance_arn("arn:aws:rds:us-west-2:123456789012:db:mydatabase")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Encrypting the automated backup with KMS
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = instance_automated_backups_replication::create(
///         "default",
///         InstanceAutomatedBackupsReplicationArgs::builder()
///             .kms_key_id(
///                 "arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012",
///             )
///             .source_db_instance_arn("arn:aws:rds:us-west-2:123456789012:db:mydatabase")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example including a RDS DB instance
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = instance::create(
///         "default",
///         InstanceArgs::builder()
///             .allocated_storage(10)
///             .backup_retention_period(7)
///             .db_name("mydb")
///             .engine("postgres")
///             .engine_version("13.4")
///             .identifier("mydb")
///             .instance_class("db.t3.micro")
///             .password("mustbeeightcharacters")
///             .skip_final_snapshot(true)
///             .storage_encrypted(true)
///             .username("masterusername")
///             .build_struct(),
///     );
///     let defaultInstanceAutomatedBackupsReplication = instance_automated_backups_replication::create(
///         "defaultInstanceAutomatedBackupsReplication",
///         InstanceAutomatedBackupsReplicationArgs::builder()
///             .kms_key_id("${defaultKey.arn}")
///             .source_db_instance_arn("${default.arn}")
///             .build_struct(),
///     );
///     let defaultKey = key::create(
///         "defaultKey",
///         KeyArgs::builder()
///             .description("Encryption key for automated backups")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RDS instance automated backups replication using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/instanceAutomatedBackupsReplication:InstanceAutomatedBackupsReplication default arn:aws:rds:us-east-1:123456789012:auto-backup:ab-faaa2mgdj1vmp4xflr7yhsrmtbtob7ltrzzz2my
/// ```
pub mod instance_automated_backups_replication {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceAutomatedBackupsReplicationArgs {
        /// The AWS KMS key identifier for encryption of the replicated automated backups. The KMS key ID is the Amazon Resource Name (ARN) for the KMS encryption key in the destination AWS Region, for example, `arn:aws:kms:us-east-1:123456789012:key/AKIAIOSFODNN7EXAMPLE`.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A URL that contains a [Signature Version 4](https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html) signed request for the [`StartDBInstanceAutomatedBackupsReplication`](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_StartDBInstanceAutomatedBackupsReplication.html) action to be called in the AWS Region of the source DB instance.
        #[builder(into, default)]
        pub pre_signed_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The retention period for the replicated automated backups, defaults to `7`.
        #[builder(into, default)]
        pub retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Amazon Resource Name (ARN) of the source DB instance for the replicated automated backups, for example, `arn:aws:rds:us-west-2:123456789012:db:mydatabase`.
        #[builder(into)]
        pub source_db_instance_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceAutomatedBackupsReplicationResult {
        /// The AWS KMS key identifier for encryption of the replicated automated backups. The KMS key ID is the Amazon Resource Name (ARN) for the KMS encryption key in the destination AWS Region, for example, `arn:aws:kms:us-east-1:123456789012:key/AKIAIOSFODNN7EXAMPLE`.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// A URL that contains a [Signature Version 4](https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html) signed request for the [`StartDBInstanceAutomatedBackupsReplication`](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_StartDBInstanceAutomatedBackupsReplication.html) action to be called in the AWS Region of the source DB instance.
        pub pre_signed_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The retention period for the replicated automated backups, defaults to `7`.
        pub retention_period: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Amazon Resource Name (ARN) of the source DB instance for the replicated automated backups, for example, `arn:aws:rds:us-west-2:123456789012:db:mydatabase`.
        pub source_db_instance_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InstanceAutomatedBackupsReplicationArgs,
    ) -> InstanceAutomatedBackupsReplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let pre_signed_url_binding = args.pre_signed_url.get_inner();
        let retention_period_binding = args.retention_period.get_inner();
        let source_db_instance_arn_binding = args.source_db_instance_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/instanceAutomatedBackupsReplication:InstanceAutomatedBackupsReplication"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "preSignedUrl".into(),
                    value: &pre_signed_url_binding,
                },
                register_interface::ObjectField {
                    name: "retentionPeriod".into(),
                    value: &retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "sourceDbInstanceArn".into(),
                    value: &source_db_instance_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "preSignedUrl".into(),
                },
                register_interface::ResultField {
                    name: "retentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "sourceDbInstanceArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceAutomatedBackupsReplicationResult {
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            pre_signed_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preSignedUrl").unwrap(),
            ),
            retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPeriod").unwrap(),
            ),
            source_db_instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDbInstanceArn").unwrap(),
            ),
        }
    }
}