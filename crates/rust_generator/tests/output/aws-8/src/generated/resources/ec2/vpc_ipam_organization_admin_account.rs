/// Enables the IPAM Service and promotes a delegated administrator.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpamOrganizationAdminAccount
///     properties:
///       delegatedAdminAccountId: ${delegated.accountId}
/// variables:
///   delegated:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamOrganizationAdminAccountArgs {
        #[builder(into)]
        pub delegated_admin_account_id: pulumi_wasm_rust::InputOrOutput<String>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpcIpamOrganizationAdminAccountArgs,
    ) -> VpcIpamOrganizationAdminAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delegated_admin_account_id_binding = args
            .delegated_admin_account_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamOrganizationAdminAccount:VpcIpamOrganizationAdminAccount"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "delegatedAdminAccountId".into(),
                    value: &delegated_admin_account_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcIpamOrganizationAdminAccountResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            delegated_admin_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("delegatedAdminAccountId"),
            ),
            email: pulumi_wasm_rust::__private::into_domain(o.extract_field("email")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            service_principal: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("servicePrincipal"),
            ),
        }
    }
}
