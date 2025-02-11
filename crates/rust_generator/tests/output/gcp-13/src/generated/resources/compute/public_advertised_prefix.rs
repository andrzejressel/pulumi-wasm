/// Represents a PublicAdvertisedPrefix for use with bring your own IP addresses (BYOIP).
///
///
/// To get more information about PublicAdvertisedPrefix, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/publicAdvertisedPrefixes)
/// * How-to Guides
///     * [Using bring your own IP](https://cloud.google.com/vpc/docs/using-bring-your-own-ip)
///
/// ## Example Usage
///
/// ### Public Advertised Prefixes Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let prefixes = public_advertised_prefix::create(
///         "prefixes",
///         PublicAdvertisedPrefixArgs::builder()
///             .description("description")
///             .dns_verification_ip("127.127.0.0")
///             .ip_cidr_range("127.127.0.0/16")
///             .name("my-prefix")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PublicAdvertisedPrefix can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/publicAdvertisedPrefixes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, PublicAdvertisedPrefix can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/publicAdvertisedPrefix:PublicAdvertisedPrefix default projects/{{project}}/global/publicAdvertisedPrefixes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/publicAdvertisedPrefix:PublicAdvertisedPrefix default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/publicAdvertisedPrefix:PublicAdvertisedPrefix default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod public_advertised_prefix {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PublicAdvertisedPrefixArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 address to be used for reverse DNS verification.
        #[builder(into)]
        pub dns_verification_ip: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IPv4 address range, in CIDR format, represented by this public advertised prefix.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub ip_cidr_range: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PublicAdvertisedPrefixResult {
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IPv4 address to be used for reverse DNS verification.
        pub dns_verification_ip: pulumi_gestalt_rust::Output<String>,
        /// The IPv4 address range, in CIDR format, represented by this public advertised prefix.
        ///
        ///
        /// - - -
        pub ip_cidr_range: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Output Only. The shared secret to be used for reverse DNS verification.
        pub shared_secret: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PublicAdvertisedPrefixArgs,
    ) -> PublicAdvertisedPrefixResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let dns_verification_ip_binding = args.dns_verification_ip.get_output(context);
        let ip_cidr_range_binding = args.ip_cidr_range.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/publicAdvertisedPrefix:PublicAdvertisedPrefix".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsVerificationIp".into(),
                    value: &dns_verification_ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipCidrRange".into(),
                    value: &ip_cidr_range_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PublicAdvertisedPrefixResult {
            description: o.get_field("description"),
            dns_verification_ip: o.get_field("dnsVerificationIp"),
            ip_cidr_range: o.get_field("ipCidrRange"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            shared_secret: o.get_field("sharedSecret"),
        }
    }
}
