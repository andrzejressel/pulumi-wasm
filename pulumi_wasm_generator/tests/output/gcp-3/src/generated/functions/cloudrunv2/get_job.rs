pub mod get_job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetJobArgs {
        /// The location of the instance. eg us-central1
        ///
        /// - - -
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Cloud Run v2 Job.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetJobResult {
        pub annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub binary_authorizations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetJobBinaryAuthorization>,
        >,
        pub client: pulumi_wasm_rust::Output<String>,
        pub client_version: pulumi_wasm_rust::Output<String>,
        pub conditions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetJobCondition>,
        >,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub creator: pulumi_wasm_rust::Output<String>,
        pub delete_time: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub execution_count: pulumi_wasm_rust::Output<i32>,
        pub expire_time: pulumi_wasm_rust::Output<String>,
        pub generation: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub last_modifier: pulumi_wasm_rust::Output<String>,
        pub latest_created_executions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetJobLatestCreatedExecution>,
        >,
        pub launch_stage: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub observed_generation: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        pub run_execution_token: pulumi_wasm_rust::Output<String>,
        pub start_execution_token: pulumi_wasm_rust::Output<String>,
        pub templates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetJobTemplate>,
        >,
        pub terminal_conditions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetJobTerminalCondition>,
        >,
        pub uid: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetJobArgs) -> GetJobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "binaryAuthorizations".into(),
                },
                register_interface::ResultField {
                    name: "client".into(),
                },
                register_interface::ResultField {
                    name: "clientVersion".into(),
                },
                register_interface::ResultField {
                    name: "conditions".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "creator".into(),
                },
                register_interface::ResultField {
                    name: "deleteTime".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "executionCount".into(),
                },
                register_interface::ResultField {
                    name: "expireTime".into(),
                },
                register_interface::ResultField {
                    name: "generation".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "lastModifier".into(),
                },
                register_interface::ResultField {
                    name: "latestCreatedExecutions".into(),
                },
                register_interface::ResultField {
                    name: "launchStage".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "observedGeneration".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "runExecutionToken".into(),
                },
                register_interface::ResultField {
                    name: "startExecutionToken".into(),
                },
                register_interface::ResultField {
                    name: "templates".into(),
                },
                register_interface::ResultField {
                    name: "terminalConditions".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetJobResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            binary_authorizations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("binaryAuthorizations").unwrap(),
            ),
            client: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("client").unwrap(),
            ),
            client_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientVersion").unwrap(),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditions").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            creator: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creator").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            execution_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionCount").unwrap(),
            ),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expireTime").unwrap(),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("generation").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            last_modifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifier").unwrap(),
            ),
            latest_created_executions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestCreatedExecutions").unwrap(),
            ),
            launch_stage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchStage").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            observed_generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("observedGeneration").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            run_execution_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runExecutionToken").unwrap(),
            ),
            start_execution_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startExecutionToken").unwrap(),
            ),
            templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templates").unwrap(),
            ),
            terminal_conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminalConditions").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
