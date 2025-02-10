#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dedicated_host_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDedicatedHostGroupArgs {
        /// Specifies the name of the Dedicated Host Group.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Dedicated Host Group is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDedicatedHostGroupResult {
        /// Whether virtual machines or virtual machine scale sets be placed automatically on this Dedicated Host Group.
        pub automatic_placement_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where the Dedicated Host Group exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of fault domains that the Dedicated Host Group spans.
        pub platform_fault_domain_count: pulumi_gestalt_rust::Output<i32>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Availability Zones in which this Dedicated Host Group is located.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDedicatedHostGroupArgs,
    ) -> GetDedicatedHostGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:compute/getDedicatedHostGroup:getDedicatedHostGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDedicatedHostGroupResult {
            automatic_placement_enabled: o.get_field("automaticPlacementEnabled"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            platform_fault_domain_count: o.get_field("platformFaultDomainCount"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
