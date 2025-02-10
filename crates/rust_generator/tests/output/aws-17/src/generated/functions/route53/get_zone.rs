#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZoneArgs {
        /// Hosted Zone name of the desired Hosted Zone.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used with `name` field to get a private Hosted Zone.
        #[builder(into, default)]
        pub private_zone: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Used with `name` field. A map of tags, each pair of which must exactly match a pair on the desired Hosted Zone.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Used with `name` field to get a private Hosted Zone associated with the vpc_id (in this case, private_zone is not mandatory).
        #[builder(into, default)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Hosted Zone id of the desired Hosted Zone.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetZoneResult {
        /// ARN of the Hosted Zone.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Caller Reference of the Hosted Zone.
        pub caller_reference: pulumi_gestalt_rust::Output<String>,
        /// Comment field of the Hosted Zone.
        pub comment: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The description provided by the service that created the Hosted Zone (e.g., `arn:aws:servicediscovery:us-east-1:1234567890:namespace/ns-xxxxxxxxxxxxxxxx`).
        pub linked_service_description: pulumi_gestalt_rust::Output<String>,
        /// The service that created the Hosted Zone (e.g., `servicediscovery.amazonaws.com`).
        pub linked_service_principal: pulumi_gestalt_rust::Output<String>,
        /// The Hosted Zone name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of DNS name servers for the Hosted Zone.
        pub name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Route 53 name server that created the SOA record.
        pub primary_name_server: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether this is a private hosted zone.
        pub private_zone: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The number of Record Set in the Hosted Zone.
        pub resource_record_set_count: pulumi_gestalt_rust::Output<i32>,
        /// A map of tags assigned to the Hosted Zone.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The Hosted Zone identifier.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetZoneArgs,
    ) -> GetZoneResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let private_zone_binding = args.private_zone.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:route53/getZone:getZone".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateZone".into(),
                    value: private_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetZoneResult {
            arn: o.get_field("arn"),
            caller_reference: o.get_field("callerReference"),
            comment: o.get_field("comment"),
            id: o.get_field("id"),
            linked_service_description: o.get_field("linkedServiceDescription"),
            linked_service_principal: o.get_field("linkedServicePrincipal"),
            name: o.get_field("name"),
            name_servers: o.get_field("nameServers"),
            primary_name_server: o.get_field("primaryNameServer"),
            private_zone: o.get_field("privateZone"),
            resource_record_set_count: o.get_field("resourceRecordSetCount"),
            tags: o.get_field("tags"),
            vpc_id: o.get_field("vpcId"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
