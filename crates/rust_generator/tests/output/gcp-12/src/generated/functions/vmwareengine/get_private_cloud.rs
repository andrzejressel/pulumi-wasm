#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_private_cloud {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrivateCloudArgs {
        /// Location of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPrivateCloudResult {
        pub deletion_delay_hours: pulumi_gestalt_rust::Output<i32>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub hcxes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetPrivateCloudHcx>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub management_clusters: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::vmwareengine::GetPrivateCloudManagementCluster,
            >,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetPrivateCloudNetworkConfig>,
        >,
        pub nsxes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetPrivateCloudNsx>,
        >,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub send_deletion_delay_hours_if_zero: pulumi_gestalt_rust::Output<bool>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub uid: pulumi_gestalt_rust::Output<String>,
        pub vcenters: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetPrivateCloudVcenter>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPrivateCloudArgs,
    ) -> GetPrivateCloudResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:vmwareengine/getPrivateCloud:getPrivateCloud".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPrivateCloudResult {
            deletion_delay_hours: o.get_field("deletionDelayHours"),
            description: o.get_field("description"),
            hcxes: o.get_field("hcxes"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            management_clusters: o.get_field("managementClusters"),
            name: o.get_field("name"),
            network_configs: o.get_field("networkConfigs"),
            nsxes: o.get_field("nsxes"),
            project: o.get_field("project"),
            send_deletion_delay_hours_if_zero: o
                .get_field("sendDeletionDelayHoursIfZero"),
            state: o.get_field("state"),
            type_: o.get_field("type"),
            uid: o.get_field("uid"),
            vcenters: o.get_field("vcenters"),
        }
    }
}
