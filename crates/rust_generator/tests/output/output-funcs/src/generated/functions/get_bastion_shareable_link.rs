#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_bastion_shareable_link {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBastionShareableLinkArgs {
        /// The name of the Bastion Host.
        #[builder(into)]
        pub bastion_host_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of VM references.
        #[builder(into, default)]
        pub vms: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::BastionShareableLink>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetBastionShareableLinkResult {
        /// The URL to get the next set of results.
        pub next_link: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBastionShareableLinkArgs,
    ) -> GetBastionShareableLinkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bastion_host_name_binding = args.bastion_host_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let vms_binding = args.vms.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "mypkg::getBastionShareableLink".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bastionHostName".into(),
                    value: bastion_host_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vms".into(),
                    value: vms_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBastionShareableLinkResult {
            next_link: o.get_field("nextLink"),
        }
    }
}
