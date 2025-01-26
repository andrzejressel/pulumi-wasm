pub mod ai_endpoint_iam_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiEndpointIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::vertex::AiEndpointIamMemberCondition>,
        >,
        #[builder(into)]
        pub endpoint: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub member: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub role: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AiEndpointIamMemberResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<super::super::types::vertex::AiEndpointIamMemberCondition>,
        >,
        pub endpoint: pulumi_wasm_rust::Output<String>,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub member: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AiEndpointIamMemberArgs,
    ) -> AiEndpointIamMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_output(context).get_inner();
        let endpoint_binding = args.endpoint.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let member_binding = args.member.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiEndpointIamMember:AiEndpointIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "member".into(),
                    value: &member_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AiEndpointIamMemberResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            member: pulumi_wasm_rust::__private::into_domain(o.extract_field("member")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            role: pulumi_wasm_rust::__private::into_domain(o.extract_field("role")),
        }
    }
}
