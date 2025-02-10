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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipam_organization_admin_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamOrganizationAdminAccountArgs {
        #[builder(into)]
        pub delegated_admin_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpcIpamOrganizationAdminAccountResult {
        /// The Organizations ARN for the delegate account.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub delegated_admin_account_id: pulumi_gestalt_rust::Output<String>,
        /// The Organizations email for the delegate account.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The Organizations name for the delegate account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The AWS service principal.
        pub service_principal: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIpamOrganizationAdminAccountArgs,
    ) -> VpcIpamOrganizationAdminAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let delegated_admin_account_id_binding = args
            .delegated_admin_account_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamOrganizationAdminAccount:VpcIpamOrganizationAdminAccount"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegatedAdminAccountId".into(),
                    value: delegated_admin_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcIpamOrganizationAdminAccountResult {
            arn: o.get_field("arn"),
            delegated_admin_account_id: o.get_field("delegatedAdminAccountId"),
            email: o.get_field("email"),
            name: o.get_field("name"),
            service_principal: o.get_field("servicePrincipal"),
        }
    }
}
