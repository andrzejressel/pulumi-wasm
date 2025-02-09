/// Provides a Cloudflare record resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let _sipTls = record::create(
///         "_sipTls",
///         RecordArgs::builder()
///             .data(
///                 RecordData::builder()
///                     .name("terraform-srv")
///                     .port(443)
///                     .priority(0)
///                     .proto("_tls")
///                     .service("_sip")
///                     .target("example.com")
///                     .weight(0)
///                     .build_struct(),
///             )
///             .name("_sip._tls")
///             .type_("SRV")
///             .zone_id("${cloudflareZoneId}")
///             .build_struct(),
///     );
///     let example = record::create(
///         "example",
///         RecordArgs::builder()
///             .content("192.0.2.1")
///             .name("terraform")
///             .ttl(3600)
///             .type_("A")
///             .zone_id("${cloudflareZoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/record:Record example <zone_id>/<record_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod record {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecordArgs {
        #[builder(into, default)]
        pub allow_overwrite: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Comments or notes about the DNS record. This field has no effect on DNS responses.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The content of the record. Must provide only one of `data`, `content`, `value`.
        #[builder(into, default)]
        pub content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of attributes that constitute the record value. Must provide only one of `data`, `content`, `value`.
        #[builder(into, default)]
        pub data: pulumi_gestalt_rust::InputOrOutput<Option<super::types::RecordData>>,
        /// The name of the record. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The priority of the record.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether the record gets Cloudflare's origin protection.
        #[builder(into, default)]
        pub proxied: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Custom tags for the DNS record.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The TTL of the record.
        #[builder(into, default)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`, `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`, `SVCB`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The value of the record. Must provide only one of `data`, `content`, `value`.
        #[builder(into, default)]
        pub value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RecordResult {
        pub allow_overwrite: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Comments or notes about the DNS record. This field has no effect on DNS responses.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The content of the record. Must provide only one of `data`, `content`, `value`.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// The RFC3339 timestamp of when the record was created.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// Map of attributes that constitute the record value. Must provide only one of `data`, `content`, `value`.
        pub data: pulumi_gestalt_rust::Output<Option<super::types::RecordData>>,
        /// The FQDN of the record.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// A key-value map of string metadata Cloudflare associates with the record.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The RFC3339 timestamp of when the record was last modified.
        pub modified_on: pulumi_gestalt_rust::Output<String>,
        /// The name of the record. **Modifying this attribute will force creation of a new resource.**
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The priority of the record.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Shows whether this record can be proxied.
        pub proxiable: pulumi_gestalt_rust::Output<bool>,
        /// Whether the record gets Cloudflare's origin protection.
        pub proxied: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Custom tags for the DNS record.
        pub tags: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The TTL of the record.
        pub ttl: pulumi_gestalt_rust::Output<i32>,
        /// The type of the record. Available values: `A`, `AAAA`, `CAA`, `CNAME`, `TXT`, `SRV`, `LOC`, `MX`, `NS`, `SPF`, `CERT`, `DNSKEY`, `DS`, `NAPTR`, `SMIMEA`, `SSHFP`, `TLSA`, `URI`, `PTR`, `HTTPS`, `SVCB`. **Modifying this attribute will force creation of a new resource.**
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The value of the record. Must provide only one of `data`, `content`, `value`.
        pub value: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RecordArgs,
    ) -> RecordResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_overwrite_binding = args.allow_overwrite.get_output(context);
        let comment_binding = args.comment.get_output(context);
        let content_binding = args.content.get_output(context);
        let data_binding = args.data.get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let proxied_binding = args.proxied.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let type__binding = args.type_.get_output(context);
        let value_binding = args.value.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/record:Record".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowOverwrite".into(),
                    value: allow_overwrite_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: comment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "content".into(),
                    value: content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "data".into(),
                    value: data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxied".into(),
                    value: proxied_binding.get_id(),
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
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: value_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RecordResult {
            allow_overwrite: o.get_field("allowOverwrite"),
            comment: o.get_field("comment"),
            content: o.get_field("content"),
            created_on: o.get_field("createdOn"),
            data: o.get_field("data"),
            hostname: o.get_field("hostname"),
            metadata: o.get_field("metadata"),
            modified_on: o.get_field("modifiedOn"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            proxiable: o.get_field("proxiable"),
            proxied: o.get_field("proxied"),
            tags: o.get_field("tags"),
            ttl: o.get_field("ttl"),
            type_: o.get_field("type"),
            value: o.get_field("value"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
