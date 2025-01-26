pub mod get_image_recipe {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageRecipeArgs {
        /// ARN of the image recipe.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the image recipe.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetImageRecipeResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Set of objects with block device mappings for the image recipe.
        pub block_device_mappings: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetImageRecipeBlockDeviceMapping,
            >,
        >,
        /// List of objects with components for the image recipe.
        pub components: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::imagebuilder::GetImageRecipeComponent>,
        >,
        /// Date the image recipe was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// Description of the image recipe.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the image recipe.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Owner of the image recipe.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// Base image of the image recipe.
        pub parent_image: pulumi_wasm_rust::Output<String>,
        /// Platform of the image recipe.
        pub platform: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the image recipe.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Base64 encoded contents of user data. Commands or a command script to run when build instance is launched.
        pub user_data_base64: pulumi_wasm_rust::Output<String>,
        /// Version of the image recipe.
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
        args: GetImageRecipeArgs,
    ) -> GetImageRecipeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getImageRecipe:getImageRecipe".into(),
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
                    name: "blockDeviceMappings".into(),
                },
                register_interface::ResultField {
                    name: "components".into(),
                },
                register_interface::ResultField {
                    name: "dateCreated".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
                    name: "userDataBase64".into(),
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
        GetImageRecipeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            block_device_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockDeviceMappings").unwrap(),
            ),
            components: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("components").unwrap(),
            ),
            date_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateCreated").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            user_data_base64: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userDataBase64").unwrap(),
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
