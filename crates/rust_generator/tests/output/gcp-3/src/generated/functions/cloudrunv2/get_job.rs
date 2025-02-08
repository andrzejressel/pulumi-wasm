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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetJobArgs,
    ) -> GetJobResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudrunv2/getJob:getJob".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetJobResult {
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            binary_authorizations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("binaryAuthorizations"),
            ),
            client: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("client"),
            ),
            client_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientVersion"),
            ),
            conditions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("conditions"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            creator: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creator"),
            ),
            delete_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteTime"),
            ),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            execution_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("executionCount"),
            ),
            expire_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expireTime"),
            ),
            generation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("generation"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            last_modifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifier"),
            ),
            latest_created_executions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("latestCreatedExecutions"),
            ),
            launch_stage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("launchStage"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            observed_generation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("observedGeneration"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            reconciling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            run_execution_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runExecutionToken"),
            ),
            start_execution_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startExecutionToken"),
            ),
            templates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templates"),
            ),
            terminal_conditions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("terminalConditions"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
