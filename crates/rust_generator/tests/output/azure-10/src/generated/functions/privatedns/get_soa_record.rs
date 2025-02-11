#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_soa_record {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSoaRecordArgs {
        /// The name of the Private DNS SOA Record.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the resource group where the Private DNS Zone (parent resource) exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Private DNS Zone where the resource exists.
        #[builder(into)]
        pub zone_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSoaRecordResult {
        /// The email contact for the SOA record.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The expire time for the SOA record.
        pub expire_time: pulumi_gestalt_rust::Output<i32>,
        /// The FQDN of the Private DNS SOA Record.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// The domain name of the authoritative name server for the SOA record.
        pub host_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The minimum Time To Live for the SOA record. By convention, it is used to determine the negative caching duration.
        pub minimum_ttl: pulumi_gestalt_rust::Output<i32>,
        /// The name of the Private DNS SOA Record.
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
        /// The Time To Live (TTL) of the Private DNS record in seconds.
        pub ttl: pulumi_gestalt_rust::Output<i32>,
        pub zone_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSoaRecordArgs,
    ) -> GetSoaRecordResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let zone_name_binding = args.zone_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:privatedns/getSoaRecord:getSoaRecord".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneName".into(),
                    value: &zone_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSoaRecordResult {
            email: o.get_field("email"),
            expire_time: o.get_field("expireTime"),
            fqdn: o.get_field("fqdn"),
            host_name: o.get_field("hostName"),
            id: o.get_field("id"),
            minimum_ttl: o.get_field("minimumTtl"),
            name: o.get_field("name"),
            refresh_time: o.get_field("refreshTime"),
            resource_group_name: o.get_field("resourceGroupName"),
            retry_time: o.get_field("retryTime"),
            serial_number: o.get_field("serialNumber"),
            tags: o.get_field("tags"),
            ttl: o.get_field("ttl"),
            zone_name: o.get_field("zoneName"),
        }
    }
}
