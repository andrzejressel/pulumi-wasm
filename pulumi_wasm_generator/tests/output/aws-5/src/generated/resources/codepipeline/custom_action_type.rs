/// Provides a CodeDeploy CustomActionType
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod custom_action_type {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomActionTypeArgs {
        /// The category of the custom action. Valid values: `Source`, `Build`, `Deploy`, `Test`, `Invoke`, `Approval`
        #[builder(into)]
        pub category: pulumi_wasm_rust::Output<String>,
        /// The configuration properties for the custom action. Max 10 items.
        #[builder(into, default)]
        pub configuration_properties: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::codepipeline::CustomActionTypeConfigurationProperty,
                >,
            >,
        >,
        #[builder(into)]
        pub input_artifact_details: pulumi_wasm_rust::Output<
            super::super::types::codepipeline::CustomActionTypeInputArtifactDetails,
        >,
        #[builder(into)]
        pub output_artifact_details: pulumi_wasm_rust::Output<
            super::super::types::codepipeline::CustomActionTypeOutputArtifactDetails,
        >,
        #[builder(into)]
        pub provider_name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::Output<
            Option<super::super::types::codepipeline::CustomActionTypeSettings>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CustomActionTypeResult {
        /// The action ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The category of the custom action. Valid values: `Source`, `Build`, `Deploy`, `Test`, `Invoke`, `Approval`
        pub category: pulumi_wasm_rust::Output<String>,
        /// The configuration properties for the custom action. Max 10 items.
        pub configuration_properties: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::codepipeline::CustomActionTypeConfigurationProperty,
                >,
            >,
        >,
        pub input_artifact_details: pulumi_wasm_rust::Output<
            super::super::types::codepipeline::CustomActionTypeInputArtifactDetails,
        >,
        pub output_artifact_details: pulumi_wasm_rust::Output<
            super::super::types::codepipeline::CustomActionTypeOutputArtifactDetails,
        >,
        /// The creator of the action being called.
        pub owner: pulumi_wasm_rust::Output<String>,
        pub provider_name: pulumi_wasm_rust::Output<String>,
        pub settings: pulumi_wasm_rust::Output<
            Option<super::super::types::codepipeline::CustomActionTypeSettings>,
        >,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CustomActionTypeArgs) -> CustomActionTypeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let category_binding = args.category.get_inner();
        let configuration_properties_binding = args.configuration_properties.get_inner();
        let input_artifact_details_binding = args.input_artifact_details.get_inner();
        let output_artifact_details_binding = args.output_artifact_details.get_inner();
        let provider_name_binding = args.provider_name.get_inner();
        let settings_binding = args.settings.get_inner();
        let tags_binding = args.tags.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codepipeline/customActionType:CustomActionType".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "category".into(),
                    value: &category_binding,
                },
                register_interface::ObjectField {
                    name: "configurationProperties".into(),
                    value: &configuration_properties_binding,
                },
                register_interface::ObjectField {
                    name: "inputArtifactDetails".into(),
                    value: &input_artifact_details_binding,
                },
                register_interface::ObjectField {
                    name: "outputArtifactDetails".into(),
                    value: &output_artifact_details_binding,
                },
                register_interface::ObjectField {
                    name: "providerName".into(),
                    value: &provider_name_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "category".into(),
                },
                register_interface::ResultField {
                    name: "configurationProperties".into(),
                },
                register_interface::ResultField {
                    name: "inputArtifactDetails".into(),
                },
                register_interface::ResultField {
                    name: "outputArtifactDetails".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "providerName".into(),
                },
                register_interface::ResultField {
                    name: "settings".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomActionTypeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            category: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("category").unwrap(),
            ),
            configuration_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationProperties").unwrap(),
            ),
            input_artifact_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputArtifactDetails").unwrap(),
            ),
            output_artifact_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputArtifactDetails").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            provider_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerName").unwrap(),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settings").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
