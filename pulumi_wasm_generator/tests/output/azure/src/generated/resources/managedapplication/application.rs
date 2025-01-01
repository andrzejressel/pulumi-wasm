/// Manages a Managed Application.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Managed Application can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:managedapplication/application:Application example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Solutions/applications/app1
/// ```
///
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The application definition ID to deploy.
        #[builder(into, default)]
        pub application_definition_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The kind of the managed application to deploy. Possible values are `MarketPlace` and `ServiceCatalog`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub kind: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the target resource group where all the resources deployed by the managed application will reside. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managed_resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Managed Application. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The parameter values to pass to the Managed Application. This field is a JSON object that allows you to assign parameters to this Managed Application.
        #[builder(into, default)]
        pub parameter_values: pulumi_wasm_rust::Output<Option<String>>,
        /// One `plan` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::managedapplication::ApplicationPlan>,
        >,
        /// The name of the Resource Group where the Managed Application should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// The application definition ID to deploy.
        pub application_definition_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The kind of the managed application to deploy. Possible values are `MarketPlace` and `ServiceCatalog`. Changing this forces a new resource to be created.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the target resource group where all the resources deployed by the managed application will reside. Changing this forces a new resource to be created.
        pub managed_resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Managed Application. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name and value pairs that define the managed application outputs.
        pub outputs: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The parameter values to pass to the Managed Application. This field is a JSON object that allows you to assign parameters to this Managed Application.
        pub parameter_values: pulumi_wasm_rust::Output<String>,
        /// One `plan` block as defined below. Changing this forces a new resource to be created.
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::managedapplication::ApplicationPlan>,
        >,
        /// The name of the Resource Group where the Managed Application should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApplicationArgs) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_definition_id_binding = args
            .application_definition_id
            .get_inner();
        let kind_binding = args.kind.get_inner();
        let location_binding = args.location.get_inner();
        let managed_resource_group_name_binding = args
            .managed_resource_group_name
            .get_inner();
        let name_binding = args.name.get_inner();
        let parameter_values_binding = args.parameter_values.get_inner();
        let plan_binding = args.plan.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:managedapplication/application:Application".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationDefinitionId".into(),
                    value: &application_definition_id_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedResourceGroupName".into(),
                    value: &managed_resource_group_name_binding,
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
                    name: "plan".into(),
                    value: &plan_binding,
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
                    name: "applicationDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedResourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outputs".into(),
                },
                register_interface::ResultField {
                    name: "parameterValues".into(),
                },
                register_interface::ResultField {
                    name: "plan".into(),
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
        ApplicationResult {
            application_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationDefinitionId").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedResourceGroupName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outputs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputs").unwrap(),
            ),
            parameter_values: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterValues").unwrap(),
            ),
            plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plan").unwrap(),
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
