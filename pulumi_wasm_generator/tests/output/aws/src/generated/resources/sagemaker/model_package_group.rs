/// Provides a SageMaker Model Package Group resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = model_package_group::create(
///         "example",
///         ModelPackageGroupArgs::builder()
///             .model_package_group_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Model Package Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/modelPackageGroup:ModelPackageGroup test_model_package_group my-code-repo
/// ```
pub mod model_package_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelPackageGroupArgs {
        /// A description for the model group.
        #[builder(into, default)]
        pub model_package_group_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the model group.
        #[builder(into)]
        pub model_package_group_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ModelPackageGroupResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Model Package Group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A description for the model group.
        pub model_package_group_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the model group.
        pub model_package_group_name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ModelPackageGroupArgs) -> ModelPackageGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let model_package_group_description_binding = args
            .model_package_group_description
            .get_inner();
        let model_package_group_name_binding = args.model_package_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/modelPackageGroup:ModelPackageGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "modelPackageGroupDescription".into(),
                    value: &model_package_group_description_binding,
                },
                register_interface::ObjectField {
                    name: "modelPackageGroupName".into(),
                    value: &model_package_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "modelPackageGroupDescription".into(),
                },
                register_interface::ResultField {
                    name: "modelPackageGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ModelPackageGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            model_package_group_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelPackageGroupDescription").unwrap(),
            ),
            model_package_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelPackageGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}