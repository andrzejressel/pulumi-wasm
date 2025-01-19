/// Manages a Logic App Integration Account Session.
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
///     let exampleIntegrationAccount = integration_account::create(
///         "exampleIntegrationAccount",
///         IntegrationAccountArgs::builder()
///             .location("${example.location}")
///             .name("example-ia")
///             .resource_group_name("${example.name}")
///             .sku_name("Basic")
///             .build_struct(),
///     );
///     let exampleIntegrationAccountSession = integration_account_session::create(
///         "exampleIntegrationAccountSession",
///         IntegrationAccountSessionArgs::builder()
///             .content(" {\n       \"controlNumber\": \"1234\"\n    }")
///             .integration_account_name("${exampleIntegrationAccount.name}")
///             .name("example-ias")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Logic App Integration Account Sessions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountSession:IntegrationAccountSession example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/sessions/session1
/// ```
///
pub mod integration_account_session {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountSessionArgs {
        /// The content of the Logic App Integration Account Session.
        #[builder(into)]
        pub content: pulumi_wasm_rust::Output<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Session to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Logic App Integration Account Session. Changing this forces a new Logic App Integration Account Session to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Session should exist. Changing this forces a new Logic App Integration Account Session to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountSessionResult {
        /// The content of the Logic App Integration Account Session.
        pub content: pulumi_wasm_rust::Output<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Session to be created.
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Logic App Integration Account Session. Changing this forces a new Logic App Integration Account Session to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account Session should exist. Changing this forces a new Logic App Integration Account Session to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IntegrationAccountSessionArgs,
    ) -> IntegrationAccountSessionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_binding = args.content.get_inner();
        let integration_account_name_binding = args.integration_account_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountSession:IntegrationAccountSession"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "integrationAccountName".into(),
                    value: &integration_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "content".into(),
                },
                register_interface::ResultField {
                    name: "integrationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationAccountSessionResult {
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
            ),
            integration_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationAccountName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
