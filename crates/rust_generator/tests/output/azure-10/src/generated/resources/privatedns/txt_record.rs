/// Enables you to manage DNS TXT Records within Azure Private DNS.
///
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
///     let exampleTxtRecord = txt_record::create(
///         "exampleTxtRecord",
///         TxtRecordArgs::builder()
///             .name("test")
///             .records(
///                 vec![TxtRecordRecord::builder().value("v=spf1 mx ~all").build_struct(),],
///             )
///             .resource_group_name("${example.name}")
///             .ttl(300)
///             .zone_name("${exampleZone.name}")
///             .build_struct(),
///     );
///     let exampleZone = zone::create(
///         "exampleZone",
///         ZoneArgs::builder()
///             .name("contoso.com")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Private DNS TXT Records can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:privatedns/txtRecord:TxtRecord test /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/privateDnsZones/contoso.com/TXT/test
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod txt_record {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TxtRecordArgs {
        /// The name of the DNS TXT Record. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `record` blocks as defined below.
        #[builder(into)]
        pub records: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::privatedns::TxtRecordRecord>,
        >,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
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
        /// Specifies the Private DNS Zone where the resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub zone_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TxtRecordResult {
        /// The FQDN of the DNS TXT Record.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// The name of the DNS TXT Record. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `record` blocks as defined below.
        pub records: pulumi_gestalt_rust::Output<
            Vec<super::super::types::privatedns::TxtRecordRecord>,
        >,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Time To Live (TTL) of the DNS record in seconds.
        pub ttl: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the Private DNS Zone where the resource exists. Changing this forces a new resource to be created.
        pub zone_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TxtRecordArgs,
    ) -> TxtRecordResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let records_binding = args.records.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let ttl_binding = args.ttl.get_output(context).get_inner();
        let zone_name_binding = args.zone_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:privatedns/txtRecord:TxtRecord".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "records".into(),
                    value: &records_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
                register_interface::ObjectField {
                    name: "zoneName".into(),
                    value: &zone_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TxtRecordResult {
            fqdn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("fqdn")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            records: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("records"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            ttl: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ttl")),
            zone_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneName"),
            ),
        }
    }
}
