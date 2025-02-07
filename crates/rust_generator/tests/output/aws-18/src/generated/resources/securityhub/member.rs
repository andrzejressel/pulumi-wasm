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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MemberArgs,
    ) -> MemberResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let email_binding = args.email.get_output(context).get_inner();
        let invite_binding = args.invite.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/member:Member".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
            email: pulumi_gestalt_rust::__private::into_domain(o.extract_field("email")),
            invite: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("invite"),
            ),
            master_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("masterId"),
            ),
            member_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("memberStatus"),
            ),
        }
    }
}
