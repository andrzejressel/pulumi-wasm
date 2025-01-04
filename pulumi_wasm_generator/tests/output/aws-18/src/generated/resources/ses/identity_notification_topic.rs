/// Resource for managing SES Identity Notification Topics
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = identity_notification_topic::create(
///         "test",
///         IdentityNotificationTopicArgs::builder()
///             .identity("${example.domain}")
///             .include_original_headers(true)
///             .notification_type("Bounce")
///             .topic_arn("${exampleAwsSnsTopic.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Identity Notification Topics using the ID of the record. The ID is made up as `IDENTITY|TYPE` where `IDENTITY` is the SES Identity and `TYPE` is the Notification Type. For example:
///
/// ```sh
/// $ pulumi import aws:ses/identityNotificationTopic:IdentityNotificationTopic test 'example.com|Bounce'
/// ```
pub mod identity_notification_topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityNotificationTopicArgs {
        /// The identity for which the Amazon SNS topic will be set. You can specify an identity by using its name or by using its Amazon Resource Name (ARN).
        #[builder(into)]
        pub identity: pulumi_wasm_rust::Output<String>,
        /// Whether SES should include original email headers in SNS notifications of this type. `false` by default.
        #[builder(into, default)]
        pub include_original_headers: pulumi_wasm_rust::Output<Option<bool>>,
        /// The type of notifications that will be published to the specified Amazon SNS topic. Valid Values: `Bounce`, `Complaint` or `Delivery`.
        #[builder(into)]
        pub notification_type: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Amazon SNS topic. Can be set to `""` (an empty string) to disable publishing.
        #[builder(into, default)]
        pub topic_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IdentityNotificationTopicResult {
        /// The identity for which the Amazon SNS topic will be set. You can specify an identity by using its name or by using its Amazon Resource Name (ARN).
        pub identity: pulumi_wasm_rust::Output<String>,
        /// Whether SES should include original email headers in SNS notifications of this type. `false` by default.
        pub include_original_headers: pulumi_wasm_rust::Output<Option<bool>>,
        /// The type of notifications that will be published to the specified Amazon SNS topic. Valid Values: `Bounce`, `Complaint` or `Delivery`.
        pub notification_type: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Amazon SNS topic. Can be set to `""` (an empty string) to disable publishing.
        pub topic_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IdentityNotificationTopicArgs,
    ) -> IdentityNotificationTopicResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_inner();
        let include_original_headers_binding = args.include_original_headers.get_inner();
        let notification_type_binding = args.notification_type.get_inner();
        let topic_arn_binding = args.topic_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/identityNotificationTopic:IdentityNotificationTopic".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "includeOriginalHeaders".into(),
                    value: &include_original_headers_binding,
                },
                register_interface::ObjectField {
                    name: "notificationType".into(),
                    value: &notification_type_binding,
                },
                register_interface::ObjectField {
                    name: "topicArn".into(),
                    value: &topic_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "includeOriginalHeaders".into(),
                },
                register_interface::ResultField {
                    name: "notificationType".into(),
                },
                register_interface::ResultField {
                    name: "topicArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IdentityNotificationTopicResult {
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            include_original_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeOriginalHeaders").unwrap(),
            ),
            notification_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationType").unwrap(),
            ),
            topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topicArn").unwrap(),
            ),
        }
    }
}
