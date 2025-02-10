#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cloud_exadata_infrastructure {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCloudExadataInfrastructureArgs {
        /// The ID of the ExadataInfrastructure.
        #[builder(into)]
        pub cloud_exadata_infrastructure_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project to which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCloudExadataInfrastructureResult {
        pub cloud_exadata_infrastructure_id: pulumi_gestalt_rust::Output<String>,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub entitlement_id: pulumi_gestalt_rust::Output<String>,
        pub gcp_oracle_zone: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub properties: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::oracledatabase::GetCloudExadataInfrastructureProperty,
            >,
        >,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCloudExadataInfrastructureArgs,
    ) -> GetCloudExadataInfrastructureResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloud_exadata_infrastructure_id_binding = args
            .cloud_exadata_infrastructure_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:oracledatabase/getCloudExadataInfrastructure:getCloudExadataInfrastructure"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudExadataInfrastructureId".into(),
                    value: cloud_exadata_infrastructure_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCloudExadataInfrastructureResult {
            cloud_exadata_infrastructure_id: o.get_field("cloudExadataInfrastructureId"),
            create_time: o.get_field("createTime"),
            deletion_protection: o.get_field("deletionProtection"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            entitlement_id: o.get_field("entitlementId"),
            gcp_oracle_zone: o.get_field("gcpOracleZone"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            properties: o.get_field("properties"),
            pulumi_labels: o.get_field("pulumiLabels"),
        }
    }
}
