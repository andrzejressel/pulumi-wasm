/// Provides a resource to manage an [Amazon Detective Member](https://docs.aws.amazon.com/detective/latest/APIReference/API_CreateMembers.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = graph::create("example", GraphArgs::builder().build_struct());
///     let exampleMember = member::create(
///         "exampleMember",
///         MemberArgs::builder()
///             .account_id("AWS ACCOUNT ID")
///             .disable_email_notification(true)
///             .email_address("EMAIL")
///             .graph_arn("${example.id}")
///             .message("Message of the invitation")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_detective_member` using the ARN of the graph followed by the account ID of the member account. For example:
///
/// ```sh
/// $ pulumi import aws:detective/member:Member example arn:aws:detective:us-east-1:123456789101:graph:231684d34gh74g4bae1dbc7bd807d02d/123456789012
/// ```
pub mod member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MemberArgs {
        /// AWS account ID for the account.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// If set to true, then the root user of the invited account will _not_ receive an email notification. This notification is in addition to an alert that the root user receives in AWS Personal Health Dashboard. By default, this is set to `false`.
        #[builder(into, default)]
        pub disable_email_notification: pulumi_wasm_rust::Output<Option<bool>>,
        /// Email address for the account.
        #[builder(into)]
        pub email_address: pulumi_wasm_rust::Output<String>,
        /// ARN of the behavior graph to invite the member accounts to contribute their data to.
        #[builder(into)]
        pub graph_arn: pulumi_wasm_rust::Output<String>,
        /// A custom message to include in the invitation. Amazon Detective adds this message to the standard content that it sends for an invitation.
        #[builder(into, default)]
        pub message: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MemberResult {
        /// AWS account ID for the account.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// AWS account ID for the administrator account.
        pub administrator_id: pulumi_wasm_rust::Output<String>,
        /// If set to true, then the root user of the invited account will _not_ receive an email notification. This notification is in addition to an alert that the root user receives in AWS Personal Health Dashboard. By default, this is set to `false`.
        pub disable_email_notification: pulumi_wasm_rust::Output<Option<bool>>,
        pub disabled_reason: pulumi_wasm_rust::Output<String>,
        /// Email address for the account.
        pub email_address: pulumi_wasm_rust::Output<String>,
        /// ARN of the behavior graph to invite the member accounts to contribute their data to.
        pub graph_arn: pulumi_wasm_rust::Output<String>,
        /// Date and time, in UTC and extended RFC 3339 format, when an Amazon Detective membership invitation was last sent to the account.
        pub invited_time: pulumi_wasm_rust::Output<String>,
        /// A custom message to include in the invitation. Amazon Detective adds this message to the standard content that it sends for an invitation.
        pub message: pulumi_wasm_rust::Output<Option<String>>,
        /// Current membership status of the member account.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Date and time, in UTC and extended RFC 3339 format, of the most recent change to the member account's status.
        pub updated_time: pulumi_wasm_rust::Output<String>,
        /// Data volume in bytes per day for the member account.
        pub volume_usage_in_bytes: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MemberArgs) -> MemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let disable_email_notification_binding = args
            .disable_email_notification
            .get_inner();
        let email_address_binding = args.email_address.get_inner();
        let graph_arn_binding = args.graph_arn.get_inner();
        let message_binding = args.message.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:detective/member:Member".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "disableEmailNotification".into(),
                    value: &disable_email_notification_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddress".into(),
                    value: &email_address_binding,
                },
                register_interface::ObjectField {
                    name: "graphArn".into(),
                    value: &graph_arn_binding,
                },
                register_interface::ObjectField {
                    name: "message".into(),
                    value: &message_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "administratorId".into(),
                },
                register_interface::ResultField {
                    name: "disableEmailNotification".into(),
                },
                register_interface::ResultField {
                    name: "disabledReason".into(),
                },
                register_interface::ResultField {
                    name: "emailAddress".into(),
                },
                register_interface::ResultField {
                    name: "graphArn".into(),
                },
                register_interface::ResultField {
                    name: "invitedTime".into(),
                },
                register_interface::ResultField {
                    name: "message".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "updatedTime".into(),
                },
                register_interface::ResultField {
                    name: "volumeUsageInBytes".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MemberResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            administrator_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorId").unwrap(),
            ),
            disable_email_notification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableEmailNotification").unwrap(),
            ),
            disabled_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabledReason").unwrap(),
            ),
            email_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAddress").unwrap(),
            ),
            graph_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("graphArn").unwrap(),
            ),
            invited_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invitedTime").unwrap(),
            ),
            message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("message").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            updated_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedTime").unwrap(),
            ),
            volume_usage_in_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeUsageInBytes").unwrap(),
            ),
        }
    }
}