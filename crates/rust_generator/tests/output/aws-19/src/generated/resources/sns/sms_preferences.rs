/// Provides a way to set SNS SMS preferences.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let updateSmsPrefs = sms_preferences::create(
///         "updateSmsPrefs",
///         SmsPreferencesArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// You cannot import the SMS preferences.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sms_preferences {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmsPreferencesArgs {
        /// A string, such as your business brand, that is displayed as the sender on the receiving device.
        #[builder(into, default)]
        pub default_sender_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of SMS message that you will send by default. Possible values are: Promotional, Transactional
        #[builder(into, default)]
        pub default_sms_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs.
        #[builder(into, default)]
        pub delivery_status_iam_role_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value must be between 0 and 100.
        #[builder(into, default)]
        pub delivery_status_success_sampling_rate: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The maximum amount in USD that you are willing to spend each month to send SMS messages.
        #[builder(into, default)]
        pub monthly_spend_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the Amazon S3 bucket to receive daily SMS usage reports from Amazon SNS.
        #[builder(into, default)]
        pub usage_report_s3_bucket: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SmsPreferencesResult {
        /// A string, such as your business brand, that is displayed as the sender on the receiving device.
        pub default_sender_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of SMS message that you will send by default. Possible values are: Promotional, Transactional
        pub default_sms_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs.
        pub delivery_status_iam_role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value must be between 0 and 100.
        pub delivery_status_success_sampling_rate: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The maximum amount in USD that you are willing to spend each month to send SMS messages.
        pub monthly_spend_limit: pulumi_gestalt_rust::Output<i32>,
        /// The name of the Amazon S3 bucket to receive daily SMS usage reports from Amazon SNS.
        pub usage_report_s3_bucket: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SmsPreferencesArgs,
    ) -> SmsPreferencesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_sender_id_binding = args.default_sender_id.get_output(context);
        let default_sms_type_binding = args.default_sms_type.get_output(context);
        let delivery_status_iam_role_arn_binding = args
            .delivery_status_iam_role_arn
            .get_output(context);
        let delivery_status_success_sampling_rate_binding = args
            .delivery_status_success_sampling_rate
            .get_output(context);
        let monthly_spend_limit_binding = args.monthly_spend_limit.get_output(context);
        let usage_report_s3_bucket_binding = args
            .usage_report_s3_bucket
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sns/smsPreferences:SmsPreferences".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultSenderId".into(),
                    value: default_sender_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultSmsType".into(),
                    value: default_sms_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryStatusIamRoleArn".into(),
                    value: delivery_status_iam_role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryStatusSuccessSamplingRate".into(),
                    value: delivery_status_success_sampling_rate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monthlySpendLimit".into(),
                    value: monthly_spend_limit_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "usageReportS3Bucket".into(),
                    value: usage_report_s3_bucket_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SmsPreferencesResult {
            default_sender_id: o.get_field("defaultSenderId"),
            default_sms_type: o.get_field("defaultSmsType"),
            delivery_status_iam_role_arn: o.get_field("deliveryStatusIamRoleArn"),
            delivery_status_success_sampling_rate: o
                .get_field("deliveryStatusSuccessSamplingRate"),
            monthly_spend_limit: o.get_field("monthlySpendLimit"),
            usage_report_s3_bucket: o.get_field("usageReportS3Bucket"),
        }
    }
}
