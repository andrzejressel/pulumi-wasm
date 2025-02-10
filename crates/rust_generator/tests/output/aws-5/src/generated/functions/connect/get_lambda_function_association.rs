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
        context: &pulumi_gestalt_rust::Context,
        args: GetLambdaFunctionAssociationArgs,
    ) -> GetLambdaFunctionAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let function_arn_binding = args.function_arn.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getLambdaFunctionAssociation:getLambdaFunctionAssociation"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "functionArn".into(),
                    value: function_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLambdaFunctionAssociationResult {
            function_arn: o.get_field("functionArn"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
        }
    }
}
