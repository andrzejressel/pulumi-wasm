/// Provides a Security Hub member resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleMember = member::create(
///         "exampleMember",
///         MemberArgs::builder()
///             .account_id("123456789012")
///             .email("example@example.com")
///             .invite(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Hub members using their account ID. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/member:Member example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MemberArgs {
        /// The ID of the member AWS account.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The email of the member AWS account.
        #[builder(into, default)]
        pub email: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Boolean whether to invite the account to Security Hub as a member. Defaults to `false`.
        #[builder(into, default)]
        pub invite: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct MemberResult {
        /// The ID of the member AWS account.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The email of the member AWS account.
        pub email: pulumi_gestalt_rust::Output<Option<String>>,
        /// Boolean whether to invite the account to Security Hub as a member. Defaults to `false`.
        pub invite: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the master Security Hub AWS account.
        pub master_id: pulumi_gestalt_rust::Output<String>,
        /// The status of the member account relationship.
        pub member_status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MemberArgs,
    ) -> MemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let email_binding = args.email.get_output(context);
        let invite_binding = args.invite.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securityhub/member:Member".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "email".into(),
                    value: email_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "invite".into(),
                    value: invite_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MemberResult {
            account_id: o.get_field("accountId"),
            email: o.get_field("email"),
            invite: o.get_field("invite"),
            master_id: o.get_field("masterId"),
            member_status: o.get_field("memberStatus"),
        }
    }
}
