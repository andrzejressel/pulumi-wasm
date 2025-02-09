#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseArgs {
        /// The name of the database's spanner instance.
        ///
        /// - - -
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the spanner database.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseResult {
        pub database_dialect: pulumi_gestalt_rust::Output<String>,
        pub ddls: pulumi_gestalt_rust::Output<Vec<String>>,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        pub enable_drop_protection: pulumi_gestalt_rust::Output<bool>,
        pub encryption_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::spanner::GetDatabaseEncryptionConfig>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub version_retention_period: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDatabaseArgs,
    ) -> GetDatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_binding = args.instance.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:spanner/getDatabase:getDatabase".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: instance_binding.get_id(),
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
        GetDatabaseResult {
            database_dialect: o.get_field("databaseDialect"),
            ddls: o.get_field("ddls"),
            deletion_protection: o.get_field("deletionProtection"),
            enable_drop_protection: o.get_field("enableDropProtection"),
            encryption_configs: o.get_field("encryptionConfigs"),
            id: o.get_field("id"),
            instance: o.get_field("instance"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            state: o.get_field("state"),
            version_retention_period: o.get_field("versionRetentionPeriod"),
        }
    }
}
