/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let examplePtrRecord = ptr_record::create(
///         "examplePtrRecord",
///         PtrRecordArgs::builder()
///             .name("test")
///             .records(vec!["yourdomain.com",])
///             .resource_group_name("${example.name}")
///             .ttl(300)
///             .zone_name("${exampleZone.name}")
///             .build_struct(),
///     );
///     let exampleZone = zone::create(
///         "exampleZone",
///         ZoneArgs::builder()
///             .name("mydomain.com")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PTR records can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dns/ptrRecord:PtrRecord example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/dnsZones/zone1/PTR/myrecord1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ptr_record {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PtrRecordArgs {
        /// The name of the DNS PTR Record. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of Fully Qualified Domain Names.
        #[builder(into)]
        pub records: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Specifies the resource group where the DNS Zone (parent resource) exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Time To Live (TTL) of the DNS record in seconds.
        #[builder(into)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specifies the DNS Zone where the resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub zone_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PtrRecordResult {
        /// The FQDN of the DNS PTR Record.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// The name of the DNS PTR Record. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// List of Fully Qualified Domain Names.
        pub records: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the resource group where the DNS Zone (parent resource) exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Time To Live (TTL) of the DNS record in seconds.
        pub ttl: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the DNS Zone where the resource exists. Changing this forces a new resource to be created.
        pub zone_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PtrRecordArgs,
    ) -> PtrRecordResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let records_binding = args.records.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let zone_name_binding = args.zone_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:dns/ptrRecord:PtrRecord".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "records".into(),
                    value: records_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: ttl_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneName".into(),
                    value: zone_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PtrRecordResult {
            fqdn: o.get_field("fqdn"),
            name: o.get_field("name"),
            records: o.get_field("records"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            ttl: o.get_field("ttl"),
            zone_name: o.get_field("zoneName"),
        }
    }
}
