/// Resource for enabling Amazon Inspector resource scans.
///
/// This resource must be created in the Organization's Administrator Account.
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
///     let example = enabler::create(
///         "example",
///         EnablerArgs::builder()
///             .account_ids(vec!["123456789012",])
///             .resource_types(vec!["EC2",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### For the Calling Account
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let test = enabler::create(
///         "test",
///         EnablerArgs::builder()
///             .account_ids(vec!["${current.accountId}",])
///             .resource_types(vec!["ECR", "EC2",])
///             .build_struct(),
///     );
/// }
/// ```
pub mod enabler {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnablerArgs {
        /// Set of account IDs.
        /// Can contain one of: the Organization's Administrator Account, or one or more Member Accounts.
        #[builder(into)]
        pub account_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Type of resources to scan.
        /// Valid values are `EC2`, `ECR`, `LAMBDA` and `LAMBDA_CODE`.
        /// At least one item is required.
        #[builder(into)]
        pub resource_types: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct EnablerResult {
        /// Set of account IDs.
        /// Can contain one of: the Organization's Administrator Account, or one or more Member Accounts.
        pub account_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Type of resources to scan.
        /// Valid values are `EC2`, `ECR`, `LAMBDA` and `LAMBDA_CODE`.
        /// At least one item is required.
        pub resource_types: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnablerArgs) -> EnablerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_ids_binding = args.account_ids.get_inner();
        let resource_types_binding = args.resource_types.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:inspector2/enabler:Enabler".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountIds".into(),
                    value: &account_ids_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTypes".into(),
                    value: &resource_types_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountIds".into(),
                },
                register_interface::ResultField {
                    name: "resourceTypes".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnablerResult {
            account_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountIds").unwrap(),
            ),
            resource_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTypes").unwrap(),
            ),
        }
    }
}