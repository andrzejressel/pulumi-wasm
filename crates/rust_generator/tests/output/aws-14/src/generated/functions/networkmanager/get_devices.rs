#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_devices {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDevicesArgs {
        /// ID of the Global Network of the devices to retrieve.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the site of the devices to retrieve.
        #[builder(into, default)]
        pub site_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Restricts the list to the devices with these tags.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDevicesResult {
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// IDs of the devices.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub site_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDevicesArgs,
    ) -> GetDevicesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let global_network_id_binding = args.global_network_id.get_output(context);
        let site_id_binding = args.site_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:networkmanager/getDevices:getDevices".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteId".into(),
                    value: &site_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDevicesResult {
            global_network_id: o.get_field("globalNetworkId"),
            id: o.get_field("id"),
            ids: o.get_field("ids"),
            site_id: o.get_field("siteId"),
            tags: o.get_field("tags"),
        }
    }
}
