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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The application definition ID to deploy.
        #[builder(into, default)]
        pub application_definition_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The kind of the managed application to deploy. Possible values are `MarketPlace` and `ServiceCatalog`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the target resource group where all the resources deployed by the managed application will reside. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managed_resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Managed Application. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parameter values to pass to the Managed Application. This field is a JSON object that allows you to assign parameters to this Managed Application.
        #[builder(into, default)]
        pub parameter_values: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One `plan` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::managedapplication::ApplicationPlan>,
        >,
        /// The name of the Resource Group where the Managed Application should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// The application definition ID to deploy.
        pub application_definition_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The kind of the managed application to deploy. Possible values are `MarketPlace` and `ServiceCatalog`. Changing this forces a new resource to be created.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the target resource group where all the resources deployed by the managed application will reside. Changing this forces a new resource to be created.
        pub managed_resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Managed Application. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name and value pairs that define the managed application outputs.
        pub outputs: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The parameter values to pass to the Managed Application. This field is a JSON object that allows you to assign parameters to this Managed Application.
        pub parameter_values: pulumi_gestalt_rust::Output<String>,
        /// One `plan` block as defined below. Changing this forces a new resource to be created.
        pub plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::managedapplication::ApplicationPlan>,
        >,
        /// The name of the Resource Group where the Managed Application should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
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
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_definition_id_binding = args
            .application_definition_id
            .get_output(context);
        let kind_binding = args.kind.get_output(context);
        let location_binding = args.location.get_output(context);
        let managed_resource_group_name_binding = args
            .managed_resource_group_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let parameter_values_binding = args.parameter_values.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:managedapplication/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationDefinitionId".into(),
                    value: application_definition_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: kind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedResourceGroupName".into(),
                    value: managed_resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameterValues".into(),
                    value: parameter_values_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: plan_binding.get_id(),
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
        ApplicationResult {
            application_definition_id: o.get_field("applicationDefinitionId"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            managed_resource_group_name: o.get_field("managedResourceGroupName"),
            name: o.get_field("name"),
            outputs: o.get_field("outputs"),
            parameter_values: o.get_field("parameterValues"),
            plan: o.get_field("plan"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
