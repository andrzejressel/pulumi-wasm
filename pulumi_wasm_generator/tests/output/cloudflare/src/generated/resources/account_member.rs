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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountMemberArgs {
        /// Account ID to create the account member in.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The email address of the user who you wish to manage. Following creation, this field becomes read only via the API and cannot be updated.
        #[builder(into)]
        pub email_address: pulumi_wasm_rust::Output<String>,
        /// List of account role IDs that you want to assign to a member.
        #[builder(into)]
        pub role_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A member's status in the account. Available values: `accepted`, `pending`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn create(name: &str, args: AccountMemberArgs) -> AccountMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let email_address_binding = args.email_address.get_inner();
        let role_ids_binding = args.role_ids.get_inner();
        let status_binding = args.status.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/accountMember:AccountMember".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "emailAddress".into(),
                },
                register_interface::ResultField {
                    name: "roleIds".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountMemberResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            email_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAddress").unwrap(),
            ),
            role_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleIds").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
