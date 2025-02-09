/// Manages a Route53 Hosted Zone. For managing Domain Name System Security Extensions (DNSSEC), see the `aws.route53.KeySigningKey` and `aws.route53.HostedZoneDnsSec` resources.
///
/// ## Example Usage
///
/// ### Public Zone
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = zone::create(
///         "primary",
///         ZoneArgs::builder().name("example.com").build_struct(),
///     );
/// }
/// ```
///
/// ### Public Subdomain Zone
///
/// For use in subdomains, note that you need to create a
/// `aws.route53.Record` of type `NS` as well as the subdomain
/// zone.
///
/// ```yaml
/// resources:
///   main:
///     type: aws:route53:Zone
///     properties:
///       name: example.com
///   dev:
///     type: aws:route53:Zone
///     properties:
///       name: dev.example.com
///       tags:
///         Environment: dev
///   dev-ns:
///     type: aws:route53:Record
///     properties:
///       zoneId: ${main.zoneId}
///       name: dev.example.com
///       type: NS
///       ttl: '30'
///       records: ${dev.nameServers}
/// ```
///
/// ### Private Zone
///
/// > **NOTE:** This provider provides both exclusive VPC associations defined in-line in this resource via `vpc` configuration blocks and a separate `Zone VPC Association resource. At this time, you cannot use in-line VPC associations in conjunction with any `aws.route53.ZoneAssociation` resources with the same zone ID otherwise it will cause a perpetual difference in plan output. You can optionally use [`ignoreChanges`](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) to manage additional associations via the `aws.route53.ZoneAssociation` resource.
///
/// > **NOTE:** Private zones require at least one VPC association at all times.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let private = zone::create(
///         "private",
///         ZoneArgs::builder()
///             .name("example.com")
///             .vpcs(vec![ZoneVpc::builder().vpcId("${example.id}").build_struct(),])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Zones using the zone `id`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/zone:Zone myzone Z1D633PJN98FT9
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneArgs {
        /// A comment for the hosted zone. Defaults to 'Managed by Pulumi'.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the reusable delegation set whose NS records you want to assign to the hosted zone. Conflicts with `vpc` as delegation sets can only be used for public zones.
        #[builder(into, default)]
        pub delegation_set_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to destroy all records (possibly managed outside of this provider) in the zone when destroying the zone.
        #[builder(into, default)]
        pub force_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// This is the name of the hosted zone.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the zone. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block(s) specifying VPC(s) to associate with a private hosted zone. Conflicts with the `delegation_set_id` argument in this resource and any `aws.route53.ZoneAssociation` resource specifying the same zone ID. Detailed below.
        #[builder(into, default)]
        pub vpcs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::route53::ZoneVpc>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ZoneResult {
        /// The Amazon Resource Name (ARN) of the Hosted Zone.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A comment for the hosted zone. Defaults to 'Managed by Pulumi'.
        pub comment: pulumi_gestalt_rust::Output<String>,
        /// The ID of the reusable delegation set whose NS records you want to assign to the hosted zone. Conflicts with `vpc` as delegation sets can only be used for public zones.
        pub delegation_set_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to destroy all records (possibly managed outside of this provider) in the zone when destroying the zone.
        pub force_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// This is the name of the hosted zone.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of name servers in associated (or default) delegation set.
        /// Find more about delegation sets in [AWS docs](https://docs.aws.amazon.com/Route53/latest/APIReference/actions-on-reusable-delegation-sets.html).
        pub name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Route 53 name server that created the SOA record.
        pub primary_name_server: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the zone. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block(s) specifying VPC(s) to associate with a private hosted zone. Conflicts with the `delegation_set_id` argument in this resource and any `aws.route53.ZoneAssociation` resource specifying the same zone ID. Detailed below.
        pub vpcs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::route53::ZoneVpc>>,
        >,
        /// The Hosted Zone ID. This can be referenced by zone records.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZoneArgs,
    ) -> ZoneResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let comment_binding = args.comment.get_output(context);
        let delegation_set_id_binding = args.delegation_set_id.get_output(context);
        let force_destroy_binding = args.force_destroy.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpcs_binding = args.vpcs.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53/zone:Zone".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: comment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegationSetId".into(),
                    value: delegation_set_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDestroy".into(),
                    value: force_destroy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcs".into(),
                    value: vpcs_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZoneResult {
            arn: o.get_field("arn"),
            comment: o.get_field("comment"),
            delegation_set_id: o.get_field("delegationSetId"),
            force_destroy: o.get_field("forceDestroy"),
            name: o.get_field("name"),
            name_servers: o.get_field("nameServers"),
            primary_name_server: o.get_field("primaryNameServer"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpcs: o.get_field("vpcs"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
