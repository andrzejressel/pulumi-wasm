/// Provides an Elastic File System (EFS) Backup Policy resource.
/// Backup policies turn automatic backups on or off for an existing file system.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPolicyArgs {
        /// A backup_policy object (documented below).
        #[builder(into)]
        pub backup_policy: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::efs::BackupPolicyBackupPolicy,
        >,
        /// The ID of the EFS file system.
        #[builder(into)]
        pub file_system_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BackupPolicyResult {
        /// A backup_policy object (documented below).
        pub backup_policy: pulumi_gestalt_rust::Output<
            super::super::types::efs::BackupPolicyBackupPolicy,
        >,
        /// The ID of the EFS file system.
        pub file_system_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPolicyArgs,
    ) -> BackupPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_policy_binding = args.backup_policy.get_output(context);
        let file_system_id_binding = args.file_system_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:efs/backupPolicy:BackupPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupPolicy".into(),
                    value: &backup_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupPolicyResult {
            backup_policy: o.get_field("backupPolicy"),
            file_system_id: o.get_field("fileSystemId"),
        }
    }
}
