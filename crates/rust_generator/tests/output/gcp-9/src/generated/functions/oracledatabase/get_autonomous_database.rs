#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_autonomous_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAutonomousDatabaseArgs {
        /// The ID of the AutonomousDatabase.
        #[builder(into)]
        pub autonomous_database_id: pulumi_gestalt_rust::InputOrOutput<String>,
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
    pub struct GetAutonomousDatabaseResult {
        pub admin_password: pulumi_gestalt_rust::Output<String>,
        pub autonomous_database_id: pulumi_gestalt_rust::Output<String>,
        pub cidr: pulumi_gestalt_rust::Output<String>,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub database: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub entitlement_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub properties: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::oracledatabase::GetAutonomousDatabaseProperty,
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
        args: GetAutonomousDatabaseArgs,
    ) -> GetAutonomousDatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autonomous_database_id_binding = args
            .autonomous_database_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:oracledatabase/getAutonomousDatabase:getAutonomousDatabase"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autonomousDatabaseId".into(),
                    value: autonomous_database_id_binding.get_id(),
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
        GetAutonomousDatabaseResult {
            admin_password: o.get_field("adminPassword"),
            autonomous_database_id: o.get_field("autonomousDatabaseId"),
            cidr: o.get_field("cidr"),
            create_time: o.get_field("createTime"),
            database: o.get_field("database"),
            deletion_protection: o.get_field("deletionProtection"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            entitlement_id: o.get_field("entitlementId"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            properties: o.get_field("properties"),
            pulumi_labels: o.get_field("pulumiLabels"),
        }
    }
}
