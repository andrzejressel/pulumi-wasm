/// Manages a Logic App Integration Account Map.
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
///       skuName: Standard
///   exampleIntegrationAccountMap:
///     type: azure:logicapps:IntegrationAccountMap
///     name: example
///     properties:
///       name: example-iamap
///       resourceGroupName: ${example.name}
///       integrationAccountName: ${exampleIntegrationAccount.name}
///       mapType: Xslt
///       content:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: testdata/integration_account_map_content.xsd
///           return: result
/// ```
///
/// ## Import
///
/// Logic App Integration Account Maps can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountMap:IntegrationAccountMap example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/maps/map1
/// ```
///
pub mod integration_account_map {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountMapArgs {
        /// The content of the Logic App Integration Account Map.
        #[builder(into)]
        pub content: pulumi_wasm_rust::Output<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Map to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// The type of the Logic App Integration Account Map. Possible values are `Liquid`, `NotSpecified`, `Xslt`, `Xslt30` and `Xslt20`.
        #[builder(into)]
        pub map_type: pulumi_wasm_rust::Output<String>,
        /// The metadata of the Logic App Integration Account Map.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Map. Changing this forces a new Logic App Integration Account Map to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Map should exist. Changing this forces a new Logic App Integration Account Map to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountMapResult {
        /// The content of the Logic App Integration Account Map.
        pub content: pulumi_wasm_rust::Output<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Map to be created.
        pub integration_account_name: pulumi_wasm_rust::Output<String>,
        /// The type of the Logic App Integration Account Map. Possible values are `Liquid`, `NotSpecified`, `Xslt`, `Xslt30` and `Xslt20`.
        pub map_type: pulumi_wasm_rust::Output<String>,
        /// The metadata of the Logic App Integration Account Map.
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Map. Changing this forces a new Logic App Integration Account Map to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account Map should exist. Changing this forces a new Logic App Integration Account Map to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IntegrationAccountMapArgs,
    ) -> IntegrationAccountMapResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_binding = args.content.get_inner();
        let integration_account_name_binding = args.integration_account_name.get_inner();
        let map_type_binding = args.map_type.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountMap:IntegrationAccountMap".into(),
            name: name.to_string(),
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
                    name: "mapType".into(),
                    value: &map_type_binding,
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
                    name: "integrationAccountName".into(),
                },
                register_interface::ResultField {
                    name: "mapType".into(),
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
        IntegrationAccountMapResult {
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
            ),
            integration_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationAccountName").unwrap(),
            ),
            map_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mapType").unwrap(),
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
