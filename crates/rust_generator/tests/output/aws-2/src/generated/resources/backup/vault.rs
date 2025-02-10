/// Provides an AWS Backup vault resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vault::create(
///         "example",
///         VaultArgs::builder()
///             .kms_key_arn("${exampleAwsKmsKey.arn}")
///             .name("example_backup_vault")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup vault using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/vault:Vault test-vault TestVault
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vault {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultArgs {
        /// A boolean that indicates that all recovery points stored in the vault are deleted so that the vault can be destroyed without error.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The server-side encryption key that is used to protect your backups.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the backup vault to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Metadata that you can assign to help organize the resources that you create. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VaultResult {
        /// The ARN of the vault.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A boolean that indicates that all recovery points stored in the vault are deleted so that the vault can be destroyed without error.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The server-side encryption key that is used to protect your backups.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the backup vault to create.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of recovery points that are stored in a backup vault.
        pub recovery_points: pulumi_gestalt_rust::Output<i32>,
        /// Metadata that you can assign to help organize the resources that you create. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VaultArgs,
    ) -> VaultResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let force_destroy_binding = args.force_destroy.get_output(context);
        let kms_key_arn_binding = args.kms_key_arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:backup/vault:Vault".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyArn".into(),
                    value: kms_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VaultResult {
            arn: o.get_field("arn"),
            force_destroy: o.get_field("forceDestroy"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            name: o.get_field("name"),
            recovery_points: o.get_field("recoveryPoints"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
