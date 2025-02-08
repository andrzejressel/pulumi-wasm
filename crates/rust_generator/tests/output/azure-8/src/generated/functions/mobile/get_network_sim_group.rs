#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_sim_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkSimGroupArgs {
        /// The ID of Mobile Network which the Mobile Network Sim Group belongs to.
        #[builder(into)]
        pub mobile_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Mobile Network Sim Groups.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkSimGroupResult {
        /// A key to encrypt the SIM data that belongs to this SIM group.
        pub encryption_key_url: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mobile::GetNetworkSimGroupIdentity>,
        >,
        /// The Azure Region where the Mobile Network Sim Groups should exist.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub mobile_network_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Groups.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetNetworkSimGroupArgs,
    ) -> GetNetworkSimGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let mobile_network_id_binding = args
            .mobile_network_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mobile/getNetworkSimGroup:getNetworkSimGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mobileNetworkId".into(),
                    value: &mobile_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNetworkSimGroupResult {
            encryption_key_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionKeyUrl"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mobile_network_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mobileNetworkId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
