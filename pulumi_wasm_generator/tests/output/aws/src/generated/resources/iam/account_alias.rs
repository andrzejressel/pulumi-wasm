/// > **Note:** There is only a single account alias per AWS account.
///
/// Manages the account alias for the AWS Account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let alias = account_alias::create(
///         "alias",
///         AccountAliasArgs::builder().account_alias("my-account-alias").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the current Account Alias using the `account_alias`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/accountAlias:AccountAlias alias my-account-alias
/// ```
pub mod account_alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountAliasArgs {
        /// The account alias
        #[builder(into)]
        pub account_alias: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AccountAliasResult {
        /// The account alias
        pub account_alias: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccountAliasArgs) -> AccountAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_alias_binding = args.account_alias.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/accountAlias:AccountAlias".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountAlias".into(),
                    value: &account_alias_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountAlias".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountAliasResult {
            account_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountAlias").unwrap(),
            ),
        }
    }
}