/// Enables you to manage DNS CNAME Records within Azure Private DNS.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleCnameRecord = cname_record::create(
///         "exampleCnameRecord",
///         CnameRecordArgs::builder()
///             .name("test")
///             .record("contoso.com")
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
/// Private DNS CName Records can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:privatedns/cnameRecord:CnameRecord example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/privateDnsZones/zone1/CNAME/myrecord1
/// ```
///
pub mod cname_record {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CnameRecordArgs {
        /// The name of the DNS CNAME Record. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The target of the CNAME.
        #[builder(into)]
        pub record: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Time To Live (TTL) of the DNS record in seconds. Possible values are between `0` and `2147483647`.
        #[builder(into)]
        pub ttl: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Specifies the Private DNS Zone where the resource exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub zone_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CnameRecordResult {
        /// The FQDN of the DNS CNAME Record.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// The name of the DNS CNAME Record. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The target of the CNAME.
        pub record: pulumi_wasm_rust::Output<String>,
        /// Specifies the resource group where the resource exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Time To Live (TTL) of the DNS record in seconds. Possible values are between `0` and `2147483647`.
        pub ttl: pulumi_wasm_rust::Output<i32>,
        /// Specifies the Private DNS Zone where the resource exists. Changing this forces a new resource to be created.
        pub zone_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CnameRecordArgs,
    ) -> CnameRecordResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let record_binding = args.record.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let ttl_binding = args.ttl.get_output(context).get_inner();
        let zone_name_binding = args.zone_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:privatedns/cnameRecord:CnameRecord".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "record".into(),
                    value: &record_binding,
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
        CnameRecordResult {
            fqdn: pulumi_wasm_rust::__private::into_domain(o.extract_field("fqdn")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            record: pulumi_wasm_rust::__private::into_domain(o.extract_field("record")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            ttl: pulumi_wasm_rust::__private::into_domain(o.extract_field("ttl")),
            zone_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("zoneName"),
            ),
        }
    }
}
