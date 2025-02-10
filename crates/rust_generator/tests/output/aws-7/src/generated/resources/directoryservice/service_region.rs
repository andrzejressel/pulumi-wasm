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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_region {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceRegionArgs {
        /// The number of domain controllers desired in the replicated directory. Minimum value of `2`.
        #[builder(into, default)]
        pub desired_number_of_domain_controllers: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The identifier of the directory to which you want to add Region replication.
        #[builder(into)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Region where you want to add domain controllers for replication.
        #[builder(into)]
        pub region_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// VPC information in the replicated Region. Detailed below.
        #[builder(into)]
        pub vpc_settings: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::directoryservice::ServiceRegionVpcSettings,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceRegionResult {
        /// The number of domain controllers desired in the replicated directory. Minimum value of `2`.
        pub desired_number_of_domain_controllers: pulumi_gestalt_rust::Output<i32>,
        /// The identifier of the directory to which you want to add Region replication.
        pub directory_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Region where you want to add domain controllers for replication.
        pub region_name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// VPC information in the replicated Region. Detailed below.
        pub vpc_settings: pulumi_gestalt_rust::Output<
            super::super::types::directoryservice::ServiceRegionVpcSettings,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceRegionArgs,
    ) -> ServiceRegionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let desired_number_of_domain_controllers_binding = args
            .desired_number_of_domain_controllers
            .get_output(context);
        let directory_id_binding = args.directory_id.get_output(context);
        let region_name_binding = args.region_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_settings_binding = args.vpc_settings.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directoryservice/serviceRegion:ServiceRegion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "desiredNumberOfDomainControllers".into(),
                    value: desired_number_of_domain_controllers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: directory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regionName".into(),
                    value: region_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcSettings".into(),
                    value: vpc_settings_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceRegionResult {
            desired_number_of_domain_controllers: o
                .get_field("desiredNumberOfDomainControllers"),
            directory_id: o.get_field("directoryId"),
            region_name: o.get_field("regionName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_settings: o.get_field("vpcSettings"),
        }
    }
}
