#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_volume_group_sap_hana {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVolumeGroupSapHanaArgs {
        /// Name of the account where the application volume group belong to.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this Application Volume Group for SAP HANA application.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Application Volume Group exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVolumeGroupSapHanaResult {
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The application identifier.
        pub application_identifier: pulumi_gestalt_rust::Output<String>,
        /// Volume group description.
        pub group_description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Application Volume Group exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of this volume.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `volume` block as defined below.
        pub volumes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::netapp::GetVolumeGroupSapHanaVolume>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVolumeGroupSapHanaArgs,
    ) -> GetVolumeGroupSapHanaResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:netapp/getVolumeGroupSapHana:getVolumeGroupSapHana".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: account_name_binding.get_id(),
                },
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
        GetVolumeGroupSapHanaResult {
            account_name: o.get_field("accountName"),
            application_identifier: o.get_field("applicationIdentifier"),
            group_description: o.get_field("groupDescription"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            volumes: o.get_field("volumes"),
        }
    }
}
