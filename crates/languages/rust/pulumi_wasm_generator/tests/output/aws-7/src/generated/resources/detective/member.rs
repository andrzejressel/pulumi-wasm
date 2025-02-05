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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MemberArgs {
        /// AWS account ID for the account.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// If set to true, then the root user of the invited account will _not_ receive an email notification. This notification is in addition to an alert that the root user receives in AWS Personal Health Dashboard. By default, this is set to `false`.
        #[builder(into, default)]
        pub disable_email_notification: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Email address for the account.
        #[builder(into)]
        pub email_address: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the behavior graph to invite the member accounts to contribute their data to.
        #[builder(into)]
        pub graph_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// A custom message to include in the invitation. Amazon Detective adds this message to the standard content that it sends for an invitation.
        #[builder(into, default)]
        pub message: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MemberArgs,
    ) -> MemberResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let disable_email_notification_binding = args
            .disable_email_notification
            .get_output(context)
            .get_inner();
        let email_address_binding = args.email_address.get_output(context).get_inner();
        let graph_arn_binding = args.graph_arn.get_output(context).get_inner();
        let message_binding = args.message.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:detective/member:Member".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        MemberResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            administrator_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("administratorId"),
            ),
            disable_email_notification: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disableEmailNotification"),
            ),
            disabled_reason: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disabledReason"),
            ),
            email_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("emailAddress"),
            ),
            graph_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("graphArn"),
            ),
            invited_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("invitedTime"),
            ),
            message: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("message"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            updated_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updatedTime"),
            ),
            volume_usage_in_bytes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("volumeUsageInBytes"),
            ),
        }
    }
}
