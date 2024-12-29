/// Enables the IPAM Service and promotes a delegated administrator.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let delegated = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let example = vpc_ipam_organization_admin_account::create(
///         "example",
///         VpcIpamOrganizationAdminAccountArgs::builder()
///             .delegated_admin_account_id("${delegated.accountId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the delegate account `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamOrganizationAdminAccount:VpcIpamOrganizationAdminAccount example 12345678901
/// ```
pub mod vpc_ipam_organization_admin_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamOrganizationAdminAccountArgs {
        #[builder(into)]
        pub delegated_admin_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamOrganizationAdminAccountResult {
        /// The Organizations ARN for the delegate account.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub delegated_admin_account_id: pulumi_wasm_rust::Output<String>,
        /// The Organizations email for the delegate account.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The Organizations name for the delegate account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The AWS service principal.
        pub service_principal: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpcIpamOrganizationAdminAccountArgs,
    ) -> VpcIpamOrganizationAdminAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delegated_admin_account_id_binding = args
            .delegated_admin_account_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamOrganizationAdminAccount:VpcIpamOrganizationAdminAccount"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "delegatedAdminAccountId".into(),
                    value: &delegated_admin_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "delegatedAdminAccountId".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "servicePrincipal".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcIpamOrganizationAdminAccountResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            delegated_admin_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegatedAdminAccountId").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            service_principal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrincipal").unwrap(),
            ),
        }
    }
}
