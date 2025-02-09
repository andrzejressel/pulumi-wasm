#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_framework {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrameworkArgs {
        /// Backup framework name.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tag key-value pair applied to those AWS resources that you want to trigger an evaluation for a rule. A maximum of one key-value pair can be provided.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFrameworkResult {
        /// ARN of the backup framework.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// One or more control blocks that make up the framework. Each control in the list has a name, input parameters, and scope. Detailed below.
        pub controls: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::backup::GetFrameworkControl>,
        >,
        /// Date and time that a framework is created, in Unix format and Coordinated Universal Time (UTC).
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// Deployment status of a framework. The statuses are: `CREATE_IN_PROGRESS` | `UPDATE_IN_PROGRESS` | `DELETE_IN_PROGRESS` | `COMPLETED`| `FAILED`.
        pub deployment_status: pulumi_gestalt_rust::Output<String>,
        /// Description of the framework.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of a parameter, for example, BackupPlanFrequency.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Framework consists of one or more controls. Each control governs a resource, such as backup plans, backup selections, backup vaults, or recovery points. You can also turn AWS Config recording on or off for each resource. The statuses are: `ACTIVE`, `PARTIALLY_ACTIVE`, `INACTIVE`, `UNAVAILABLE`. For more information refer to the [AWS documentation for Framework Status](https://docs.aws.amazon.com/aws-backup/latest/devguide/API_DescribeFramework.html#Backup-DescribeFramework-response-FrameworkStatus)
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Tag key-value pair applied to those AWS resources that you want to trigger an evaluation for a rule. A maximum of one key-value pair can be provided.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFrameworkArgs,
    ) -> GetFrameworkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:backup/getFramework:getFramework".into(),
            version: super::super::super::get_version(),
            object: &[
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
        let o = context.invoke_resource(request);
        GetFrameworkResult {
            arn: o.get_field("arn"),
            controls: o.get_field("controls"),
            creation_time: o.get_field("creationTime"),
            deployment_status: o.get_field("deploymentStatus"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
        }
    }
}
