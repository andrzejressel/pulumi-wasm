pub mod get_repository {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryArgs {
        /// Name of the ECR Repository.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Registry ID where the repository was created.
        #[builder(into, default)]
        pub registry_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryResult {
        /// Full ARN of the repository.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Encryption configuration for the repository. See Encryption Configuration below.
        pub encryption_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ecr::GetRepositoryEncryptionConfiguration>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Configuration block that defines image scanning configuration for the repository. See Image Scanning Configuration below.
        pub image_scanning_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ecr::GetRepositoryImageScanningConfiguration>,
        >,
        /// The tag mutability setting for the repository.
        pub image_tag_mutability: pulumi_wasm_rust::Output<String>,
        /// List of image tags associated with the most recently pushed image in the repository.
        pub most_recent_image_tags: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// URL of the repository (in the form `aws_account_id.dkr.ecr.region.amazonaws.com/repositoryName`).
        pub repository_url: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRepositoryArgs,
    ) -> GetRepositoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let registry_id_binding = args.registry_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecr/getRepository:getRepository".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "registryId".into(),
                    value: &registry_id_binding,
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
                    name: "encryptionConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageScanningConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "imageTagMutability".into(),
                },
                register_interface::ResultField {
                    name: "mostRecentImageTags".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryUrl".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRepositoryResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            encryption_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfigurations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_scanning_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageScanningConfigurations").unwrap(),
            ),
            image_tag_mutability: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageTagMutability").unwrap(),
            ),
            most_recent_image_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecentImageTags").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            repository_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryUrl").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
