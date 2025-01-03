/// Provides a resource to manage an [Amazon Macie Organization Admin Account](https://docs.aws.amazon.com/macie/latest/APIReference/admin.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleOrganizationAdminAccount = organization_admin_account::create(
///         "exampleOrganizationAdminAccount",
///         OrganizationAdminAccountArgs::builder()
///             .admin_account_id("ID OF THE ADMIN ACCOUNT")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_organization_admin_account` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:macie2/organizationAdminAccount:OrganizationAdminAccount example abcd1
/// ```
pub mod organization_admin_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationAdminAccountArgs {
        /// The AWS account ID for the account to designate as the delegated Amazon Macie administrator account for the organization.
        #[builder(into)]
        pub admin_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationAdminAccountResult {
        /// The AWS account ID for the account to designate as the delegated Amazon Macie administrator account for the organization.
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
            type_: "aws:macie2/organizationAdminAccount:OrganizationAdminAccount".into(),
            name: name.to_string(),
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
