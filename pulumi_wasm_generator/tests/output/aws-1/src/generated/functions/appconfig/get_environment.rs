pub mod get_environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnvironmentArgs {
        /// ID of the AppConfig Application to which this Environment belongs.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the AppConfig Environment.
        #[builder(into)]
        pub environment_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetEnvironmentResult {
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the environment.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the environment.
        pub description: pulumi_wasm_rust::Output<String>,
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Set of Amazon CloudWatch alarms to monitor during the deployment process.
        pub monitors: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appconfig::GetEnvironmentMonitor>,
        >,
        /// Name of the environment.
        pub name: pulumi_wasm_rust::Output<String>,
        /// State of the environment. Possible values are `READY_FOR_DEPLOYMENT`, `DEPLOYING`, `ROLLING_BACK`
        /// or `ROLLED_BACK`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Map of tags for the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEnvironmentArgs,
    ) -> GetEnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let environment_id_binding = args.environment_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:appconfig/getEnvironment:getEnvironment".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEnvironmentResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            environment_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("environmentId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            monitors: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("monitors"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
