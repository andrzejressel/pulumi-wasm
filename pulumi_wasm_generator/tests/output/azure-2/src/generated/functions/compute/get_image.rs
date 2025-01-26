pub mod get_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// The name of the Image.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Regex pattern of the image to match.
        #[builder(into, default)]
        pub name_regex: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Name of the Resource Group where this Image exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// By default when matching by regex, images are sorted by name in ascending order and the first match is chosen, to sort descending, set this flag.
        #[builder(into, default)]
        pub sort_descending: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        /// a collection of `data_disk` blocks as defined below.
        pub data_disks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetImageDataDisk>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// the Azure Location where this Image exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// the name of the Image.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub name_regex: pulumi_wasm_rust::Output<Option<String>>,
        /// a `os_disk` block as defined below.
        pub os_disks: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetImageOsDisk>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub sort_descending: pulumi_wasm_rust::Output<Option<bool>>,
        /// a mapping of tags to assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// is zone resiliency enabled?
        pub zone_resilient: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetImageArgs,
    ) -> GetImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let name_regex_binding = args.name_regex.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sort_descending_binding = args
            .sort_descending
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getImage:getImage".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nameRegex".into(),
                    value: &name_regex_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sortDescending".into(),
                    value: &sort_descending_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataDisks".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nameRegex".into(),
                },
                register_interface::ResultField {
                    name: "osDisks".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sortDescending".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "zoneResilient".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetImageResult {
            data_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataDisks").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_regex: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameRegex").unwrap(),
            ),
            os_disks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osDisks").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sort_descending: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sortDescending").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            zone_resilient: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneResilient").unwrap(),
            ),
        }
    }
}
