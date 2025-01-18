/// Manages an API Connection.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleNamespace:
///     type: azure:servicebus:Namespace
///     name: example
///     properties:
///       name: acctestsbn-conn-example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       sku: Basic
///   exampleApiConnection:
///     type: azure:connections:ApiConnection
///     name: example
///     properties:
///       name: example-connection
///       resourceGroupName: ${exampleResourceGroup.name}
///       managedApiId: ${example.id}
///       displayName: Example 1
///       parameterValues:
///         connectionString: ${exampleNamespace.defaultPrimaryConnectionString}
///       tags:
///         Hello: World
/// variables:
///   example:
///     fn::invoke:
///       function: azure:connections:getManagedApi
///       arguments:
///         name: servicebus
///         location: ${exampleResourceGroup.location}
/// ```
///
/// ## Import
///
/// API Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:connections/apiConnection:ApiConnection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.Web/connections/example-connection
/// ```
///
pub mod api_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiConnectionArgs {
        /// A display name for this API Connection. Defaults to `Service Bus`. Changing this forces a new API Connection to be created.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Managed API which this API Connection is linked to. Changing this forces a new API Connection to be created.
        #[builder(into)]
        pub managed_api_id: pulumi_wasm_rust::Output<String>,
        /// The Name which should be used for this API Connection. Changing this forces a new API Connection to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub parameter_values: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group where this API Connection should exist. Changing this forces a new API Connection to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the API Connection.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApiConnectionResult {
        /// A display name for this API Connection. Defaults to `Service Bus`. Changing this forces a new API Connection to be created.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Managed API which this API Connection is linked to. Changing this forces a new API Connection to be created.
        pub managed_api_id: pulumi_wasm_rust::Output<String>,
        /// The Name which should be used for this API Connection. Changing this forces a new API Connection to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        pub parameter_values: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group where this API Connection should exist. Changing this forces a new API Connection to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the API Connection.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiConnectionArgs) -> ApiConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let managed_api_id_binding = args.managed_api_id.get_inner();
        let name_binding = args.name.get_inner();
        let parameter_values_binding = args.parameter_values.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:connections/apiConnection:ApiConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "managedApiId".into(),
                    value: &managed_api_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameterValues".into(),
                    value: &parameter_values_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "managedApiId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameterValues".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiConnectionResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            managed_api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedApiId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameter_values: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterValues").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
