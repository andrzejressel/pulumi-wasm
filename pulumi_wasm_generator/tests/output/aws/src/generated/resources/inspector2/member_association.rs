/// Resource for associating accounts to existing Inspector instances.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = member_association::create(
///         "example",
///         MemberAssociationArgs::builder().account_id("123456789012").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Inspector Member Association using the `account_id`. For example:
///
/// ```sh
/// $ pulumi import aws:inspector2/memberAssociation:MemberAssociation example 123456789012
/// ```
pub mod member_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MemberAssociationArgs {
        /// ID of the account to associate
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MemberAssociationResult {
        /// ID of the account to associate
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Account ID of the delegated administrator account
        pub delegated_admin_account_id: pulumi_wasm_rust::Output<String>,
        /// Status of the member relationship
        pub relationship_status: pulumi_wasm_rust::Output<String>,
        /// Date and time of the last update of the relationship
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MemberAssociationArgs) -> MemberAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:inspector2/memberAssociation:MemberAssociation".into(),
            name: name.to_string(),
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
                    name: "delegatedAdminAccountId".into(),
                },
                register_interface::ResultField {
                    name: "relationshipStatus".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MemberAssociationResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            delegated_admin_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegatedAdminAccountId").unwrap(),
            ),
            relationship_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("relationshipStatus").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}