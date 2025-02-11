#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_inference_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInferenceProfileArgs {
        /// Inference Profile identifier.
        #[builder(into)]
        pub inference_profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInferenceProfileResult {
        /// The time at which the inference profile was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The description of the inference profile.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the inference profile.
        pub inference_profile_arn: pulumi_gestalt_rust::Output<String>,
        pub inference_profile_id: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the inference profile.
        pub inference_profile_name: pulumi_gestalt_rust::Output<String>,
        /// A list of information about each model in the inference profile. See `models`.
        pub models: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::bedrock::GetInferenceProfileModel>,
        >,
        /// The status of the inference profile. `ACTIVE` means that the inference profile is available to use.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The type of the inference profile. `SYSTEM_DEFINED` means that the inference profile is defined by Amazon Bedrock. `APPLICATION` means that the inference profile is defined by the user.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The time at which the inference profile was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInferenceProfileArgs,
    ) -> GetInferenceProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let inference_profile_id_binding = args.inference_profile_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:bedrock/getInferenceProfile:getInferenceProfile".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inferenceProfileId".into(),
                    value: &inference_profile_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInferenceProfileResult {
            created_at: o.get_field("createdAt"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            inference_profile_arn: o.get_field("inferenceProfileArn"),
            inference_profile_id: o.get_field("inferenceProfileId"),
            inference_profile_name: o.get_field("inferenceProfileName"),
            models: o.get_field("models"),
            status: o.get_field("status"),
            type_: o.get_field("type"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
