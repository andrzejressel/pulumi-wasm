/// Provides a resource to manage a GuardDuty member. To accept invitations in member accounts, see the `aws.guardduty.InviteAccepter` resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MemberArgs {
        /// AWS account ID for member account.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The detector ID of the GuardDuty account where you want to create member accounts.
        #[builder(into)]
        pub detector_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Boolean whether an email notification is sent to the accounts. Defaults to `false`.
        #[builder(into, default)]
        pub disable_email_notification: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Email address for member account.
        #[builder(into)]
        pub email: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Message for invitation.
        #[builder(into, default)]
        pub invitation_message: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean whether to invite the account to GuardDuty as a member. Defaults to `false`. To detect if an invitation needs to be (re-)sent, the this provider state value is `true` based on a `relationship_status` of `Disabled`, `Enabled`, `Invited`, or `EmailVerificationInProgress`.
        #[builder(into, default)]
        pub invite: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct MemberResult {
        /// AWS account ID for member account.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The detector ID of the GuardDuty account where you want to create member accounts.
        pub detector_id: pulumi_gestalt_rust::Output<String>,
        /// Boolean whether an email notification is sent to the accounts. Defaults to `false`.
        pub disable_email_notification: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Email address for member account.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// Message for invitation.
        pub invitation_message: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean whether to invite the account to GuardDuty as a member. Defaults to `false`. To detect if an invitation needs to be (re-)sent, the this provider state value is `true` based on a `relationship_status` of `Disabled`, `Enabled`, `Invited`, or `EmailVerificationInProgress`.
        pub invite: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The status of the relationship between the member account and its primary account. More information can be found in [Amazon GuardDuty API Reference](https://docs.aws.amazon.com/guardduty/latest/ug/get-members.html).
        pub relationship_status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MemberArgs,
    ) -> MemberResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let detector_id_binding_1 = args.detector_id.get_output(context);
        let detector_id_binding = detector_id_binding_1.get_inner();
        let disable_email_notification_binding_1 = args
            .disable_email_notification
            .get_output(context);
        let disable_email_notification_binding = disable_email_notification_binding_1
            .get_inner();
        let email_binding_1 = args.email.get_output(context);
        let email_binding = email_binding_1.get_inner();
        let invitation_message_binding_1 = args.invitation_message.get_output(context);
        let invitation_message_binding = invitation_message_binding_1.get_inner();
        let invite_binding_1 = args.invite.get_output(context);
        let invite_binding = invite_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        MemberResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            detector_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("detectorId"),
            ),
            disable_email_notification: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disableEmailNotification"),
            ),
            email: pulumi_gestalt_rust::__private::into_domain(o.extract_field("email")),
            invitation_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("invitationMessage"),
            ),
            invite: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("invite"),
            ),
            relationship_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("relationshipStatus"),
            ),
        }
    }
}
