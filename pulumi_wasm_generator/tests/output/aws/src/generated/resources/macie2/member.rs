/// Provides a resource to manage an [Amazon Macie Member](https://docs.aws.amazon.com/macie/latest/APIReference/members-id.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleMember = member::create(
///         "exampleMember",
///         MemberArgs::builder()
///             .account_id("AWS ACCOUNT ID")
///             .email("EMAIL")
///             .invitation_disable_email_notification(true)
///             .invitation_message("Message of the invitation")
///             .invite(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_member` using the account ID of the member account. For example:
///
/// ```sh
/// $ pulumi import aws:macie2/member:Member example 123456789012
/// ```
pub mod member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MemberArgs {
        /// The AWS account ID for the account.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The email address for the account.
        #[builder(into)]
        pub email: pulumi_wasm_rust::Output<String>,
        /// Specifies whether to send an email notification to the root user of each account that the invitation will be sent to. This notification is in addition to an alert that the root user receives in AWS Personal Health Dashboard. To send an email notification to the root user of each account, set this value to `true`.
        #[builder(into, default)]
        pub invitation_disable_email_notification: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// A custom message to include in the invitation. Amazon Macie adds this message to the standard content that it sends for an invitation.
        #[builder(into, default)]
        pub invitation_message: pulumi_wasm_rust::Output<Option<String>>,
        /// Send an invitation to a member
        #[builder(into, default)]
        pub invite: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the status for the account. To enable Amazon Macie and start all Macie activities for the account, set this value to `ENABLED`. Valid values are `ENABLED` or `PAUSED`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MemberResult {
        /// The AWS account ID for the account.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The AWS account ID for the administrator account.
        pub administrator_account_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the account.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The email address for the account.
        pub email: pulumi_wasm_rust::Output<String>,
        /// Specifies whether to send an email notification to the root user of each account that the invitation will be sent to. This notification is in addition to an alert that the root user receives in AWS Personal Health Dashboard. To send an email notification to the root user of each account, set this value to `true`.
        pub invitation_disable_email_notification: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// A custom message to include in the invitation. Amazon Macie adds this message to the standard content that it sends for an invitation.
        pub invitation_message: pulumi_wasm_rust::Output<Option<String>>,
        /// Send an invitation to a member
        pub invite: pulumi_wasm_rust::Output<bool>,
        /// The date and time, in UTC and extended RFC 3339 format, when an Amazon Macie membership invitation was last sent to the account. This value is null if a Macie invitation hasn't been sent to the account.
        pub invited_at: pulumi_wasm_rust::Output<String>,
        pub master_account_id: pulumi_wasm_rust::Output<String>,
        /// The current status of the relationship between the account and the administrator account.
        pub relationship_status: pulumi_wasm_rust::Output<String>,
        /// Specifies the status for the account. To enable Amazon Macie and start all Macie activities for the account, set this value to `ENABLED`. Valid values are `ENABLED` or `PAUSED`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A map of key-value pairs that specifies the tags to associate with the account in Amazon Macie.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The date and time, in UTC and extended RFC 3339 format, of the most recent change to the status of the relationship between the account and the administrator account.
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MemberArgs) -> MemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let email_binding = args.email.get_inner();
        let invitation_disable_email_notification_binding = args
            .invitation_disable_email_notification
            .get_inner();
        let invitation_message_binding = args.invitation_message.get_inner();
        let invite_binding = args.invite.get_inner();
        let status_binding = args.status.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:macie2/member:Member".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "invitationDisableEmailNotification".into(),
                    value: &invitation_disable_email_notification_binding,
                },
                register_interface::ObjectField {
                    name: "invitationMessage".into(),
                    value: &invitation_message_binding,
                },
                register_interface::ObjectField {
                    name: "invite".into(),
                    value: &invite_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "administratorAccountId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "invitationDisableEmailNotification".into(),
                },
                register_interface::ResultField {
                    name: "invitationMessage".into(),
                },
                register_interface::ResultField {
                    name: "invite".into(),
                },
                register_interface::ResultField {
                    name: "invitedAt".into(),
                },
                register_interface::ResultField {
                    name: "masterAccountId".into(),
                },
                register_interface::ResultField {
                    name: "relationshipStatus".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
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
            administrator_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorAccountId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            invitation_disable_email_notification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invitationDisableEmailNotification").unwrap(),
            ),
            invitation_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invitationMessage").unwrap(),
            ),
            invite: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invite").unwrap(),
            ),
            invited_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invitedAt").unwrap(),
            ),
            master_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterAccountId").unwrap(),
            ),
            relationship_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("relationshipStatus").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}