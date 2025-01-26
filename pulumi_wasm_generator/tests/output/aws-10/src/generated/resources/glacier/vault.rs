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
pub mod vault {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VaultArgs {
        /// The policy document. This is a JSON formatted string.
        /// The heredoc syntax or `file` function is helpful here. Use the [Glacier Developer Guide](https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html) for more information on Glacier Vault Policy
        #[builder(into, default)]
        pub access_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Vault. Names can be between 1 and 255 characters long and the valid characters are a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), and '.' (period).
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The notifications for the Vault. Fields documented below.
        #[builder(into, default)]
        pub notification: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::glacier::VaultNotification>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VaultResult {
        /// The policy document. This is a JSON formatted string.
        /// The heredoc syntax or `file` function is helpful here. Use the [Glacier Developer Guide](https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html) for more information on Glacier Vault Policy
        pub access_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the vault.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The URI of the vault that was created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Vault. Names can be between 1 and 255 characters long and the valid characters are a-z, A-Z, 0-9, '_' (underscore), '-' (hyphen), and '.' (period).
        pub name: pulumi_wasm_rust::Output<String>,
        /// The notifications for the Vault. Fields documented below.
        pub notification: pulumi_wasm_rust::Output<
            Option<super::super::types::glacier::VaultNotification>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VaultArgs,
    ) -> VaultResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_policy_binding = args.access_policy.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let notification_binding = args.notification.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glacier/vault:Vault".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPolicy".into(),
                    value: &access_policy_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notification".into(),
                    value: &notification_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPolicy".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notification".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VaultResult {
            access_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPolicy").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notification").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
