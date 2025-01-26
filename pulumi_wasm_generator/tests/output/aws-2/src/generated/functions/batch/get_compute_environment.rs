pub mod get_compute_environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetComputeEnvironmentArgs {
        /// Name of the Batch Compute Environment
        #[builder(into)]
        pub compute_environment_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of resource tags
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetComputeEnvironmentResult {
        /// ARN of the compute environment.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub compute_environment_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the underlying Amazon ECS cluster used by the compute environment.
        pub ecs_cluster_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the IAM role that allows AWS Batch to make calls to other AWS services on your behalf.
        pub service_role: pulumi_wasm_rust::Output<String>,
        /// State of the compute environment (for example, `ENABLED` or `DISABLED`). If the state is `ENABLED`, then the compute environment accepts jobs from a queue and can scale out automatically based on queues.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Current status of the compute environment (for example, `CREATING` or `VALID`).
        pub status: pulumi_wasm_rust::Output<String>,
        /// Short, human-readable string to provide additional details about the current status of the compute environment.
        pub status_reason: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of the compute environment (for example, `MANAGED` or `UNMANAGED`).
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Specifies the infrastructure update policy for the compute environment.
        pub update_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::batch::GetComputeEnvironmentUpdatePolicy>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetComputeEnvironmentArgs,
    ) -> GetComputeEnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let compute_environment_name_binding = args
            .compute_environment_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:batch/getComputeEnvironment:getComputeEnvironment".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "computeEnvironmentName".into(),
                    value: &compute_environment_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetComputeEnvironmentResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            compute_environment_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("computeEnvironmentName"),
            ),
            ecs_cluster_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ecsClusterArn"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            service_role: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceRole"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            status_reason: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statusReason"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            update_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updatePolicies"),
            ),
        }
    }
}
