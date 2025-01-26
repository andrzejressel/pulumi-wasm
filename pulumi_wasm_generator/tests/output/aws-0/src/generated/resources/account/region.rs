/// Enable (Opt-In) or Disable (Opt-Out) a particular Region for an AWS account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = region::create(
///         "example",
///         RegionArgs::builder().enabled(true).region_name("ap-southeast-3").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`. For example:
///
/// ```sh
/// $ pulumi import aws:account/region:Region example ap-southeast-3
/// ```
pub mod region {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionArgs {
        /// The ID of the target account when managing member accounts. Will manage current user's account by default if omitted. To use this parameter, the caller must be an identity in the organization's management account or a delegated administrator account. The specified account ID must also be a member account in the same organization. The organization must have all features enabled, and the organization must have trusted access enabled for the Account Management service, and optionally a delegated admin account assigned.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether the region is enabled.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The region name to manage.
        #[builder(into)]
        pub region_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegionResult {
        /// The ID of the target account when managing member accounts. Will manage current user's account by default if omitted. To use this parameter, the caller must be an identity in the organization's management account or a delegated administrator account. The specified account ID must also be a member account in the same organization. The organization must have all features enabled, and the organization must have trusted access enabled for the Account Management service, and optionally a delegated admin account assigned.
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the region is enabled.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The region opt status.
        pub opt_status: pulumi_wasm_rust::Output<String>,
        /// The region name to manage.
        pub region_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegionArgs,
    ) -> RegionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let region_name_binding = args.region_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:account/region:Region".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "regionName".into(),
                    value: &region_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "optStatus".into(),
                },
                register_interface::ResultField {
                    name: "regionName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            opt_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optStatus").unwrap(),
            ),
            region_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regionName").unwrap(),
            ),
        }
    }
}
