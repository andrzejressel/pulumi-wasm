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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetZoneArgs,
    ) -> GetZoneResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let private_zone_binding = args.private_zone.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getZone:getZone".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateZone".into(),
                    value: &private_zone_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetZoneResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            caller_reference: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("callerReference"),
            ),
            comment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            linked_service_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkedServiceDescription"),
            ),
            linked_service_principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("linkedServicePrincipal"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nameServers"),
            ),
            primary_name_server: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryNameServer"),
            ),
            private_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateZone"),
            ),
            resource_record_set_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceRecordSetCount"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcId"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
