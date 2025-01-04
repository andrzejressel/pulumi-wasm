/// Manages a Automation Credential.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("account1")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let exampleCredential = credential::create(
///         "exampleCredential",
///         CredentialArgs::builder()
///             .automation_account_name("${exampleAccount.name}")
///             .description("This is an example credential")
///             .name("credential1")
///             .password("example_pwd")
///             .resource_group_name("${example.name}")
///             .username("example_user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Automation Credentials can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/credential:Credential credential1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1/credentials/credential1
/// ```
///
pub mod credential {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CredentialArgs {
        /// The name of the automation account in which the Credential is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The description associated with this Automation Credential.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Credential. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The password associated with this Automation Credential.
        #[builder(into)]
        pub password: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Credential is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The username associated with this Automation Credential.
        #[builder(into)]
        pub username: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CredentialResult {
        /// The name of the automation account in which the Credential is created. Changing this forces a new resource to be created.
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The description associated with this Automation Credential.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Credential. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password associated with this Automation Credential.
        pub password: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Credential is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The username associated with this Automation Credential.
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CredentialArgs) -> CredentialResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args.automation_account_name.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let password_binding = args.password.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let username_binding = args.username.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:automation/credential:Credential".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "automationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "username".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CredentialResult {
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("automationAccountName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("username").unwrap(),
            ),
        }
    }
}
