/// Provides an Elastic File System (EFS) Backup Policy resource.
/// Backup policies turn automatic backups on or off for an existing file system.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let fs = file_system::create(
///         "fs",
///         FileSystemArgs::builder().creation_token("my-product").build_struct(),
///     );
///     let policy = backup_policy::create(
///         "policy",
///         BackupPolicyArgs::builder()
///             .backup_policy(
///                 BackupPolicyBackupPolicy::builder().status("ENABLED").build_struct(),
///             )
///             .file_system_id("${fs.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the EFS backup policies using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:efs/backupPolicy:BackupPolicy example fs-6fa144c6
/// ```
pub mod backup_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyArgs {
        /// A backup_policy object (documented below).
        #[builder(into)]
        pub backup_policy: pulumi_wasm_rust::InputOrOutput<
            super::super::types::efs::BackupPolicyBackupPolicy,
        >,
        /// The ID of the EFS file system.
        #[builder(into)]
        pub file_system_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyResult {
        /// A backup_policy object (documented below).
        pub backup_policy: pulumi_wasm_rust::Output<
            super::super::types::efs::BackupPolicyBackupPolicy,
        >,
        /// The ID of the EFS file system.
        pub file_system_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BackupPolicyArgs,
    ) -> BackupPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_policy_binding = args.backup_policy.get_output(context).get_inner();
        let file_system_id_binding = args.file_system_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:efs/backupPolicy:BackupPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupPolicy".into(),
                    value: &backup_policy_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackupPolicyResult {
            backup_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupPolicy"),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fileSystemId"),
            ),
        }
    }
}
