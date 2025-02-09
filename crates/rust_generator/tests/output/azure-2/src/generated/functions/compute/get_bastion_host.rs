#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_bastion_host {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBastionHostArgs {
        /// The name of the Bastion Host.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Bastion Host exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBastionHostResult {
        /// Is Copy/Paste feature enabled for the Bastion Host.
        pub copy_paste_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The FQDN for the Bastion Host.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Is File Copy feature enabled for the Bastion Host.
        pub file_copy_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `ip_configuration` block as defined below.
        pub ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBastionHostIpConfiguration>,
        >,
        /// Is IP Connect feature enabled for the Bastion Host.
        pub ip_connect_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Azure Region where the Bastion Host exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the IP configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The number of scale units provisioned for the Bastion Host.
        pub scale_units: pulumi_gestalt_rust::Output<i32>,
        /// Is Session Recording feature enabled for the Bastion Host.
        pub session_recording_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Is Shareable Link feature enabled for the Bastion Host.
        pub shareable_link_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The SKU of the Bastion Host.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Bastion Host.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Is Tunneling feature enabled for the Bastion Host.
        pub tunneling_enabled: pulumi_gestalt_rust::Output<bool>,
        /// A list of Availability Zones in which this Bastion Host is located.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBastionHostArgs,
    ) -> GetBastionHostResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:compute/getBastionHost:getBastionHost".into(),
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
        GetBastionHostResult {
            copy_paste_enabled: o.get_field("copyPasteEnabled"),
            dns_name: o.get_field("dnsName"),
            file_copy_enabled: o.get_field("fileCopyEnabled"),
            id: o.get_field("id"),
            ip_configurations: o.get_field("ipConfigurations"),
            ip_connect_enabled: o.get_field("ipConnectEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            scale_units: o.get_field("scaleUnits"),
            session_recording_enabled: o.get_field("sessionRecordingEnabled"),
            shareable_link_enabled: o.get_field("shareableLinkEnabled"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            tunneling_enabled: o.get_field("tunnelingEnabled"),
            zones: o.get_field("zones"),
        }
    }
}
