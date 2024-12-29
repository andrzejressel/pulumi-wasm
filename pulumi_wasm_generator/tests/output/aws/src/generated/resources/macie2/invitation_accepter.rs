/// Provides a resource to manage an [Amazon Macie Invitation Accepter](https://docs.aws.amazon.com/macie/latest/APIReference/invitations-accept.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = account::create("member", AccountArgs::builder().build_struct());
///     let memberInvitationAccepter = invitation_accepter::create(
///         "memberInvitationAccepter",
///         InvitationAccepterArgs::builder()
///             .administrator_account_id("ADMINISTRATOR ACCOUNT ID")
///             .build_struct(),
///     );
///     let primary = account::create("primary", AccountArgs::builder().build_struct());
///     let primaryMember = member::create(
///         "primaryMember",
///         MemberArgs::builder()
///             .account_id("ACCOUNT ID")
///             .email("EMAIL")
///             .invitation_message("Message of the invite")
///             .invite(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_invitation_accepter` using the admin account ID. For example:
///
/// ```sh
/// $ pulumi import aws:macie2/invitationAccepter:InvitationAccepter example 123456789012
/// ```
pub mod invitation_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InvitationAccepterArgs {
        /// The AWS account ID for the account that sent the invitation.
        #[builder(into)]
        pub administrator_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct InvitationAccepterResult {
        /// The AWS account ID for the account that sent the invitation.
        pub administrator_account_id: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the invitation.
        pub invitation_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InvitationAccepterArgs) -> InvitationAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let administrator_account_id_binding = args.administrator_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:macie2/invitationAccepter:InvitationAccepter".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "administratorAccountId".into(),
                    value: &administrator_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "administratorAccountId".into(),
                },
                register_interface::ResultField {
                    name: "invitationId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InvitationAccepterResult {
            administrator_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorAccountId").unwrap(),
            ),
            invitation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invitationId").unwrap(),
            ),
        }
    }
}
