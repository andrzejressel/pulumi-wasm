pub mod get_container_recipe {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContainerRecipeArgs {
        /// ARN of the container recipe.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the container recipe.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetContainerRecipeResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// List of objects with components for the container recipe.
        pub components: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::imagebuilder::GetContainerRecipeComponent>,
        >,
        /// Type of the container.
        pub container_type: pulumi_wasm_rust::Output<String>,
        /// Date the container recipe was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// Description of the container recipe.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Dockerfile template used to build the image.
        pub dockerfile_template_data: pulumi_wasm_rust::Output<String>,
        /// Whether to encrypt the volume. Defaults to unset, which is the value inherited from the parent image.
        pub encrypted: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of objects with instance configurations for building and testing container images.
        pub instance_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetContainerRecipeInstanceConfiguration,
            >,
        >,
        /// KMS key used to encrypt the container image.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Name of the container recipe.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Owner of the container recipe.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// Base image for the container recipe.
        pub parent_image: pulumi_wasm_rust::Output<String>,
        /// Platform of the container recipe.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the container recipe.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Destination repository for the container image.
        pub target_repositories: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetContainerRecipeTargetRepository,
            >,
        >,
        /// Version of the container recipe.
        pub version: pulumi_wasm_rust::Output<String>,
        /// Working directory used during build and test workflows.
        pub working_directory: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetContainerRecipeArgs,
    ) -> GetContainerRecipeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getContainerRecipe:getContainerRecipe".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
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
                    name: "components".into(),
                },
                register_interface::ResultField {
                    name: "containerType".into(),
                },
                register_interface::ResultField {
                    name: "dateCreated".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dockerfileTemplateData".into(),
                },
                register_interface::ResultField {
                    name: "encrypted".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "parentImage".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "targetRepositories".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "workingDirectory".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetContainerRecipeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            components: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("components").unwrap(),
            ),
            container_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerType").unwrap(),
            ),
            date_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateCreated").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dockerfile_template_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dockerfileTemplateData").unwrap(),
            ),
            encrypted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encrypted").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceConfigurations").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            parent_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentImage").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            target_repositories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetRepositories").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            working_directory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workingDirectory").unwrap(),
            ),
        }
    }
}
