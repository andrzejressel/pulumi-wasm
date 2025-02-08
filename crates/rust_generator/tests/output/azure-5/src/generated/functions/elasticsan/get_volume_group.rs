#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_volume_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVolumeGroupArgs {
        /// The Elastic SAN ID within which the Elastic SAN Volume Group exists.
        #[builder(into)]
        pub elastic_san_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Elastic SAN Volume Group.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVolumeGroupResult {
        pub elastic_san_id: pulumi_gestalt_rust::Output<String>,
        /// The type of the key used to encrypt the data of the disk.
        pub encryption_type: pulumi_gestalt_rust::Output<String>,
        /// An `encryption` block as defined below.
        pub encryptions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsan::GetVolumeGroupEncryption>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsan::GetVolumeGroupIdentity>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `network_rule` blocks as defined below.
        pub network_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsan::GetVolumeGroupNetworkRule>,
        >,
        /// The type of the storage target.
        pub protocol_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVolumeGroupArgs,
    ) -> GetVolumeGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let elastic_san_id_binding = args.elastic_san_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:elasticsan/getVolumeGroup:getVolumeGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "elasticSanId".into(),
                    value: &elastic_san_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVolumeGroupResult {
            elastic_san_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("elasticSanId"),
            ),
            encryption_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionType"),
            ),
            encryptions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptions"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkRules"),
            ),
            protocol_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocolType"),
            ),
        }
    }
}
