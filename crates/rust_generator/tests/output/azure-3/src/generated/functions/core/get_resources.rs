#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resources {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourcesArgs {
        /// The name of the Resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which the resource has to have in order to be included in the result.
        #[builder(into, default)]
        pub required_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource group where the Resources are located.
        #[builder(into, default)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Resource Type of the Resources you want to list (e.g. `Microsoft.Network/virtualNetworks`). A resource type's name follows the format: `{resource-provider}/{resource-type}`. The resource type for a key vault is `Microsoft.KeyVault/vaults`. A full list of available Resource Providers can be found [here](https://docs.microsoft.com/azure/azure-resource-manager/azure-services-resource-providers). A full list of Resources Types can be found [here](https://learn.microsoft.com/en-us/azure/templates/#find-resources).
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetResourcesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of this Resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub required_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Resource Group in which this Resource exists.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `resource` blocks as defined below.
        pub resources: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::core::GetResourcesResource>,
        >,
        /// The type of this Resource. (e.g. `Microsoft.Network/virtualNetworks`).
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResourcesArgs,
    ) -> GetResourcesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let required_tags_binding = args.required_tags.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:core/getResources:getResources".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requiredTags".into(),
                    value: required_tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResourcesResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            required_tags: o.get_field("requiredTags"),
            resource_group_name: o.get_field("resourceGroupName"),
            resources: o.get_field("resources"),
            type_: o.get_field("type"),
        }
    }
}
