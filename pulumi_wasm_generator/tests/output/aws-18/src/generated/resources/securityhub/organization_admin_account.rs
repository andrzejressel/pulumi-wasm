/// Manages a Security Hub administrator account for an organization. The AWS account utilizing this resource must be an Organizations primary account. More information about Organizations support in Security Hub can be found in the [Security Hub User Guide](https://docs.aws.amazon.com/securityhub/latest/userguide/designate-orgs-admin-account.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization::create(
///         "example",
///         OrganizationArgs::builder()
///             .aws_service_access_principals(vec!["securityhub.amazonaws.com",])
///             .feature_set("ALL")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder().build_struct(),
///     );
///     let exampleOrganizationAdminAccount = organization_admin_account::create(
///         "exampleOrganizationAdminAccount",
///         OrganizationAdminAccountArgs::builder()
///             .admin_account_id("123456789012")
///             .build_struct(),
///     );
///     let exampleOrganizationConfiguration = organization_configuration::create(
///         "exampleOrganizationConfiguration",
///         OrganizationConfigurationArgs::builder().auto_enable(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Hub Organization Admin Accounts using the AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/organizationAdminAccount:OrganizationAdminAccount example 123456789012
/// ```
pub mod organization_admin_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationAdminAccountArgs {
        /// The AWS account identifier of the account to designate as the Security Hub administrator account.
        #[builder(into)]
        pub admin_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationAdminAccountResult {
        /// The AWS account identifier of the account to designate as the Security Hub administrator account.
        pub admin_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationAdminAccountArgs,
    ) -> OrganizationAdminAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_account_id_binding = args.admin_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/organizationAdminAccount:OrganizationAdminAccount"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminAccountId".into(),
                    value: &admin_account_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adminAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationAdminAccountResult {
            admin_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminAccountId").unwrap(),
            ),
        }
    }
}
