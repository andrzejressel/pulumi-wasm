/// Resource for managing AWS Audit Manager Organization Admin Account Registration.
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
///     let example = organization_admin_account_registration::create(
///         "example",
///         OrganizationAdminAccountRegistrationArgs::builder()
///             .admin_account_id("123456789012")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Organization Admin Account Registration using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/organizationAdminAccountRegistration:OrganizationAdminAccountRegistration example 123456789012
/// ```
pub mod organization_admin_account_registration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationAdminAccountRegistrationArgs {
        /// Identifier for the organization administrator account.
        #[builder(into)]
        pub admin_account_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationAdminAccountRegistrationResult {
        /// Identifier for the organization administrator account.
        pub admin_account_id: pulumi_wasm_rust::Output<String>,
        /// Identifier for the organization.
        pub organization_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationAdminAccountRegistrationArgs,
    ) -> OrganizationAdminAccountRegistrationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_account_id_binding = args.admin_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:auditmanager/organizationAdminAccountRegistration:OrganizationAdminAccountRegistration"
                .into(),
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
                register_interface::ResultField {
                    name: "organizationId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationAdminAccountRegistrationResult {
            admin_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminAccountId").unwrap(),
            ),
            organization_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationId").unwrap(),
            ),
        }
    }
}
