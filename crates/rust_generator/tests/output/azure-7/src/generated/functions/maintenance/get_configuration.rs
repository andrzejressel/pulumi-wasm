#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationArgs {
        /// Specifies the name of the Maintenance Configuration.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where this Maintenance Configuration exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The in guest user patch mode.
        pub in_guest_user_patch_mode: pulumi_gestalt_rust::Output<String>,
        /// An `install_patches` block as defined below.
        pub install_patches: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::maintenance::GetConfigurationInstallPatch>,
        >,
        /// The Azure location where the resource exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The properties assigned to the resource.
        pub properties: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The scope of the Maintenance Configuration.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The visibility of the Maintenance Configuration.
        pub visibility: pulumi_gestalt_rust::Output<String>,
        /// A `window` block as defined below.
        pub windows: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::maintenance::GetConfigurationWindow>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConfigurationArgs,
    ) -> GetConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:maintenance/getConfiguration:getConfiguration".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConfigurationResult {
            id: o.get_field("id"),
            in_guest_user_patch_mode: o.get_field("inGuestUserPatchMode"),
            install_patches: o.get_field("installPatches"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            properties: o.get_field("properties"),
            resource_group_name: o.get_field("resourceGroupName"),
            scope: o.get_field("scope"),
            tags: o.get_field("tags"),
            visibility: o.get_field("visibility"),
            windows: o.get_field("windows"),
        }
    }
}
