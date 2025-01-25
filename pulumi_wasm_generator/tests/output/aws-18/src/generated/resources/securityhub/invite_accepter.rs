/// > **Note:** AWS accounts can only be associated with a single Security Hub master account. Destroying this resource will disassociate the member account from the master account.
///
/// Accepts a Security Hub invitation.
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
pub mod invite_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InviteAccepterArgs {
        /// The account ID of the master Security Hub account whose invitation you're accepting.
        #[builder(into)]
        pub master_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InviteAccepterResult {
        /// The ID of the invitation.
        pub invitation_id: pulumi_wasm_rust::Output<String>,
        /// The account ID of the master Security Hub account whose invitation you're accepting.
        pub master_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InviteAccepterArgs,
    ) -> InviteAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let master_id_binding = args.master_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/inviteAccepter:InviteAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "masterId".into(),
                    value: &master_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "invitationId".into(),
                },
                register_interface::ResultField {
                    name: "masterId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InviteAccepterResult {
            invitation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invitationId").unwrap(),
            ),
            master_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterId").unwrap(),
            ),
        }
    }
}
