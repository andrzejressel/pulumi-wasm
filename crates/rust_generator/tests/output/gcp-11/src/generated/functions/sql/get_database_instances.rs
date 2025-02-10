#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_database_instances {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseInstancesArgs {
        /// To filter out the Cloud SQL instances which are of the specified database version.
        #[builder(into, default)]
        pub database_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resources belong. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// To filter out the Cloud SQL instances which are located in the specified region.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// To filter out the Cloud SQL instances based on the current serving state of the database instance. Supported values include `SQL_INSTANCE_STATE_UNSPECIFIED`, `RUNNABLE`, `SUSPENDED`, `PENDING_DELETE`, `PENDING_CREATE`, `MAINTENANCE`, `FAILED`.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// To filter out the Cloud SQL instances based on the tier(or machine type) of the database instances.
        #[builder(into, default)]
        pub tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// To filter out the Cloud SQL instances which are located in the specified zone. This zone refers to the Compute Engine zone that the instance is currently serving from.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseInstancesResult {
        pub database_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instances: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sql::GetDatabaseInstancesInstance>,
        >,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub state: pulumi_gestalt_rust::Output<Option<String>>,
        pub tier: pulumi_gestalt_rust::Output<Option<String>>,
        pub zone: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDatabaseInstancesArgs,
    ) -> GetDatabaseInstancesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let database_version_binding = args.database_version.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let state_binding = args.state.get_output(context);
        let tier_binding = args.tier.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:sql/getDatabaseInstances:getDatabaseInstances".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseVersion".into(),
                    value: database_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tier".into(),
                    value: tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDatabaseInstancesResult {
            database_version: o.get_field("databaseVersion"),
            id: o.get_field("id"),
            instances: o.get_field("instances"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            state: o.get_field("state"),
            tier: o.get_field("tier"),
            zone: o.get_field("zone"),
        }
    }
}
