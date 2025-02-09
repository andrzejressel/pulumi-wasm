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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefinitionArgs {
        /// One or more `authorization` block defined below.
        #[builder(into, default)]
        pub authorizations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::managedapplication::DefinitionAuthorization>>,
        >,
        /// Specifies the `createUiDefinition` JSON for the backing template with `Microsoft.Solutions/applications` resource.
        #[builder(into, default)]
        pub create_ui_definition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the managed application definition description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the managed application definition display name.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the managed application lock level. Valid values include `CanNotDelete`, `None`, `ReadOnly`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lock_level: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the inline main template JSON which has resources to be provisioned.
        #[builder(into, default)]
        pub main_template: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Managed Application Definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is the package enabled? Defaults to `true`.
        #[builder(into, default)]
        pub package_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the managed application definition package file Uri.
        #[builder(into, default)]
        pub package_file_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Managed Application Definition should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **NOTE:** If either `create_ui_definition` or `main_template` is set they both must be set.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DefinitionResult {
        /// One or more `authorization` block defined below.
        pub authorizations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::managedapplication::DefinitionAuthorization>>,
        >,
        /// Specifies the `createUiDefinition` JSON for the backing template with `Microsoft.Solutions/applications` resource.
        pub create_ui_definition: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the managed application definition description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the managed application definition display name.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the managed application lock level. Valid values include `CanNotDelete`, `None`, `ReadOnly`. Changing this forces a new resource to be created.
        pub lock_level: pulumi_gestalt_rust::Output<String>,
        /// Specifies the inline main template JSON which has resources to be provisioned.
        pub main_template: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Managed Application Definition. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Is the package enabled? Defaults to `true`.
        pub package_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the managed application definition package file Uri.
        pub package_file_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Managed Application Definition should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        ///
        /// > **NOTE:** If either `create_ui_definition` or `main_template` is set they both must be set.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefinitionArgs,
    ) -> DefinitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorizations_binding = args.authorizations.get_output(context);
        let create_ui_definition_binding = args.create_ui_definition.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let lock_level_binding = args.lock_level.get_output(context);
        let main_template_binding = args.main_template.get_output(context);
        let name_binding = args.name.get_output(context);
        let package_enabled_binding = args.package_enabled.get_output(context);
        let package_file_uri_binding = args.package_file_uri.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:managedapplication/definition:Definition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizations".into(),
                    value: authorizations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createUiDefinition".into(),
                    value: create_ui_definition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lockLevel".into(),
                    value: lock_level_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mainTemplate".into(),
                    value: main_template_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageEnabled".into(),
                    value: package_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageFileUri".into(),
                    value: package_file_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefinitionResult {
            authorizations: o.get_field("authorizations"),
            create_ui_definition: o.get_field("createUiDefinition"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            location: o.get_field("location"),
            lock_level: o.get_field("lockLevel"),
            main_template: o.get_field("mainTemplate"),
            name: o.get_field("name"),
            package_enabled: o.get_field("packageEnabled"),
            package_file_uri: o.get_field("packageFileUri"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
