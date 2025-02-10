/// Represents a PublicDelegatedPrefix for use with bring your own IP addresses (BYOIP).
///
///
/// To get more information about PublicDelegatedPrefix, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/publicDelegatedPrefixes)
/// * How-to Guides
///     * [Using bring your own IP](https://cloud.google.com/vpc/docs/using-bring-your-own-ip)
///
/// ## Example Usage
///
/// ### Public Delegated Prefixes Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let advertised = public_advertised_prefix::create(
///         "advertised",
///         PublicAdvertisedPrefixArgs::builder()
///             .description("description")
///             .dns_verification_ip("127.127.0.0")
///             .ip_cidr_range("127.127.0.0/16")
///             .name("my-prefix")
///             .build_struct(),
///     );
///     let prefixes = public_delegated_prefix::create(
///         "prefixes",
///         PublicDelegatedPrefixArgs::builder()
///             .description("my description")
///             .ip_cidr_range("127.127.0.0/24")
///             .name("my-prefix")
///             .parent_prefix("${advertised.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PublicDelegatedPrefix can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/publicDelegatedPrefixes/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, PublicDelegatedPrefix can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/publicDelegatedPrefix:PublicDelegatedPrefix default projects/{{project}}/regions/{{region}}/publicDelegatedPrefixes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/publicDelegatedPrefix:PublicDelegatedPrefix default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/publicDelegatedPrefix:PublicDelegatedPrefix default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/publicDelegatedPrefix:PublicDelegatedPrefix default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod public_delegated_prefix {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PublicDelegatedPrefixArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 address range, in CIDR format, represented by this public advertised prefix.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub ip_cidr_range: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If true, the prefix will be live migrated.
        #[builder(into, default)]
        pub is_live_migration: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of parent prefix. Either PublicAdvertisedPrefix or PublicDelegatedPrefix.
        #[builder(into)]
        pub parent_prefix: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A region where the prefix will reside.
        #[builder(into)]
        pub region: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PublicDelegatedPrefixResult {
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IPv4 address range, in CIDR format, represented by this public advertised prefix.
        ///
        ///
        /// - - -
        pub ip_cidr_range: pulumi_gestalt_rust::Output<String>,
        /// If true, the prefix will be live migrated.
        pub is_live_migration: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The URL of parent prefix. Either PublicAdvertisedPrefix or PublicDelegatedPrefix.
        pub parent_prefix: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A region where the prefix will reside.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PublicDelegatedPrefixArgs,
    ) -> PublicDelegatedPrefixResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let ip_cidr_range_binding = args.ip_cidr_range.get_output(context);
        let is_live_migration_binding = args.is_live_migration.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_prefix_binding = args.parent_prefix.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/publicDelegatedPrefix:PublicDelegatedPrefix".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipCidrRange".into(),
                    value: ip_cidr_range_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isLiveMigration".into(),
                    value: is_live_migration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentPrefix".into(),
                    value: parent_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PublicDelegatedPrefixResult {
            description: o.get_field("description"),
            ip_cidr_range: o.get_field("ipCidrRange"),
            is_live_migration: o.get_field("isLiveMigration"),
            name: o.get_field("name"),
            parent_prefix: o.get_field("parentPrefix"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
        }
    }
}
