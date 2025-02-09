/// Provides a Log subscription for AWS Directory Service that pushes logs to cloudwatch.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       name: /aws/directoryservice/${exampleAwsDirectoryServiceDirectory.id}
///       retentionInDays: 14
///   ad-log-policyLogResourcePolicy:
///     type: aws:cloudwatch:LogResourcePolicy
///     name: ad-log-policy
///     properties:
///       policyDocument: ${["ad-log-policy"].json}
///       policyName: ad-log-policy
///   exampleLogService:
///     type: aws:directoryservice:LogService
///     name: example
///     properties:
///       directoryId: ${exampleAwsDirectoryServiceDirectory.id}
///       logGroupName: ${example.name}
/// variables:
///   ad-log-policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - logs:CreateLogStream
///               - logs:PutLogEvents
///             principals:
///               - identifiers:
///                   - ds.amazonaws.com
///                 type: Service
///             resources:
///               - ${example.arn}:*
///             effect: Allow
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Directory Service Log Subscriptions using the directory id. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/logService:LogService msad d-1234567890
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogServiceArgs {
        /// ID of directory.
        #[builder(into)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the cloudwatch log group to which the logs should be published. The log group should be already created and the directory service principal should be provided with required permission to create stream and publish logs. Changing this value would delete the current subscription and create a new one. A directory can only have one log subscription at a time.
        #[builder(into)]
        pub log_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LogServiceResult {
        /// ID of directory.
        pub directory_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the cloudwatch log group to which the logs should be published. The log group should be already created and the directory service principal should be provided with required permission to create stream and publish logs. Changing this value would delete the current subscription and create a new one. A directory can only have one log subscription at a time.
        pub log_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogServiceArgs,
    ) -> LogServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let directory_id_binding = args.directory_id.get_output(context);
        let log_group_name_binding = args.log_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directoryservice/logService:LogService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: directory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupName".into(),
                    value: log_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogServiceResult {
            directory_id: o.get_field("directoryId"),
            log_group_name: o.get_field("logGroupName"),
        }
    }
}
