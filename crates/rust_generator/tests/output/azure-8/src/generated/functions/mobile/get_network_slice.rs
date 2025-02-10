#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_slice {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkSliceArgs {
        /// The ID of Mobile Network which the Mobile Network Slice belongs to.
        #[builder(into)]
        pub mobile_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Mobile Network Slice.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkSliceResult {
        /// A description of this Mobile Network Slice.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Mobile Network Slice exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub mobile_network_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `single_network_slice_selection_assistance_information` block as defined below. Single-network slice selection assistance information (S-NSSAI).
        pub single_network_slice_selection_assistance_informations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkSliceSingleNetworkSliceSelectionAssistanceInformation,
            >,
        >,
        /// A mapping of tags which are assigned to the Mobile Network Slice.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkSliceArgs,
    ) -> GetNetworkSliceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mobile_network_id_binding = args.mobile_network_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mobile/getNetworkSlice:getNetworkSlice".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkId".into(),
                    value: mobile_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkSliceResult {
            description: o.get_field("description"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            mobile_network_id: o.get_field("mobileNetworkId"),
            name: o.get_field("name"),
            single_network_slice_selection_assistance_informations: o
                .get_field("singleNetworkSliceSelectionAssistanceInformations"),
            tags: o.get_field("tags"),
        }
    }
}
