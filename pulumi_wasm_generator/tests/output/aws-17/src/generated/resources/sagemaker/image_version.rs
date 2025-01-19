/// Provides a SageMaker Image Version resource.
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
///     let test = image_version::create(
///         "test",
///         ImageVersionArgs::builder()
///             .base_image("012345678912.dkr.ecr.us-west-2.amazonaws.com/image:latest")
///             .image_name("${testAwsSagemakerImage.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Image Versions using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/imageVersion:ImageVersion test_image my-code-repo
/// ```
pub mod image_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageVersionArgs {
        /// The registry path of the container image on which this image version is based.
        #[builder(into)]
        pub base_image: pulumi_wasm_rust::Output<String>,
        /// The name of the image. Must be unique to your account.
        #[builder(into)]
        pub image_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ImageVersionResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Image Version.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The registry path of the container image on which this image version is based.
        pub base_image: pulumi_wasm_rust::Output<String>,
        /// The registry path of the container image that contains this image version.
        pub container_image: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the image the version is based on.
        pub image_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the image. Must be unique to your account.
        pub image_name: pulumi_wasm_rust::Output<String>,
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ImageVersionArgs) -> ImageVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let base_image_binding = args.base_image.get_inner();
        let image_name_binding = args.image_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/imageVersion:ImageVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "baseImage".into(),
                    value: &base_image_binding,
                },
                register_interface::ObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "baseImage".into(),
                },
                register_interface::ResultField {
                    name: "containerImage".into(),
                },
                register_interface::ResultField {
                    name: "imageArn".into(),
                },
                register_interface::ResultField {
                    name: "imageName".into(),
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
        ImageVersionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            base_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseImage").unwrap(),
            ),
            container_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerImage").unwrap(),
            ),
            image_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageArn").unwrap(),
            ),
            image_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageName").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
