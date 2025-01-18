/// Provides a resource to manage a GuardDuty member. To accept invitations in member accounts, see the `aws.guardduty.InviteAccepter` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = detector::create(
///         "member",
///         DetectorArgs::builder().enable(true).build_struct(),
///     );
///     let memberMember = member::create(
///         "memberMember",
///         MemberArgs::builder()
///             .account_id("${member.accountId}")
///             .detector_id("${primary.id}")
///             .email("required@example.com")
///             .invitation_message("please accept guardduty invitation")
///             .invite(true)
///             .build_struct(),
///     );
///     let primary = detector::create(
///         "primary",
///         DetectorArgs::builder().enable(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GuardDuty members using the primary GuardDuty detector ID and member AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:guardduty/member:Member MyMember 00b00fd5aecc0ab60a708659477e9617:123456789012
/// ```
pub mod member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MemberArgs {
        /// AWS account ID for member account.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The detector ID of the GuardDuty account where you want to create member accounts.
        #[builder(into)]
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// Boolean whether an email notification is sent to the accounts. Defaults to `false`.
        #[builder(into, default)]
        pub disable_email_notification: pulumi_wasm_rust::Output<Option<bool>>,
        /// Email address for member account.
        #[builder(into)]
        pub email: pulumi_wasm_rust::Output<String>,
        /// Message for invitation.
        #[builder(into, default)]
        pub invitation_message: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean whether to invite the account to GuardDuty as a member. Defaults to `false`. To detect if an invitation needs to be (re-)sent, the this provider state value is `true` based on a `relationship_status` of `Disabled`, `Enabled`, `Invited`, or `EmailVerificationInProgress`.
        #[builder(into, default)]
        pub invite: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct MemberResult {
        /// AWS account ID for member account.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The detector ID of the GuardDuty account where you want to create member accounts.
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// Boolean whether an email notification is sent to the accounts. Defaults to `false`.
        pub disable_email_notification: pulumi_wasm_rust::Output<Option<bool>>,
        /// Email address for member account.
        pub email: pulumi_wasm_rust::Output<String>,
        /// Message for invitation.
        pub invitation_message: pulumi_wasm_rust::Output<Option<String>>,
        /// Boolean whether to invite the account to GuardDuty as a member. Defaults to `false`. To detect if an invitation needs to be (re-)sent, the this provider state value is `true` based on a `relationship_status` of `Disabled`, `Enabled`, `Invited`, or `EmailVerificationInProgress`.
        pub invite: pulumi_wasm_rust::Output<Option<bool>>,
        /// The status of the relationship between the member account and its primary account. More information can be found in [Amazon GuardDuty API Reference](https://docs.aws.amazon.com/guardduty/latest/ug/get-members.html).
        pub relationship_status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MemberArgs) -> MemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let detector_id_binding = args.detector_id.get_inner();
        let disable_email_notification_binding = args
            .disable_email_notification
            .get_inner();
        let email_binding = args.email.get_inner();
        let invitation_message_binding = args.invitation_message.get_inner();
        let invite_binding = args.invite.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:guardduty/member:Member".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding,
                },
                register_interface::ObjectField {
                    name: "disableEmailNotification".into(),
                    value: &disable_email_notification_binding,
                },
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "invitationMessage".into(),
                    value: &invitation_message_binding,
                },
                register_interface::ObjectField {
                    name: "invite".into(),
                    value: &invite_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "detectorId".into(),
                },
                register_interface::ResultField {
                    name: "disableEmailNotification".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "invitationMessage".into(),
                },
                register_interface::ResultField {
                    name: "invite".into(),
                },
                register_interface::ResultField {
                    name: "relationshipStatus".into(),
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
            detector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectorId").unwrap(),
            ),
            disable_email_notification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableEmailNotification").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            invitation_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invitationMessage").unwrap(),
            ),
            invite: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invite").unwrap(),
            ),
            relationship_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("relationshipStatus").unwrap(),
            ),
        }
    }
}
