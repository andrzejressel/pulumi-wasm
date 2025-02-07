pub mod get_soa_record {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSoaRecordArgs {
        /// The name of the DNS SOA Record.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the resource group where the DNS Zone (parent resource) exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the DNS Zone where the resource exists.
        #[builder(into)]
        pub zone_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSoaRecordResult {
        /// The email contact for the SOA record.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The expire time for the SOA record.
        pub expire_time: pulumi_gestalt_rust::Output<i32>,
        /// The FQDN of the DNS SOA Record.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// The domain name of the authoritative name server for the SOA record.
        pub host_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The minimum Time To Live for the SOA record. By convention, it is used to determine the negative caching duration.
        pub minimum_ttl: pulumi_gestalt_rust::Output<i32>,
        /// The name of the DNS SOA Record.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The refresh time for the SOA record.
        pub refresh_time: pulumi_gestalt_rust::Output<i32>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The retry time for the SOA record.
        pub retry_time: pulumi_gestalt_rust::Output<i32>,
        /// The serial number for the SOA record.
        pub serial_number: pulumi_gestalt_rust::Output<i32>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Time To Live (TTL) of the DNS record in seconds.
        pub ttl: pulumi_gestalt_rust::Output<i32>,
        pub zone_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSoaRecordArgs,
    ) -> GetSoaRecordResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let zone_name_binding = args.zone_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:dns/getSoaRecord:getSoaRecord".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "zoneName".into(),
                    value: &zone_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSoaRecordResult {
            email: pulumi_gestalt_rust::__private::into_domain(o.extract_field("email")),
            expire_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expireTime"),
            ),
            fqdn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("fqdn")),
            host_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            minimum_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumTtl"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            refresh_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("refreshTime"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            retry_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("retryTime"),
            ),
            serial_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serialNumber"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            ttl: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ttl")),
            zone_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneName"),
            ),
        }
    }
}
