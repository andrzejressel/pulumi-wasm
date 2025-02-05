/// Manages a replicated Region and directory for Multi-Region replication.
/// Multi-Region replication is only supported for the Enterprise Edition of AWS Managed Microsoft AD.
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import Replicated Regions using directory ID,Region name. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/serviceRegion:ServiceRegion example d-9267651497,us-east-2
/// ```
pub mod service_region {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceRegionArgs {
        /// The number of domain controllers desired in the replicated directory. Minimum value of `2`.
        #[builder(into, default)]
        pub desired_number_of_domain_controllers: pulumi_wasm_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The identifier of the directory to which you want to add Region replication.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Region where you want to add domain controllers for replication.
        #[builder(into)]
        pub region_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// VPC information in the replicated Region. Detailed below.
        #[builder(into)]
        pub vpc_settings: pulumi_wasm_rust::InputOrOutput<
            super::super::types::directoryservice::ServiceRegionVpcSettings,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceRegionResult {
        /// The number of domain controllers desired in the replicated directory. Minimum value of `2`.
        pub desired_number_of_domain_controllers: pulumi_wasm_rust::Output<i32>,
        /// The identifier of the directory to which you want to add Region replication.
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Region where you want to add domain controllers for replication.
        pub region_name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// VPC information in the replicated Region. Detailed below.
        pub vpc_settings: pulumi_wasm_rust::Output<
            super::super::types::directoryservice::ServiceRegionVpcSettings,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServiceRegionArgs,
    ) -> ServiceRegionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let desired_number_of_domain_controllers_binding = args
            .desired_number_of_domain_controllers
            .get_output(context)
            .get_inner();
        let directory_id_binding = args.directory_id.get_output(context).get_inner();
        let region_name_binding = args.region_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_settings_binding = args.vpc_settings.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directoryservice/serviceRegion:ServiceRegion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "desiredNumberOfDomainControllers".into(),
                    value: &desired_number_of_domain_controllers_binding,
                },
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "regionName".into(),
                    value: &region_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSettings".into(),
                    value: &vpc_settings_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceRegionResult {
            desired_number_of_domain_controllers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("desiredNumberOfDomainControllers"),
            ),
            directory_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("directoryId"),
            ),
            region_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("regionName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcSettings"),
            ),
        }
    }
}
