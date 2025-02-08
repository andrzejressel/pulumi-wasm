#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_lambda_function_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLambdaFunctionAssociationArgs {
        /// ARN of the Lambda Function, omitting any version or alias qualifier.
        #[builder(into)]
        pub function_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLambdaFunctionAssociationResult {
        pub function_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetLambdaFunctionAssociationArgs,
    ) -> GetLambdaFunctionAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let function_arn_binding = args.function_arn.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getLambdaFunctionAssociation:getLambdaFunctionAssociation"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "functionArn".into(),
                    value: &function_arn_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLambdaFunctionAssociationResult {
            function_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("functionArn"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
        }
    }
}
