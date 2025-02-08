#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resource_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourcePolicyArgs {
        /// The name of the Resource Policy.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Project from which to list the Resource Policy. Defaults to project declared in the provider.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region where the Resource Policy resides.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetResourcePolicyResult {
        /// Description of this Resource Policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub disk_consistency_group_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetResourcePolicyDiskConsistencyGroupPolicy,
            >,
        >,
        pub group_placement_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetResourcePolicyGroupPlacementPolicy,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_schedule_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetResourcePolicyInstanceSchedulePolicy,
            >,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub snapshot_schedule_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetResourcePolicySnapshotSchedulePolicy,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResourcePolicyArgs,
    ) -> GetResourcePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getResourcePolicy:getResourcePolicy".into(),
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
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResourcePolicyResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disk_consistency_group_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskConsistencyGroupPolicies"),
            ),
            group_placement_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupPlacementPolicies"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_schedule_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceSchedulePolicies"),
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
            snapshot_schedule_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotSchedulePolicies"),
            ),
        }
    }
}
