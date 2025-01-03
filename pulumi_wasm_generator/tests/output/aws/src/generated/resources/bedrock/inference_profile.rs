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
pub mod inference_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InferenceProfileArgs {
        /// The description of the inference profile.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The source of the model this inference profile will track metrics and cost for. See `model_source`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub model_source: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::InferenceProfileModelSource>,
        >,
        /// The name of the inference profile.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags for the inference profile.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::InferenceProfileTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct InferenceProfileResult {
        /// The Amazon Resource Name (ARN) of the inference profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The time at which the inference profile was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The description of the inference profile.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The source of the model this inference profile will track metrics and cost for. See `model_source`.
        ///
        /// The following arguments are optional:
        pub model_source: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::InferenceProfileModelSource>,
        >,
        /// A list of information about each model in the inference profile. See `models`.
        pub models: pulumi_wasm_rust::Output<
            Vec<super::super::types::bedrock::InferenceProfileModel>,
        >,
        /// The name of the inference profile.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The status of the inference profile. `ACTIVE` means that the inference profile is available to use.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags for the inference profile.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::bedrock::InferenceProfileTimeouts>,
        >,
        /// The type of the inference profile. `SYSTEM_DEFINED` means that the inference profile is defined by Amazon Bedrock. `APPLICATION` means that the inference profile is defined by the user.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The time at which the inference profile was last updated.
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InferenceProfileArgs) -> InferenceProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let model_source_binding = args.model_source.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/inferenceProfile:InferenceProfile".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "modelSource".into(),
                    value: &model_source_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "modelSource".into(),
                },
                register_interface::ResultField {
                    name: "models".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InferenceProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            model_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelSource").unwrap(),
            ),
            models: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("models").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}
