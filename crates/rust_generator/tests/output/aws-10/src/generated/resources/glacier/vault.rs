/// Provides a Glacier Vault Resource. You can refer to the [Glacier Developer Guide](https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-vaults.html) for a full explanation of the Glacier Vault functionality
///
/// > **NOTE:** When removing a Glacier Vault, the Vault must be empty.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   awsSnsTopic:
///     type: aws:sns:Topic
///     name: aws_sns_topic
///     properties:
///       name: glacier-sns-topic
///   myArchiveVault:
///     type: aws:glacier:Vault
///     name: my_archive
///     properties:
///       name: MyArchive
///       notification:
///         snsTopic: ${awsSnsTopic.arn}
///         events:
///           - ArchiveRetrievalCompleted
///           - InventoryRetrievalCompleted
///       accessPolicy: ${myArchive.json}
///       tags:
///         Test: MyArchive
/// variables:
///   myArchive:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: add-read-only-perm
///             effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - glacier:InitiateJob
///               - glacier:GetJobOutput
///             resources:
///               - arn:aws:glacier:eu-west-1:432981146916:vaults/MyArchive
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glacier Vaults using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glacier/vault:Vault archive my_archive
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vault {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultArgs {
        /// The policy document. This is a JSON formatted string.
        /// The heredoc syntax or `file` function is helpful here. Use the [Glacier Developer Guide](https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html) for more information on Glacier Vault Policy
        #[builder(into, default)]
        pub access_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Vault. Names can be between 1 and 255 characters long and the valid characters are a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), and '.' (period).
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The notifications for the Vault. Fields documented below.
        #[builder(into, default)]
        pub notification: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glacier::VaultNotification>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VaultResult {
        /// The policy document. This is a JSON formatted string.
        /// The heredoc syntax or `file` function is helpful here. Use the [Glacier Developer Guide](https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html) for more information on Glacier Vault Policy
        pub access_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the vault.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The URI of the vault that was created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Vault. Names can be between 1 and 255 characters long and the valid characters are a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), and '.' (period).
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The notifications for the Vault. Fields documented below.
        pub notification: pulumi_gestalt_rust::Output<
            Option<super::super::types::glacier::VaultNotification>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_policy_binding = args.access_policy.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_binding = args.notification.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glacier/vault:Vault".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessPolicy".into(),
                    value: access_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notification".into(),
                    value: notification_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VaultResult {
            access_policy: o.get_field("accessPolicy"),
            arn: o.get_field("arn"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            notification: o.get_field("notification"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
