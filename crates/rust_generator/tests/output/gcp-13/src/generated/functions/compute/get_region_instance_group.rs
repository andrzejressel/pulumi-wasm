#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_region_instance_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionInstanceGroupArgs {
        /// The name of the instance group.  One of `name` or `self_link` must be provided.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If `self_link` is provided, this value is ignored.  If neither `self_link`
        /// nor `project` are provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region in which the resource belongs.  If `self_link`
        /// is provided, this value is ignored.  If neither `self_link` nor `region` are
        /// provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The link to the instance group.  One of `name` or `self_link` must be provided.
        ///
        /// - - -
        #[builder(into, default)]
        pub self_link: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionInstanceGroupResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of instances in the group, as a list of resources, each containing:
        pub instances: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetRegionInstanceGroupInstance>,
        >,
        /// String port name
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The number of instances in the group.
        pub size: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRegionInstanceGroupArgs,
    ) -> GetRegionInstanceGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let self_link_binding_1 = args.self_link.get_output(context);
        let self_link_binding = self_link_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getRegionInstanceGroup:getRegionInstanceGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "selfLink".into(),
                    value: &self_link_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRegionInstanceGroupResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instances"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            size: pulumi_gestalt_rust::__private::into_domain(o.extract_field("size")),
        }
    }
}
