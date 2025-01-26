pub mod get_resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceArgs {
        /// Identifier of the CloudFormation resource type. For example, `vpc-12345678`.
        #[builder(into)]
        pub identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// ARN of the IAM Role to assume for operations.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// CloudFormation resource type name. For example, `AWS::EC2::VPC`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Identifier of the CloudFormation resource type version.
        #[builder(into, default)]
        pub type_version_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetResourceResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identifier: pulumi_wasm_rust::Output<String>,
        /// JSON string matching the CloudFormation resource type schema with current configuration.
        pub properties: pulumi_wasm_rust::Output<String>,
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        pub type_name: pulumi_wasm_rust::Output<String>,
        pub type_version_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetResourceArgs,
    ) -> GetResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identifier_binding = args.identifier.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let type_name_binding = args.type_name.get_output(context).get_inner();
        let type_version_id_binding = args
            .type_version_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudcontrol/getResource:getResource".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "typeName".into(),
                    value: &type_name_binding,
                },
                register_interface::ObjectField {
                    name: "typeVersionId".into(),
                    value: &type_version_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResourceResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identifier"),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("properties"),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            type_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("typeName"),
            ),
            type_version_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("typeVersionId"),
            ),
        }
    }
}
