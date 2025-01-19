/// Resource for managing an Amazon Inspector Delegated Admin Account.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:inspector2:DelegatedAdminAccount
///     properties:
///       accountId: ${current.accountId}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Inspector Delegated Admin Account using the `account_id`. For example:
///
/// ```sh
/// $ pulumi import aws:inspector2/delegatedAdminAccount:DelegatedAdminAccount example 123456789012
/// ```
pub mod delegated_admin_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DelegatedAdminAccountArgs {
        /// Account to enable as delegated admin account.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DelegatedAdminAccountResult {
        /// Account to enable as delegated admin account.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Status of this delegated admin account.
        pub relationship_status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DelegatedAdminAccountArgs,
    ) -> DelegatedAdminAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:inspector2/delegatedAdminAccount:DelegatedAdminAccount".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
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
        DelegatedAdminAccountResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            relationship_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("relationshipStatus").unwrap(),
            ),
        }
    }
}
