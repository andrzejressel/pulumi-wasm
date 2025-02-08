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
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       name: example
///       assumeRolePolicy: ${assumeRole.json}
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       policyArn: arn:aws:iam::aws:policy/service-role/AWSBackupServiceRolePolicyForBackup
///       role: ${example.name}
///   exampleSelection:
///     type: aws:backup:Selection
///     name: example
///     properties:
///       iamRoleArn: ${example.arn}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - backup.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Selecting Backups By Tag
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod selection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SelectionArgs {
        /// A list of conditions that you define to assign resources to your backup plans using tags.
        #[builder(into, default)]
        pub conditions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::backup::SelectionCondition>>,
        >,
        /// The ARN of the IAM role that AWS Backup uses to authenticate when restoring and backing up the target resource. See the [AWS Backup Developer Guide](https://docs.aws.amazon.com/aws-backup/latest/devguide/access-control.html#managed-policies) for additional information about using AWS managed policies or creating custom policies attached to the IAM role.
        #[builder(into)]
        pub iam_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The display name of a resource selection document.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An array of strings that either contain Amazon Resource Names (ARNs) or match patterns of resources to exclude from a backup plan.
        #[builder(into, default)]
        pub not_resources: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The backup plan ID to be associated with the selection of resources.
        #[builder(into)]
        pub plan_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An array of strings that either contain Amazon Resource Names (ARNs) or match patterns of resources to assign to a backup plan.
        #[builder(into, default)]
        pub resources: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Tag-based conditions used to specify a set of resources to assign to a backup plan.
        #[builder(into, default)]
        pub selection_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::backup::SelectionSelectionTag>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SelectionResult {
        /// A list of conditions that you define to assign resources to your backup plans using tags.
        pub conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::backup::SelectionCondition>,
        >,
        /// The ARN of the IAM role that AWS Backup uses to authenticate when restoring and backing up the target resource. See the [AWS Backup Developer Guide](https://docs.aws.amazon.com/aws-backup/latest/devguide/access-control.html#managed-policies) for additional information about using AWS managed policies or creating custom policies attached to the IAM role.
        pub iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// The display name of a resource selection document.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An array of strings that either contain Amazon Resource Names (ARNs) or match patterns of resources to exclude from a backup plan.
        pub not_resources: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The backup plan ID to be associated with the selection of resources.
        pub plan_id: pulumi_gestalt_rust::Output<String>,
        /// An array of strings that either contain Amazon Resource Names (ARNs) or match patterns of resources to assign to a backup plan.
        pub resources: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Tag-based conditions used to specify a set of resources to assign to a backup plan.
        pub selection_tags: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::backup::SelectionSelectionTag>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SelectionArgs,
    ) -> SelectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let conditions_binding = args.conditions.get_output(context).get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let not_resources_binding = args.not_resources.get_output(context).get_inner();
        let plan_id_binding = args.plan_id.get_output(context).get_inner();
        let resources_binding = args.resources.get_output(context).get_inner();
        let selection_tags_binding = args.selection_tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/selection:Selection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        SelectionResult {
            conditions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("conditions"),
            ),
            iam_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamRoleArn"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            not_resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notResources"),
            ),
            plan_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("planId"),
            ),
            resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resources"),
            ),
            selection_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selectionTags"),
            ),
        }
    }
}
