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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDbServersArgs,
    ) -> GetDbServersResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cloud_exadata_infrastructure_name_binding = args
            .cloud_exadata_infrastructure_name
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:oracle/getDbServers:getDbServers".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudExadataInfrastructureName".into(),
                    value: &cloud_exadata_infrastructure_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDbServersResult {
            cloud_exadata_infrastructure_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cloudExadataInfrastructureName"),
            ),
            db_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbServers"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
