#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetJobArgs {
        /// The location of the instance. eg us-central1
        ///
        /// - - -
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud Run v2 Job.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetJobResult {
        pub annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub binary_authorizations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetJobBinaryAuthorization>,
        >,
        pub client: pulumi_gestalt_rust::Output<String>,
        pub client_version: pulumi_gestalt_rust::Output<String>,
        pub conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetJobCondition>,
        >,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub creator: pulumi_gestalt_rust::Output<String>,
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub execution_count: pulumi_gestalt_rust::Output<i32>,
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        pub generation: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub last_modifier: pulumi_gestalt_rust::Output<String>,
        pub latest_created_executions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetJobLatestCreatedExecution>,
        >,
        pub launch_stage: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub observed_generation: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        pub run_execution_token: pulumi_gestalt_rust::Output<String>,
        pub start_execution_token: pulumi_gestalt_rust::Output<String>,
        pub templates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetJobTemplate>,
        >,
        pub terminal_conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetJobTerminalCondition>,
        >,
        pub uid: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetJobArgs,
    ) -> GetJobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudrunv2/getJob:getJob".into(),
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
        GetJobResult {
            annotations: o.get_field("annotations"),
            binary_authorizations: o.get_field("binaryAuthorizations"),
            client: o.get_field("client"),
            client_version: o.get_field("clientVersion"),
            conditions: o.get_field("conditions"),
            create_time: o.get_field("createTime"),
            creator: o.get_field("creator"),
            delete_time: o.get_field("deleteTime"),
            deletion_protection: o.get_field("deletionProtection"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            execution_count: o.get_field("executionCount"),
            expire_time: o.get_field("expireTime"),
            generation: o.get_field("generation"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            last_modifier: o.get_field("lastModifier"),
            latest_created_executions: o.get_field("latestCreatedExecutions"),
            launch_stage: o.get_field("launchStage"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            observed_generation: o.get_field("observedGeneration"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            run_execution_token: o.get_field("runExecutionToken"),
            start_execution_token: o.get_field("startExecutionToken"),
            templates: o.get_field("templates"),
            terminal_conditions: o.get_field("terminalConditions"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
