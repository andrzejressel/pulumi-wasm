/// > **Note:** AWS accounts can only be associated with a single Security Hub master account. Destroying this resource will disassociate the member account from the master account.
///
/// Accepts a Security Hub invitation.
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
///     let invitee = account::create("invitee", AccountArgs::builder().build_struct());
///     let inviteeInviteAccepter = invite_accepter::create(
///         "inviteeInviteAccepter",
///         InviteAccepterArgs::builder()
///             .master_id("${exampleMember.masterId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Hub invite acceptance using the account ID. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/inviteAccepter:InviteAccepter example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod invite_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InviteAccepterArgs {
        /// The account ID of the master Security Hub account whose invitation you're accepting.
        #[builder(into)]
        pub master_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InviteAccepterResult {
        /// The ID of the invitation.
        pub invitation_id: pulumi_gestalt_rust::Output<String>,
        /// The account ID of the master Security Hub account whose invitation you're accepting.
        pub master_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InviteAccepterArgs,
    ) -> InviteAccepterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let master_id_binding = args.master_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securityhub/inviteAccepter:InviteAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterId".into(),
                    value: master_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InviteAccepterResult {
            invitation_id: o.get_field("invitationId"),
            master_id: o.get_field("masterId"),
        }
    }
}
