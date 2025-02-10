/// Provides an AWS Backup Framework resource.
///
/// > **Note:** For the Deployment Status of the Framework to be successful, please turn on resource tracking to enable AWS Config recording to track configuration changes of your backup resources. This can be done from the AWS Console.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:backup:Framework
///     name: Example
///     properties:
///       name: exampleFramework
///       description: this is an example framework
///       controls:
///         - name: BACKUP_RECOVERY_POINT_MINIMUM_RETENTION_CHECK
///           inputParameters:
///             - name: requiredRetentionDays
///               value: '35'
///         - name: BACKUP_PLAN_MIN_FREQUENCY_AND_MIN_RETENTION_CHECK
///           inputParameters:
///             - name: requiredFrequencyUnit
///               value: hours
///             - name: requiredRetentionDays
///               value: '35'
///             - name: requiredFrequencyValue
///               value: '1'
///         - name: BACKUP_RECOVERY_POINT_ENCRYPTED
///         - name: BACKUP_RESOURCES_PROTECTED_BY_BACKUP_PLAN
///           scope:
///             complianceResourceTypes:
///               - EBS
///         - name: BACKUP_RECOVERY_POINT_MANUAL_DELETION_DISABLED
///         - name: BACKUP_RESOURCES_PROTECTED_BY_BACKUP_VAULT_LOCK
///           inputParameters:
///             - name: maxRetentionDays
///               value: '100'
///             - name: minRetentionDays
///               value: '1'
///           scope:
///             complianceResourceTypes:
///               - EBS
///         - name: BACKUP_LAST_RECOVERY_POINT_CREATED
///           inputParameters:
///             - name: recoveryPointAgeUnit
///               value: days
///             - name: recoveryPointAgeValue
///               value: '1'
///           scope:
///             complianceResourceTypes:
///               - EBS
///       tags:
///         Name: Example Framework
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup Framework using the `id` which corresponds to the name of the Backup Framework. For example:
///
/// ```sh
/// $ pulumi import aws:backup/framework:Framework test <id>
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod framework {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrameworkArgs {
        /// One or more control blocks that make up the framework. Each control in the list has a name, input parameters, and scope. Detailed below.
        #[builder(into)]
        pub controls: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::backup::FrameworkControl>,
        >,
        /// The description of the framework with a maximum of 1,024 characters
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unique name of the framework. The name must be between 1 and 256 characters, starting with a letter, and consisting of letters, numbers, and underscores.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Metadata that you can assign to help organize the frameworks you create. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FrameworkResult {
        /// The ARN of the backup framework.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// One or more control blocks that make up the framework. Each control in the list has a name, input parameters, and scope. Detailed below.
        pub controls: pulumi_gestalt_rust::Output<
            Vec<super::super::types::backup::FrameworkControl>,
        >,
        /// The date and time that a framework is created, in Unix format and Coordinated Universal Time (UTC).
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// The deployment status of a framework. The statuses are: `CREATE_IN_PROGRESS` | `UPDATE_IN_PROGRESS` | `DELETE_IN_PROGRESS` | `COMPLETED` | `FAILED`.
        pub deployment_status: pulumi_gestalt_rust::Output<String>,
        /// The description of the framework with a maximum of 1,024 characters
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique name of the framework. The name must be between 1 and 256 characters, starting with a letter, and consisting of letters, numbers, and underscores.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A framework consists of one or more controls. Each control governs a resource, such as backup plans, backup selections, backup vaults, or recovery points. You can also turn AWS Config recording on or off for each resource. For more information refer to the [AWS documentation for Framework Status](https://docs.aws.amazon.com/aws-backup/latest/devguide/API_DescribeFramework.html#Backup-DescribeFramework-response-FrameworkStatus)
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Metadata that you can assign to help organize the frameworks you create. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: FrameworkArgs,
    ) -> FrameworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let controls_binding = args.controls.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:backup/framework:Framework".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controls".into(),
                    value: controls_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
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
        FrameworkResult {
            arn: o.get_field("arn"),
            controls: o.get_field("controls"),
            creation_time: o.get_field("creationTime"),
            deployment_status: o.get_field("deploymentStatus"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
