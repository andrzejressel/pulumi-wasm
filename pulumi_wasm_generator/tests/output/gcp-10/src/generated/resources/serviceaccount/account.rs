/// Allows management of a Google Cloud service account.
///
/// * [API documentation](https://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/access/service-accounts)
///
/// > **Warning:**  If you delete and recreate a service account, you must reapply any IAM roles that it had before.
///
/// > Creation of service accounts is eventually consistent, and that can lead to
/// errors when you try to apply ACLs to service accounts immediately after
/// creation.
///
/// ## Example Usage
///
/// This snippet creates a service account in a project.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let serviceAccount = account::create(
///         "serviceAccount",
///         AccountArgs::builder()
///             .account_id("service-account-id")
///             .display_name("Service Account")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Service accounts can be imported using their URI, e.g.
///
/// * `projects/{{project_id}}/serviceAccounts/{{email}}`
///
/// When using the `pulumi import` command, service accounts can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:serviceaccount/account:Account default projects/{{project_id}}/serviceAccounts/{{email}}
/// ```
///
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// The account id that is used to generate the service
        /// account email address and a stable unique id. It is unique within a project,
        /// must be 6-30 characters long, and match the regular expression `a-z`
        /// to comply with RFC1035. Changing this forces a new service account to be created.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If set to true, skip service account creation if a service account with the same email already exists.
        #[builder(into, default)]
        pub create_ignore_already_exists: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A text description of the service account.
        /// Must be less than or equal to 256 UTF-8 bytes.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether a service account is disabled or not. Defaults to `false`. This field has no effect during creation.
        /// Must be set after creation to disable a service account.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The display name for the service account.
        /// Can be updated without creating a new resource.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project that the service account will be created in.
        /// Defaults to the provider project configuration.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// The account id that is used to generate the service
        /// account email address and a stable unique id. It is unique within a project,
        /// must be 6-30 characters long, and match the regular expression `a-z`
        /// to comply with RFC1035. Changing this forces a new service account to be created.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// If set to true, skip service account creation if a service account with the same email already exists.
        pub create_ignore_already_exists: pulumi_wasm_rust::Output<Option<bool>>,
        /// A text description of the service account.
        /// Must be less than or equal to 256 UTF-8 bytes.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether a service account is disabled or not. Defaults to `false`. This field has no effect during creation.
        /// Must be set after creation to disable a service account.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The display name for the service account.
        /// Can be updated without creating a new resource.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The e-mail address of the service account. This value
        /// should be referenced from any `gcp.organizations.getIAMPolicy` data sources
        /// that would grant the service account privileges.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The Identity of the service account in the form `serviceAccount:{email}`. This value is often used to refer to the service account in order to grant IAM permissions.
        pub member: pulumi_wasm_rust::Output<String>,
        /// The fully-qualified name of the service account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project that the service account will be created in.
        /// Defaults to the provider project configuration.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The unique id of the service account.
        pub unique_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccountArgs,
    ) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let create_ignore_already_exists_binding = args
            .create_ignore_already_exists
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:serviceaccount/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "createIgnoreAlreadyExists".into(),
                    value: &create_ignore_already_exists_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "createIgnoreAlreadyExists".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disabled".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "member".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "uniqueId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            create_ignore_already_exists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createIgnoreAlreadyExists").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabled").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            member: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("member").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            unique_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueId").unwrap(),
            ),
        }
    }
}
