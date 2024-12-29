/// Manages selection conditions for AWS Backup plan resources.
///
/// ## Example Usage
///
/// ### IAM Role
///
/// > For more information about creating and managing IAM Roles for backups and restores, see the [AWS Backup Developer Guide](https://docs.aws.amazon.com/aws-backup/latest/devguide/iam-service-roles.html).
///
/// The below example creates an IAM role with the default managed IAM Policy for allowing AWS Backup to create backups.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["backup.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let example = role::create(
///         "example",
///         RoleArgs::builder()
///             .assume_role_policy("${assumeRole.json}")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleRolePolicyAttachment = role_policy_attachment::create(
///         "exampleRolePolicyAttachment",
///         RolePolicyAttachmentArgs::builder()
///             .policy_arn(
///                 "arn:aws:iam::aws:policy/service-role/AWSBackupServiceRolePolicyForBackup",
///             )
///             .role("${example.name}")
///             .build_struct(),
///     );
///     let exampleSelection = selection::create(
///         "exampleSelection",
///         SelectionArgs::builder().iam_role_arn("${example.arn}").build_struct(),
///     );
/// }
/// ```
///
/// ### Selecting Backups By Tag
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = selection::create(
///         "example",
///         SelectionArgs::builder()
///             .iam_role_arn("${exampleAwsIamRole.arn}")
///             .name("my_example_backup_selection")
///             .plan_id("${exampleAwsBackupPlan.id}")
///             .selection_tags(
///                 vec![
///                     SelectionSelectionTag::builder().key("foo"). type ("STRINGEQUALS")
///                     .value("bar").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Selecting Backups By Conditions
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = selection::create(
///         "example",
///         SelectionArgs::builder()
///             .conditions(
///                 vec![
///                     SelectionCondition::builder()
///                     .stringEquals(vec![SelectionConditionStringEqual::builder()
///                     .key("aws:ResourceTag/Component").value("rds").build_struct(),])
///                     .stringLikes(vec![SelectionConditionStringLike::builder()
///                     .key("aws:ResourceTag/Application").value("app*").build_struct(),])
///                     .stringNotEquals(vec![SelectionConditionStringNotEqual::builder()
///                     .key("aws:ResourceTag/Backup").value("false").build_struct(),])
///                     .stringNotLikes(vec![SelectionConditionStringNotLike::builder()
///                     .key("aws:ResourceTag/Environment").value("test*").build_struct(),])
///                     .build_struct(),
///                 ],
///             )
///             .iam_role_arn("${exampleAwsIamRole.arn}")
///             .name("my_example_backup_selection")
///             .plan_id("${exampleAwsBackupPlan.id}")
///             .resources(vec!["*",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Selecting Backups By Resource
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = selection::create(
///         "example",
///         SelectionArgs::builder()
///             .iam_role_arn("${exampleAwsIamRole.arn}")
///             .name("my_example_backup_selection")
///             .plan_id("${exampleAwsBackupPlan.id}")
///             .resources(
///                 vec![
///                     "${exampleAwsDbInstance.arn}", "${exampleAwsEbsVolume.arn}",
///                     "${exampleAwsEfsFileSystem.arn}",
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Selecting Backups By Not Resource
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = selection::create(
///         "example",
///         SelectionArgs::builder()
///             .iam_role_arn("${exampleAwsIamRole.arn}")
///             .name("my_example_backup_selection")
///             .not_resources(
///                 vec![
///                     "${exampleAwsDbInstance.arn}", "${exampleAwsEbsVolume.arn}",
///                     "${exampleAwsEfsFileSystem.arn}",
///                 ],
///             )
///             .plan_id("${exampleAwsBackupPlan.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup selection using the role plan_id and id separated by `|`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/selection:Selection example plan-id|selection-id
/// ```
pub mod selection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SelectionArgs {
        /// A list of conditions that you define to assign resources to your backup plans using tags.
        #[builder(into, default)]
        pub conditions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::backup::SelectionCondition>>,
        >,
        /// The ARN of the IAM role that AWS Backup uses to authenticate when restoring and backing up the target resource. See the [AWS Backup Developer Guide](https://docs.aws.amazon.com/aws-backup/latest/devguide/access-control.html#managed-policies) for additional information about using AWS managed policies or creating custom policies attached to the IAM role.
        #[builder(into)]
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// The display name of a resource selection document.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// An array of strings that either contain Amazon Resource Names (ARNs) or match patterns of resources to exclude from a backup plan.
        #[builder(into, default)]
        pub not_resources: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The backup plan ID to be associated with the selection of resources.
        #[builder(into)]
        pub plan_id: pulumi_wasm_rust::Output<String>,
        /// An array of strings that either contain Amazon Resource Names (ARNs) or match patterns of resources to assign to a backup plan.
        #[builder(into, default)]
        pub resources: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Tag-based conditions used to specify a set of resources to assign to a backup plan.
        #[builder(into, default)]
        pub selection_tags: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::backup::SelectionSelectionTag>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SelectionResult {
        /// A list of conditions that you define to assign resources to your backup plans using tags.
        pub conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::backup::SelectionCondition>,
        >,
        /// The ARN of the IAM role that AWS Backup uses to authenticate when restoring and backing up the target resource. See the [AWS Backup Developer Guide](https://docs.aws.amazon.com/aws-backup/latest/devguide/access-control.html#managed-policies) for additional information about using AWS managed policies or creating custom policies attached to the IAM role.
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// The display name of a resource selection document.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An array of strings that either contain Amazon Resource Names (ARNs) or match patterns of resources to exclude from a backup plan.
        pub not_resources: pulumi_wasm_rust::Output<Vec<String>>,
        /// The backup plan ID to be associated with the selection of resources.
        pub plan_id: pulumi_wasm_rust::Output<String>,
        /// An array of strings that either contain Amazon Resource Names (ARNs) or match patterns of resources to assign to a backup plan.
        pub resources: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Tag-based conditions used to specify a set of resources to assign to a backup plan.
        pub selection_tags: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::backup::SelectionSelectionTag>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SelectionArgs) -> SelectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let conditions_binding = args.conditions.get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_inner();
        let name_binding = args.name.get_inner();
        let not_resources_binding = args.not_resources.get_inner();
        let plan_id_binding = args.plan_id.get_inner();
        let resources_binding = args.resources.get_inner();
        let selection_tags_binding = args.selection_tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/selection:Selection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "conditions".into(),
                    value: &conditions_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notResources".into(),
                    value: &not_resources_binding,
                },
                register_interface::ObjectField {
                    name: "planId".into(),
                    value: &plan_id_binding,
                },
                register_interface::ObjectField {
                    name: "resources".into(),
                    value: &resources_binding,
                },
                register_interface::ObjectField {
                    name: "selectionTags".into(),
                    value: &selection_tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "conditions".into(),
                },
                register_interface::ResultField {
                    name: "iamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notResources".into(),
                },
                register_interface::ResultField {
                    name: "planId".into(),
                },
                register_interface::ResultField {
                    name: "resources".into(),
                },
                register_interface::ResultField {
                    name: "selectionTags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SelectionResult {
            conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditions").unwrap(),
            ),
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            not_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notResources").unwrap(),
            ),
            plan_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("planId").unwrap(),
            ),
            resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resources").unwrap(),
            ),
            selection_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selectionTags").unwrap(),
            ),
        }
    }
}
