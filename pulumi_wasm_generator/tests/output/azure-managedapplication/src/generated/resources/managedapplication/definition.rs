/// Manages a Managed Application Definition.
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
///   exampleDefinition:
///     type: azure:managedapplication:Definition
///     name: example
///     properties:
///       name: examplemanagedapplicationdefinition
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       lockLevel: ReadOnly
///       packageFileUri: https://github.com/Azure/azure-managedapp-samples/raw/master/Managed Application Sample Packages/201-managed-storage-account/managedstorage.zip
///       displayName: TestManagedApplicationDefinition
///       description: Test Managed Application Definition
///       authorizations:
///         - servicePrincipalId: ${current.objectId}
///           roleDefinitionId: a094b430-dad3-424d-ae58-13f72fd72591
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Managed Application Definition can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:managedapplication/definition:Definition example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Solutions/applicationDefinitions/appDefinition1
/// ```
///
pub mod definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefinitionArgs {
        /// One or more `authorization` block defined below.
        #[builder(into, default)]
        pub authorizations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::managedapplication::DefinitionAuthorization>>,
        >,
        /// Specifies the `createUiDefinition` JSON for the backing template with `Microsoft.Solutions/applications` resource.
        #[builder(into, default)]
        pub create_ui_definition: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the managed application definition description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the managed application definition display name.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the managed application lock level. Valid values include `CanNotDelete`, `None`, `ReadOnly`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lock_level: pulumi_wasm_rust::Output<String>,
        /// Specifies the inline main template JSON which has resources to be provisioned.
        #[builder(into, default)]
        pub main_template: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Managed Application Definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Is the package enabled? Defaults to `true`.
        #[builder(into, default)]
        pub package_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the managed application definition package file Uri.
        #[builder(into, default)]
        pub package_file_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Managed Application Definition should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **NOTE:** If either `create_ui_definition` or `main_template` is set they both must be set.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DefinitionResult {
        /// One or more `authorization` block defined below.
        pub authorizations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::managedapplication::DefinitionAuthorization>>,
        >,
        /// Specifies the `createUiDefinition` JSON for the backing template with `Microsoft.Solutions/applications` resource.
        pub create_ui_definition: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the managed application definition description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the managed application definition display name.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the managed application lock level. Valid values include `CanNotDelete`, `None`, `ReadOnly`. Changing this forces a new resource to be created.
        pub lock_level: pulumi_wasm_rust::Output<String>,
        /// Specifies the inline main template JSON which has resources to be provisioned.
        pub main_template: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Managed Application Definition. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Is the package enabled? Defaults to `true`.
        pub package_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the managed application definition package file Uri.
        pub package_file_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Managed Application Definition should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **NOTE:** If either `create_ui_definition` or `main_template` is set they both must be set.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DefinitionArgs) -> DefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authorizations_binding = args.authorizations.get_inner();
        let create_ui_definition_binding = args.create_ui_definition.get_inner();
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let location_binding = args.location.get_inner();
        let lock_level_binding = args.lock_level.get_inner();
        let main_template_binding = args.main_template.get_inner();
        let name_binding = args.name.get_inner();
        let package_enabled_binding = args.package_enabled.get_inner();
        let package_file_uri_binding = args.package_file_uri.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:managedapplication/definition:Definition".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizations".into(),
                    value: &authorizations_binding,
                },
                register_interface::ObjectField {
                    name: "createUiDefinition".into(),
                    value: &create_ui_definition_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "lockLevel".into(),
                    value: &lock_level_binding,
                },
                register_interface::ObjectField {
                    name: "mainTemplate".into(),
                    value: &main_template_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "packageEnabled".into(),
                    value: &package_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "packageFileUri".into(),
                    value: &package_file_uri_binding,
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
                    name: "authorizations".into(),
                },
                register_interface::ResultField {
                    name: "createUiDefinition".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "lockLevel".into(),
                },
                register_interface::ResultField {
                    name: "mainTemplate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "packageEnabled".into(),
                },
                register_interface::ResultField {
                    name: "packageFileUri".into(),
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
        DefinitionResult {
            authorizations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizations").unwrap(),
            ),
            create_ui_definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createUiDefinition").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            lock_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lockLevel").unwrap(),
            ),
            main_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mainTemplate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            package_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageEnabled").unwrap(),
            ),
            package_file_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageFileUri").unwrap(),
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
