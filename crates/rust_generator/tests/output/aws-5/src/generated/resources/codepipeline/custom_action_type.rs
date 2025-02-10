/// Provides a CodeDeploy CustomActionType
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = custom_action_type::create(
///         "example",
///         CustomActionTypeArgs::builder()
///             .category("Build")
///             .input_artifact_details(
///                 CustomActionTypeInputArtifactDetails::builder()
///                     .maximumCount(1)
///                     .minimumCount(0)
///                     .build_struct(),
///             )
///             .output_artifact_details(
///                 CustomActionTypeOutputArtifactDetails::builder()
///                     .maximumCount(1)
///                     .minimumCount(0)
///                     .build_struct(),
///             )
///             .provider_name("example")
///             .version("1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeDeploy CustomActionType using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:codepipeline/customActionType:CustomActionType example Build:pulumi:1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_action_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomActionTypeArgs {
        /// The category of the custom action. Valid values: `Source`, `Build`, `Deploy`, `Test`, `Invoke`, `Approval`
        #[builder(into)]
        pub category: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The configuration properties for the custom action. Max 10 items.
        #[builder(into, default)]
        pub configuration_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::codepipeline::CustomActionTypeConfigurationProperty,
                >,
            >,
        >,
        #[builder(into)]
        pub input_artifact_details: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::codepipeline::CustomActionTypeInputArtifactDetails,
        >,
        #[builder(into)]
        pub output_artifact_details: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::codepipeline::CustomActionTypeOutputArtifactDetails,
        >,
        #[builder(into)]
        pub provider_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codepipeline::CustomActionTypeSettings>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomActionTypeResult {
        /// The action ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The category of the custom action. Valid values: `Source`, `Build`, `Deploy`, `Test`, `Invoke`, `Approval`
        pub category: pulumi_gestalt_rust::Output<String>,
        /// The configuration properties for the custom action. Max 10 items.
        pub configuration_properties: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::codepipeline::CustomActionTypeConfigurationProperty,
                >,
            >,
        >,
        pub input_artifact_details: pulumi_gestalt_rust::Output<
            super::super::types::codepipeline::CustomActionTypeInputArtifactDetails,
        >,
        pub output_artifact_details: pulumi_gestalt_rust::Output<
            super::super::types::codepipeline::CustomActionTypeOutputArtifactDetails,
        >,
        /// The creator of the action being called.
        pub owner: pulumi_gestalt_rust::Output<String>,
        pub provider_name: pulumi_gestalt_rust::Output<String>,
        pub settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::codepipeline::CustomActionTypeSettings>,
        >,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomActionTypeArgs,
    ) -> CustomActionTypeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let category_binding = args.category.get_output(context);
        let configuration_properties_binding = args
            .configuration_properties
            .get_output(context);
        let input_artifact_details_binding = args
            .input_artifact_details
            .get_output(context);
        let output_artifact_details_binding = args
            .output_artifact_details
            .get_output(context);
        let provider_name_binding = args.provider_name.get_output(context);
        let settings_binding = args.settings.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codepipeline/customActionType:CustomActionType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "category".into(),
                    value: category_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationProperties".into(),
                    value: configuration_properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputArtifactDetails".into(),
                    value: input_artifact_details_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outputArtifactDetails".into(),
                    value: output_artifact_details_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerName".into(),
                    value: provider_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomActionTypeResult {
            arn: o.get_field("arn"),
            category: o.get_field("category"),
            configuration_properties: o.get_field("configurationProperties"),
            input_artifact_details: o.get_field("inputArtifactDetails"),
            output_artifact_details: o.get_field("outputArtifactDetails"),
            owner: o.get_field("owner"),
            provider_name: o.get_field("providerName"),
            settings: o.get_field("settings"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
        }
    }
}
