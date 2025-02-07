pub mod get_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// The name of the Image.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Regex pattern of the image to match.
        #[builder(into, default)]
        pub name_regex: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name of the Resource Group where this Image exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// By default when matching by regex, images are sorted by name in ascending order and the first match is chosen, to sort descending, set this flag.
        #[builder(into, default)]
        pub sort_descending: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        /// a collection of `data_disk` blocks as defined below.
        pub data_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetImageDataDisk>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// the Azure Location where this Image exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// the name of the Image.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        pub name_regex: pulumi_gestalt_rust::Output<Option<String>>,
        /// a `os_disk` block as defined below.
        pub os_disks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetImageOsDisk>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub sort_descending: pulumi_gestalt_rust::Output<Option<bool>>,
        /// a mapping of tags to assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// is zone resiliency enabled?
        pub zone_resilient: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetImageArgs,
    ) -> GetImageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetImageResult {
            data_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataDisks"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_regex: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nameRegex"),
            ),
            os_disks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("osDisks"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sort_descending: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sortDescending"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            zone_resilient: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneResilient"),
            ),
        }
    }
}
