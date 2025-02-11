#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_models {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetModelsArgs {
        /// Customization type to filter on. Valid values are `FINE_TUNING`.
        #[builder(into, default)]
        pub by_customization_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Inference type to filter on. Valid values are `ON_DEMAND` and `PROVISIONED`.
        #[builder(into, default)]
        pub by_inference_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Output modality to filter on. Valid values are `TEXT`, `IMAGE`, and `EMBEDDING`.
        #[builder(into, default)]
        pub by_output_modality: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Model provider to filter on.
        #[builder(into, default)]
        pub by_provider: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetModelsResult {
        pub by_customization_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub by_inference_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub by_output_modality: pulumi_gestalt_rust::Output<Option<String>>,
        pub by_provider: pulumi_gestalt_rust::Output<Option<String>>,
        /// AWS region.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of model summary objects. See `model_summaries`.
        pub model_summaries: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::bedrockfoundation::GetModelsModelSummary>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetModelsArgs,
    ) -> GetModelsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let by_customization_type_binding = args
            .by_customization_type
            .get_output(context);
        let by_inference_type_binding = args.by_inference_type.get_output(context);
        let by_output_modality_binding = args.by_output_modality.get_output(context);
        let by_provider_binding = args.by_provider.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:bedrockfoundation/getModels:getModels".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "byCustomizationType".into(),
                    value: &by_customization_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "byInferenceType".into(),
                    value: &by_inference_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "byOutputModality".into(),
                    value: &by_output_modality_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "byProvider".into(),
                    value: &by_provider_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetModelsResult {
            by_customization_type: o.get_field("byCustomizationType"),
            by_inference_type: o.get_field("byInferenceType"),
            by_output_modality: o.get_field("byOutputModality"),
            by_provider: o.get_field("byProvider"),
            id: o.get_field("id"),
            model_summaries: o.get_field("modelSummaries"),
        }
    }
}
