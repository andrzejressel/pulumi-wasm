/// Provides a way to set SNS SMS preferences.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod sms_preferences {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SmsPreferencesArgs {
        /// A string, such as your business brand, that is displayed as the sender on the receiving device.
        #[builder(into, default)]
        pub default_sender_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of SMS message that you will send by default. Possible values are: Promotional, Transactional
        #[builder(into, default)]
        pub default_sms_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs.
        #[builder(into, default)]
        pub delivery_status_iam_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value must be between 0 and 100.
        #[builder(into, default)]
        pub delivery_status_success_sampling_rate: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The maximum amount in USD that you are willing to spend each month to send SMS messages.
        #[builder(into, default)]
        pub monthly_spend_limit: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the Amazon S3 bucket to receive daily SMS usage reports from Amazon SNS.
        #[builder(into, default)]
        pub usage_report_s3_bucket: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SmsPreferencesResult {
        /// A string, such as your business brand, that is displayed as the sender on the receiving device.
        pub default_sender_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of SMS message that you will send by default. Possible values are: Promotional, Transactional
        pub default_sms_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the IAM role that allows Amazon SNS to write logs about SMS deliveries in CloudWatch Logs.
        pub delivery_status_iam_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The percentage of successful SMS deliveries for which Amazon SNS will write logs in CloudWatch Logs. The value must be between 0 and 100.
        pub delivery_status_success_sampling_rate: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The maximum amount in USD that you are willing to spend each month to send SMS messages.
        pub monthly_spend_limit: pulumi_wasm_rust::Output<i32>,
        /// The name of the Amazon S3 bucket to receive daily SMS usage reports from Amazon SNS.
        pub usage_report_s3_bucket: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SmsPreferencesArgs) -> SmsPreferencesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_sender_id_binding = args.default_sender_id.get_inner();
        let default_sms_type_binding = args.default_sms_type.get_inner();
        let delivery_status_iam_role_arn_binding = args
            .delivery_status_iam_role_arn
            .get_inner();
        let delivery_status_success_sampling_rate_binding = args
            .delivery_status_success_sampling_rate
            .get_inner();
        let monthly_spend_limit_binding = args.monthly_spend_limit.get_inner();
        let usage_report_s3_bucket_binding = args.usage_report_s3_bucket.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sns/smsPreferences:SmsPreferences".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultSenderId".into(),
                    value: &default_sender_id_binding,
                },
                register_interface::ObjectField {
                    name: "defaultSmsType".into(),
                    value: &default_sms_type_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryStatusIamRoleArn".into(),
                    value: &delivery_status_iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryStatusSuccessSamplingRate".into(),
                    value: &delivery_status_success_sampling_rate_binding,
                },
                register_interface::ObjectField {
                    name: "monthlySpendLimit".into(),
                    value: &monthly_spend_limit_binding,
                },
                register_interface::ObjectField {
                    name: "usageReportS3Bucket".into(),
                    value: &usage_report_s3_bucket_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultSenderId".into(),
                },
                register_interface::ResultField {
                    name: "defaultSmsType".into(),
                },
                register_interface::ResultField {
                    name: "deliveryStatusIamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "deliveryStatusSuccessSamplingRate".into(),
                },
                register_interface::ResultField {
                    name: "monthlySpendLimit".into(),
                },
                register_interface::ResultField {
                    name: "usageReportS3Bucket".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SmsPreferencesResult {
            default_sender_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSenderId").unwrap(),
            ),
            default_sms_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSmsType").unwrap(),
            ),
            delivery_status_iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryStatusIamRoleArn").unwrap(),
            ),
            delivery_status_success_sampling_rate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryStatusSuccessSamplingRate").unwrap(),
            ),
            monthly_spend_limit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monthlySpendLimit").unwrap(),
            ),
            usage_report_s3_bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usageReportS3Bucket").unwrap(),
            ),
        }
    }
}