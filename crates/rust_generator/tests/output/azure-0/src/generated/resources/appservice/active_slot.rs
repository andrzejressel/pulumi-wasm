/// Promotes an App Service Slot to Production within an App Service.
///
/// !> **NOTE:** This resource has been deprecated in version 5.0 of the provider and will be removed in version 6.0. Please use `azure.appservice.WebAppActiveSlot` resource instead.
///
/// > **Note:** When using Slots - the `app_settings`, `connection_string` and `site_config` blocks on the `azure.appservice.AppService` resource will be overwritten when promoting a Slot using the `azure.appservice.ActiveSlot` resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   server:
///     type: random:RandomId
///   example:
///     type: azure:core:ResourceGroup
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///   exampleAppService:
///     type: azure:appservice:AppService
///     name: example
///   exampleSlot:
///     type: azure:appservice:Slot
///     name: example
///   exampleActiveSlot:
///     type: azure:appservice:ActiveSlot
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       appServiceName: ${exampleAppService.name}
///       appServiceSlotName: ${exampleSlot.name}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod active_slot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActiveSlotArgs {
        /// The name of the App Service within which the Slot exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the App Service Slot which should be promoted to the Production Slot within the App Service.
        #[builder(into)]
        pub app_service_slot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the App Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ActiveSlotResult {
        /// The name of the App Service within which the Slot exists. Changing this forces a new resource to be created.
        pub app_service_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the App Service Slot which should be promoted to the Production Slot within the App Service.
        pub app_service_slot_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the App Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ActiveSlotArgs,
    ) -> ActiveSlotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_service_name_binding = args.app_service_name.get_output(context);
        let app_service_slot_name_binding = args
            .app_service_slot_name
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/activeSlot:ActiveSlot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appServiceName".into(),
                    value: app_service_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appServiceSlotName".into(),
                    value: app_service_slot_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ActiveSlotResult {
            app_service_name: o.get_field("appServiceName"),
            app_service_slot_name: o.get_field("appServiceSlotName"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
