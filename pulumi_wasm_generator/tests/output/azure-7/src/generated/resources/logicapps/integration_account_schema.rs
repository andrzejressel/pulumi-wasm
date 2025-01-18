/// Manages a Logic App Integration Account Schema.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleIntegrationAccount:
///     type: azure:logicapps:IntegrationAccount
///     name: example
///     properties:
///       name: example-ia
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///   exampleIntegrationAccountSchema:
///     type: azure:logicapps:IntegrationAccountSchema
///     name: example
///     properties:
///       name: example-ias
///       resourceGroupName: ${example.name}
///       integrationAccountName: ${exampleIntegrationAccount.name}
///       content:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: testdata/integration_account_schema_content.xsd
///           return: result
/// ```
///
/// ## Import
///
/// Logic App Integration Account Schemas can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountSchema:IntegrationAccountSchema example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/schemas/schema1
/// ```
///
pub mod integration_account_schema {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountSchemaArgs {
        /// The content of the Logic App Integration Account Schema.
        #[builder(into)]
        pub content: pulumi_wasm_rust::Output<String>,
        /// The file name of the Logic App Integration Account Schema.
        #[builder(into, default)]
        pub file_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Schema to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// The metadata of the Logic App Integration Account Schema.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Logic App Integration Account Schema. Changing this forces a new Logic App Integration Account Schema to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Schema should exist. Changing this forces a new Logic App Integration Account Schema to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountSchemaResult {
        /// The content of the Logic App Integration Account Schema.
        pub content: pulumi_wasm_rust::Output<String>,
        /// The file name of the Logic App Integration Account Schema.
        pub file_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Schema to be created.
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// The metadata of the Logic App Integration Account Schema.
        pub metadata: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Logic App Integration Account Schema. Changing this forces a new Logic App Integration Account Schema to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account Schema should exist. Changing this forces a new Logic App Integration Account Schema to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IntegrationAccountSchemaArgs,
    ) -> IntegrationAccountSchemaResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_binding = args.content.get_inner();
        let file_name_binding = args.file_name.get_inner();
        let integration_account_name_binding = args.integration_account_name.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountSchema:IntegrationAccountSchema"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "fileName".into(),
                    value: &file_name_binding,
                },
                register_interface::ObjectField {
                    name: "integrationAccountName".into(),
                    value: &integration_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
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
                    name: "fileName".into(),
                },
                register_interface::ResultField {
                    name: "integrationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
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
        IntegrationAccountSchemaResult {
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
            ),
            file_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileName").unwrap(),
            ),
            integration_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationAccountName").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
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
