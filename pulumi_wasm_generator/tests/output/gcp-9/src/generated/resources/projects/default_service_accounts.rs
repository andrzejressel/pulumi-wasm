/// Allows management of Google Cloud Platform project default service accounts.
///
/// When certain service APIs are enabled, Google Cloud Platform automatically creates service accounts to help get started, but
/// this is not recommended for production environments as per [Google's documentation](https://cloud.google.com/iam/docs/service-accounts#default).
/// See the [Organization documentation](https://cloud.google.com/resource-manager/docs/quickstarts) for more details.
///
/// > **WARNING** Some Google Cloud products do not work if the default service accounts are deleted so it is better to `DEPRIVILEGE` as
/// Google **CAN NOT** recover service accounts that have been deleted for more than 30 days.
/// Also Google recommends using the `constraints/iam.automaticIamGrantsForDefaultServiceAccounts` [constraint](https://www.terraform.io/docs/providers/google/r/google_organization_policy.html)
/// to disable automatic IAM Grants to default service accounts.
///
/// > This resource works on a best-effort basis, as no API formally describes the default service accounts
/// and it is for users who are unable to use constraints. If the default service accounts change their name
/// or additional service accounts are added, this resource will need to be updated.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myProject = default_service_accounts::create(
///         "myProject",
///         DefaultServiceAccountsArgs::builder()
///             .action("DELETE")
///             .project("my-project-id")
///             .build_struct(),
///     );
/// }
/// ```
///
/// To enable the default service accounts on the resource destroy:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myProject = default_service_accounts::create(
///         "myProject",
///         DefaultServiceAccountsArgs::builder()
///             .action("DISABLE")
///             .project("my-project-id")
///             .restore_policy("REVERT")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// This resource does not support import
///
pub mod default_service_accounts {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultServiceAccountsArgs {
        /// The action to be performed in the default service accounts. Valid values are: `DEPRIVILEGE`, `DELETE`, `DISABLE`. Note that `DEPRIVILEGE` action will ignore the REVERT configuration in the restore_policy
        #[builder(into)]
        pub action: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project ID where service accounts are created.
        #[builder(into)]
        pub project: pulumi_wasm_rust::InputOrOutput<String>,
        /// The action to be performed in the default service accounts on the resource destroy.
        /// Valid values are NONE, REVERT and REVERT_AND_IGNORE_FAILURE. It is applied for any action but in the DEPRIVILEGE.
        /// If set to REVERT it attempts to restore all default SAs but the DEPRIVILEGE action.
        /// If set to REVERT_AND_IGNORE_FAILURE it is the same behavior as REVERT but ignores errors returned by the API.
        #[builder(into, default)]
        pub restore_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DefaultServiceAccountsResult {
        /// The action to be performed in the default service accounts. Valid values are: `DEPRIVILEGE`, `DELETE`, `DISABLE`. Note that `DEPRIVILEGE` action will ignore the REVERT configuration in the restore_policy
        pub action: pulumi_wasm_rust::Output<String>,
        /// The project ID where service accounts are created.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The action to be performed in the default service accounts on the resource destroy.
        /// Valid values are NONE, REVERT and REVERT_AND_IGNORE_FAILURE. It is applied for any action but in the DEPRIVILEGE.
        /// If set to REVERT it attempts to restore all default SAs but the DEPRIVILEGE action.
        /// If set to REVERT_AND_IGNORE_FAILURE it is the same behavior as REVERT but ignores errors returned by the API.
        pub restore_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The Service Accounts changed by this resource. It is used for `REVERT` the `action` on the destroy.
        pub service_accounts: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DefaultServiceAccountsArgs,
    ) -> DefaultServiceAccountsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let restore_policy_binding = args.restore_policy.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:projects/defaultServiceAccounts:DefaultServiceAccounts".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "restorePolicy".into(),
                    value: &restore_policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "restorePolicy".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccounts".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DefaultServiceAccountsResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            restore_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restorePolicy").unwrap(),
            ),
            service_accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccounts").unwrap(),
            ),
        }
    }
}
