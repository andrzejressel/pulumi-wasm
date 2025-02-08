/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleZone:
///     type: azure:dns:Zone
///     name: example
///     properties:
///       name: mydomain.com
///       resourceGroupName: ${example.name}
///   exampleMxRecord:
///     type: azure:dns:MxRecord
///     name: example
///     properties:
///       name: test
///       zoneName: ${exampleZone.name}
///       resourceGroupName: ${example.name}
///       ttl: 300
///       records:
///         - preference: 10
///           exchange: mail1.contoso.com
///         - preference: 20
///           exchange: mail2.contoso.com
///       tags:
///         Environment: Production
/// ```
///
/// ## Import
///
/// MX records can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:dns/mxRecord:MxRecord example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/dnsZones/zone1/MX/myrecord1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mx_record {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MxRecordArgs {
        /// The name of the DNS MX Record. Defaults to `@` (root). Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of values that make up the MX record. Each `record` block supports fields documented below.
        #[builder(into)]
        pub records: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::dns::MxRecordRecord>,
        >,
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
    pub struct MxRecordResult {
        /// The FQDN of the DNS MX Record.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// The name of the DNS MX Record. Defaults to `@` (root). Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of values that make up the MX record. Each `record` block supports fields documented below.
        pub records: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dns::MxRecordRecord>,
        >,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MxRecordArgs,
    ) -> MxRecordResult {
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
            type_: "azure:dns/mxRecord:MxRecord".into(),
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
        MxRecordResult {
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
