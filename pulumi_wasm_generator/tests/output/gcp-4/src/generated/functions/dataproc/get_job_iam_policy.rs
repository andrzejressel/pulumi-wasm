pub mod get_job_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetJobIamPolicyArgs {
        /// The name or relative resource id of the job to manage IAM policies for.
        #[builder(into)]
        pub job_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetJobIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub job_id: pulumi_wasm_rust::Output<String>,
        /// (Computed) The policy data
        pub policy_data: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub region: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetJobIamPolicyArgs) -> GetJobIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let job_id_binding = args.job_id.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:dataproc/getJobIamPolicy:getJobIamPolicy".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "jobId".into(),
                    value: &job_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "jobId".into(),
                },
                register_interface::ResultField {
                    name: "policyData".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetJobIamPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            job_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobId").unwrap(),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyData").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
        }
    }
}
