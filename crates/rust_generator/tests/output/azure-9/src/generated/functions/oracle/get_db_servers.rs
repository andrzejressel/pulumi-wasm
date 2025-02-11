#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_db_servers {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDbServersArgs {
        /// The name of the Cloud Exadata Infrastructure.
        #[builder(into)]
        pub cloud_exadata_infrastructure_name: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The name of the Resource Group where the DB Server exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDbServersResult {
        pub cloud_exadata_infrastructure_name: pulumi_gestalt_rust::Output<String>,
        /// A `db_servers` block as defined below.
        pub db_servers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::oracle::GetDbServersDbServer>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDbServersArgs,
    ) -> GetDbServersResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloud_exadata_infrastructure_name_binding = args
            .cloud_exadata_infrastructure_name
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:oracle/getDbServers:getDbServers".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudExadataInfrastructureName".into(),
                    value: &cloud_exadata_infrastructure_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDbServersResult {
            cloud_exadata_infrastructure_name: o
                .get_field("cloudExadataInfrastructureName"),
            db_servers: o.get_field("dbServers"),
            id: o.get_field("id"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
