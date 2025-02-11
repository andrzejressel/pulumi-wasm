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
        context: &pulumi_gestalt_rust::Context,
        args: GetVolumeGroupArgs,
    ) -> GetVolumeGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let elastic_san_id_binding = args.elastic_san_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:elasticsan/getVolumeGroup:getVolumeGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "elasticSanId".into(),
                    value: &elastic_san_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVolumeGroupResult {
            elastic_san_id: o.get_field("elasticSanId"),
            encryption_type: o.get_field("encryptionType"),
            encryptions: o.get_field("encryptions"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            name: o.get_field("name"),
            network_rules: o.get_field("networkRules"),
            protocol_type: o.get_field("protocolType"),
        }
    }
}
