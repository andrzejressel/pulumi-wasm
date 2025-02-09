#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_model {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetModelArgs {
        /// Model identifier.
        #[builder(into)]
        pub model_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetModelResult {
        /// Customizations that the model supports.
        pub customizations_supporteds: pulumi_gestalt_rust::Output<Vec<String>>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Inference types that the model supports.
        pub inference_types_supporteds: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Input modalities that the model supports.
        pub input_modalities: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Model ARN.
        pub model_arn: pulumi_gestalt_rust::Output<String>,
        pub model_id: pulumi_gestalt_rust::Output<String>,
        /// Model name.
        pub model_name: pulumi_gestalt_rust::Output<String>,
        /// Output modalities that the model supports.
        pub output_modalities: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Model provider name.
        pub provider_name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the model supports streaming.
        pub response_streaming_supported: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetModelArgs,
    ) -> GetModelResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let model_id_binding_1 = args.model_id.get_output(context);
        let model_id_binding = model_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:bedrockfoundation/getModel:getModel".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "modelId".into(),
                    value: &model_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetModelResult {
            customizations_supporteds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customizationsSupporteds"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            inference_types_supporteds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inferenceTypesSupporteds"),
            ),
            input_modalities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputModalities"),
            ),
            model_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modelArn"),
            ),
            model_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modelId"),
            ),
            model_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modelName"),
            ),
            output_modalities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outputModalities"),
            ),
            provider_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("providerName"),
            ),
            response_streaming_supported: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("responseStreamingSupported"),
            ),
        }
    }
}
