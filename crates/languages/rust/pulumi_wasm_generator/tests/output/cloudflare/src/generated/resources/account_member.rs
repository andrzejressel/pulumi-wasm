/// Provides a resource which manages Cloudflare account members.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account_member::create(
///         "example",
///         AccountMemberArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .email_address("user@example.com")
///             .role_ids(
///                 vec![
///                     "68b329da9893e34099c7d8ad5cb9c940",
///                     "d784fa8b6d98d27699781bd9a7cf19f0",
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/accountMember:AccountMember example <account_id>/<member_id>
/// ```
///
pub mod account_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountMemberArgs {
        /// Account ID to create the account member in.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The email address of the user who you wish to manage. Following creation, this field becomes read only via the API and cannot be updated.
        #[builder(into)]
        pub email_address: pulumi_wasm_rust::InputOrOutput<String>,
        /// List of account role IDs that you want to assign to a member.
        #[builder(into)]
        pub role_ids: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// A member's status in the account. Available values: `accepted`, `pending`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountMemberResult {
        /// Account ID to create the account member in.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The email address of the user who you wish to manage. Following creation, this field becomes read only via the API and cannot be updated.
        pub email_address: pulumi_wasm_rust::Output<String>,
        /// List of account role IDs that you want to assign to a member.
        pub role_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A member's status in the account. Available values: `accepted`, `pending`.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccountMemberArgs,
    ) -> AccountMemberResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let email_address_binding = args.email_address.get_output(context).get_inner();
        let role_ids_binding = args.role_ids.get_output(context).get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/accountMember:AccountMember".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddress".into(),
                    value: &email_address_binding,
                },
                register_interface::ObjectField {
                    name: "roleIds".into(),
                    value: &role_ids_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountMemberResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            email_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("emailAddress"),
            ),
            role_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleIds"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
        }
    }
}
