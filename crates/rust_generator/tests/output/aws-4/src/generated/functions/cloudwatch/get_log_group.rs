#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_log_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLogGroupArgs {
        /// Name of the Cloudwatch log group
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetLogGroupResult {
        /// ARN of the Cloudwatch log group. Any `:*` suffix added by the API, denoting all CloudWatch Log Streams under the CloudWatch Log Group, is removed for greater compatibility with other AWS services that do not accept the suffix.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Creation time of the log group, expressed as the number of milliseconds after Jan 1, 1970 00:00:00 UTC.
        pub creation_time: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the KMS Key to use when encrypting log data.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The log class of the log group.
        pub log_group_class: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Number of days log events retained in the specified log group.
        pub retention_in_days: pulumi_gestalt_rust::Output<i32>,
        /// Map of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLogGroupArgs,
    ) -> GetLogGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudwatch/getLogGroup:getLogGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLogGroupResult {
            arn: o.get_field("arn"),
            creation_time: o.get_field("creationTime"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            log_group_class: o.get_field("logGroupClass"),
            name: o.get_field("name"),
            retention_in_days: o.get_field("retentionInDays"),
            tags: o.get_field("tags"),
        }
    }
}
