/// Resource for managing an AWS Bedrock Inference Profile.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:bedrock:InferenceProfile
///     properties:
///       name: Claude Sonnet for Project 123
///       description: Profile with tag for cost allocation tracking
///       modelSource:
///         copyFrom: arn:aws:bedrock:us-west-2::foundation-model/anthropic.claude-3-5-sonnet-20241022-v2:0
///       tags:
///         ProjectID: '123'
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Bedrock Inference Profile using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/inferenceProfile:InferenceProfile example inference_profile-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod inference_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InferenceProfileArgs {
        /// The description of the inference profile.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The source of the model this inference profile will track metrics and cost for. See `model_source`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub model_source: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::InferenceProfileModelSource>,
        >,
        /// The name of the inference profile.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags for the inference profile.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::InferenceProfileTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct InferenceProfileResult {
        /// The Amazon Resource Name (ARN) of the inference profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The time at which the inference profile was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The description of the inference profile.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The source of the model this inference profile will track metrics and cost for. See `model_source`.
        ///
        /// The following arguments are optional:
        pub model_source: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::InferenceProfileModelSource>,
        >,
        /// A list of information about each model in the inference profile. See `models`.
        pub models: pulumi_gestalt_rust::Output<
            Vec<super::super::types::bedrock::InferenceProfileModel>,
        >,
        /// The name of the inference profile.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The status of the inference profile. `ACTIVE` means that the inference profile is available to use.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags for the inference profile.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::InferenceProfileTimeouts>,
        >,
        /// The type of the inference profile. `SYSTEM_DEFINED` means that the inference profile is defined by Amazon Bedrock. `APPLICATION` means that the inference profile is defined by the user.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The time at which the inference profile was last updated.
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InferenceProfileArgs,
    ) -> InferenceProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let model_source_binding = args.model_source.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:bedrock/inferenceProfile:InferenceProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "modelSource".into(),
                    value: &model_source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InferenceProfileResult {
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            description: o.get_field("description"),
            model_source: o.get_field("modelSource"),
            models: o.get_field("models"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            type_: o.get_field("type"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
