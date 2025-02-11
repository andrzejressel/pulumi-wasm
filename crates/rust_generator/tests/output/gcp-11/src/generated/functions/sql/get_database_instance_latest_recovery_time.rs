#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_database_instance_latest_recovery_time {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseInstanceLatestRecoveryTimeArgs {
        /// The name of the instance.
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseInstanceLatestRecoveryTimeResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the instance.
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// Timestamp, identifies the latest recovery time of the source instance.
        pub latest_recovery_time: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDatabaseInstanceLatestRecoveryTimeArgs,
    ) -> GetDatabaseInstanceLatestRecoveryTimeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_binding = args.instance.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:sql/getDatabaseInstanceLatestRecoveryTime:getDatabaseInstanceLatestRecoveryTime"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: &instance_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDatabaseInstanceLatestRecoveryTimeResult {
            id: o.get_field("id"),
            instance: o.get_field("instance"),
            latest_recovery_time: o.get_field("latestRecoveryTime"),
            project: o.get_field("project"),
        }
    }
}
