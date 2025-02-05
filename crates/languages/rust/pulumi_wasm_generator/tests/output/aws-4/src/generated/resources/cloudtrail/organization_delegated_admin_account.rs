/// Provides a resource to manage an AWS CloudTrail Delegated Administrator.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudtrail:OrganizationDelegatedAdminAccount
///     properties:
///       accountId: ${delegated.accountId}
/// variables:
///   delegated:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import delegated administrators using the delegate account `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudtrail/organizationDelegatedAdminAccount:OrganizationDelegatedAdminAccount example 12345678901
/// ```
pub mod organization_delegated_admin_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationDelegatedAdminAccountArgs {
        /// An organization member account ID that you want to designate as a delegated administrator.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationDelegatedAdminAccountResult {
        /// An organization member account ID that you want to designate as a delegated administrator.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the delegated administrator's account.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The email address that is associated with the delegated administrator's AWS account.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The friendly name of the delegated administrator's account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The AWS CloudTrail service principal name.
        pub service_principal: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OrganizationDelegatedAdminAccountArgs,
    ) -> OrganizationDelegatedAdminAccountResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudtrail/organizationDelegatedAdminAccount:OrganizationDelegatedAdminAccount"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OrganizationDelegatedAdminAccountResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            email: pulumi_wasm_rust::__private::into_domain(o.extract_field("email")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            service_principal: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("servicePrincipal"),
            ),
        }
    }
}
