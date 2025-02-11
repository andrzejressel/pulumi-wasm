#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dev_environment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDevEnvironmentArgs {
        /// The user-specified alias for the Dev Environment.
        #[builder(into, default)]
        pub alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The system-generated unique ID of the user who created the Dev Environment.
        #[builder(into, default)]
        pub creator_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// - (Required) The system-generated unique ID of the Dev Environment for which you want to view information. To retrieve a list of Dev Environment IDs, use [ListDevEnvironments](https://docs.aws.amazon.com/codecatalyst/latest/APIReference/API_ListDevEnvironments.html).
        #[builder(into)]
        pub env_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the project in the space.
        #[builder(into)]
        pub project_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The source repository that contains the branch to clone into the Dev Environment.
        #[builder(into, default)]
        pub repositories: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::codecatalyst::GetDevEnvironmentRepository,
                >,
            >,
        >,
        /// The name of the space.
        #[builder(into)]
        pub space_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDevEnvironmentResult {
        /// The user-specified alias for the Dev Environment.
        pub alias: pulumi_gestalt_rust::Output<Option<String>>,
        /// The system-generated unique ID of the user who created the Dev Environment.
        pub creator_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub env_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Information about the integrated development environment (IDE) configured for a Dev Environment.
        pub ides: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::codecatalyst::GetDevEnvironmentIde>,
        >,
        /// The amount of time the Dev Environment will run without any activity detected before stopping, in minutes. Only whole integers are allowed. Dev Environments consume compute minutes when running.
        pub inactivity_timeout_minutes: pulumi_gestalt_rust::Output<i32>,
        /// The Amazon EC2 instace type to use for the Dev Environment.
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// The time when the Dev Environment was last updated, in coordinated universal time (UTC) timestamp format as specified in [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339#section-5.6).
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// Information about the amount of storage allocated to the Dev Environment.
        pub persistent_storages: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::codecatalyst::GetDevEnvironmentPersistentStorage,
            >,
        >,
        pub project_name: pulumi_gestalt_rust::Output<String>,
        /// The source repository that contains the branch to clone into the Dev Environment.
        pub repositories: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::codecatalyst::GetDevEnvironmentRepository,
                >,
            >,
        >,
        pub space_name: pulumi_gestalt_rust::Output<String>,
        /// The current status of the Dev Environment. From: PENDING | RUNNING | STARTING | STOPPING | STOPPED | FAILED | DELETING | DELETED.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The reason for the status.
        pub status_reason: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDevEnvironmentArgs,
    ) -> GetDevEnvironmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alias_binding = args.alias.get_output(context);
        let creator_id_binding = args.creator_id.get_output(context);
        let env_id_binding = args.env_id.get_output(context);
        let project_name_binding = args.project_name.get_output(context);
        let repositories_binding = args.repositories.get_output(context);
        let space_name_binding = args.space_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:codecatalyst/getDevEnvironment:getDevEnvironment".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alias".into(),
                    value: &alias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "creatorId".into(),
                    value: &creator_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envId".into(),
                    value: &env_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectName".into(),
                    value: &project_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositories".into(),
                    value: &repositories_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spaceName".into(),
                    value: &space_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDevEnvironmentResult {
            alias: o.get_field("alias"),
            creator_id: o.get_field("creatorId"),
            env_id: o.get_field("envId"),
            id: o.get_field("id"),
            ides: o.get_field("ides"),
            inactivity_timeout_minutes: o.get_field("inactivityTimeoutMinutes"),
            instance_type: o.get_field("instanceType"),
            last_updated_time: o.get_field("lastUpdatedTime"),
            persistent_storages: o.get_field("persistentStorages"),
            project_name: o.get_field("projectName"),
            repositories: o.get_field("repositories"),
            space_name: o.get_field("spaceName"),
            status: o.get_field("status"),
            status_reason: o.get_field("statusReason"),
            tags: o.get_field("tags"),
        }
    }
}
