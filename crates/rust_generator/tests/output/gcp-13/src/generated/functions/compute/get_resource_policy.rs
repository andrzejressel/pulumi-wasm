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
        context: &pulumi_gestalt_rust::Context,
        args: GetResourcePolicyArgs,
    ) -> GetResourcePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getResourcePolicy:getResourcePolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResourcePolicyResult {
            description: o.get_field("description"),
            disk_consistency_group_policies: o.get_field("diskConsistencyGroupPolicies"),
            group_placement_policies: o.get_field("groupPlacementPolicies"),
            id: o.get_field("id"),
            instance_schedule_policies: o.get_field("instanceSchedulePolicies"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            snapshot_schedule_policies: o.get_field("snapshotSchedulePolicies"),
        }
    }
}
