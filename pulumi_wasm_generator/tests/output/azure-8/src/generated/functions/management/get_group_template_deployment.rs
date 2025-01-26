pub mod get_group_template_deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupTemplateDeploymentArgs {
        /// The ID of the Management Group to which this template was applied.
        #[builder(into)]
        pub management_group_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of this Management Group Template Deployment.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupTemplateDeploymentResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub management_group_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The JSON Content of the Outputs of the ARM Template Deployment.
        pub output_content: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetGroupTemplateDeploymentArgs,
    ) -> GetGroupTemplateDeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let management_group_id_binding = args
            .management_group_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:management/getGroupTemplateDeployment:getGroupTemplateDeployment"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managementGroupId".into(),
                    value: &management_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGroupTemplateDeploymentResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            management_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managementGroupId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            output_content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("outputContent"),
            ),
        }
    }
}
