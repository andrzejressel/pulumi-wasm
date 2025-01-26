/// Resource for managing AWS Audit Manager Account Registration.
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
///     let example = account_registration::create(
///         "example",
///         AccountRegistrationArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ### Deregister On Destroy
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account_registration::create(
///         "example",
///         AccountRegistrationArgs::builder().deregister_on_destroy(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Account Registration resources using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/accountRegistration:AccountRegistration example us-east-1
/// ```
pub mod account_registration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountRegistrationArgs {
        /// Identifier for the delegated administrator account.
        #[builder(into, default)]
        pub delegated_admin_account: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Flag to deregister AuditManager in the account upon destruction. Defaults to `false` (ie. AuditManager will remain active in the account, even if this resource is removed).
        #[builder(into, default)]
        pub deregister_on_destroy: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// KMS key identifier.
        #[builder(into, default)]
        pub kms_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountRegistrationResult {
        /// Identifier for the delegated administrator account.
        pub delegated_admin_account: pulumi_wasm_rust::Output<Option<String>>,
        /// Flag to deregister AuditManager in the account upon destruction. Defaults to `false` (ie. AuditManager will remain active in the account, even if this resource is removed).
        pub deregister_on_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// KMS key identifier.
        pub kms_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Status of the account registration request.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccountRegistrationArgs,
    ) -> AccountRegistrationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delegated_admin_account_binding = args
            .delegated_admin_account
            .get_output(context)
            .get_inner();
        let deregister_on_destroy_binding = args
            .deregister_on_destroy
            .get_output(context)
            .get_inner();
        let kms_key_binding = args.kms_key.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:auditmanager/accountRegistration:AccountRegistration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "delegatedAdminAccount".into(),
                    value: &delegated_admin_account_binding,
                },
                register_interface::ObjectField {
                    name: "deregisterOnDestroy".into(),
                    value: &deregister_on_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "delegatedAdminAccount".into(),
                },
                register_interface::ResultField {
                    name: "deregisterOnDestroy".into(),
                },
                register_interface::ResultField {
                    name: "kmsKey".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountRegistrationResult {
            delegated_admin_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegatedAdminAccount").unwrap(),
            ),
            deregister_on_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deregisterOnDestroy").unwrap(),
            ),
            kms_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKey").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
