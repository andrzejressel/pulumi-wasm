/// CertificateMapEntry is a list of certificate configurations,
/// that have been issued for a particular hostname
///
///
///
/// ## Example Usage
///
/// ### Certificate Manager Certificate Map Entry Full
///
///
/// ```yaml
/// resources:
///   certificateMap:
///     type: gcp:certificatemanager:CertificateMap
///     name: certificate_map
///     properties:
///       name: cert-map-entry
///       description: My acceptance test certificate map
///       labels:
///         terraform: true
///         acc-test: true
///   default:
///     type: gcp:certificatemanager:CertificateMapEntry
///     properties:
///       name: cert-map-entry
///       description: My acceptance test certificate map entry
///       map: ${certificateMap.name}
///       labels:
///         terraform: true
///         acc-test: true
///       certificates:
///         - ${certificate.id}
///       matcher: PRIMARY
///   certificate:
///     type: gcp:certificatemanager:Certificate
///     properties:
///       name: cert-map-entry
///       description: The default cert
///       scope: DEFAULT
///       managed:
///         domains:
///           - ${instance.domain}
///           - ${instance2.domain}
///         dnsAuthorizations:
///           - ${instance.id}
///           - ${instance2.id}
///   instance:
///     type: gcp:certificatemanager:DnsAuthorization
///     properties:
///       name: dns-auth
///       description: The default dnss
///       domain: subdomain.hashicorptest.com
///   instance2:
///     type: gcp:certificatemanager:DnsAuthorization
///     properties:
///       name: dns-auth2
///       description: The default dnss
///       domain: subdomain2.hashicorptest.com
/// ```
///
/// ## Import
///
/// CertificateMapEntry can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/certificateMaps/{{map}}/certificateMapEntries/{{name}}`
///
/// * `{{project}}/{{map}}/{{name}}`
///
/// * `{{map}}/{{name}}`
///
/// When using the `pulumi import` command, CertificateMapEntry can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/certificateMapEntry:CertificateMapEntry default projects/{{project}}/locations/global/certificateMaps/{{map}}/certificateMapEntries/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/certificateMapEntry:CertificateMapEntry default {{project}}/{{map}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/certificateMapEntry:CertificateMapEntry default {{map}}/{{name}}
/// ```
///
pub mod certificate_map_entry {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateMapEntryArgs {
        /// A set of Certificates defines for the given hostname.
        /// There can be defined up to fifteen certificates in each Certificate Map Entry.
        /// Each certificate must match pattern projects/*/locations/*/certificates/*.
        #[builder(into)]
        pub certificates: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A Hostname (FQDN, e.g. example.com) or a wildcard hostname expression (*.example.com)
        /// for a set of hostnames with common suffix. Used as Server Name Indication (SNI) for
        /// selecting a proper certificate.
        #[builder(into, default)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of labels associated with a Certificate Map Entry.
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map entry that is inputted into the cetrificate map
        ///
        ///
        /// - - -
        #[builder(into)]
        pub map: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A predefined matcher for particular cases, other than SNI selection
        #[builder(into, default)]
        pub matcher: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A user-defined name of the Certificate Map Entry. Certificate Map Entry
        /// names must be unique globally and match pattern
        /// 'projects/*/locations/*/certificateMaps/*/certificateMapEntries/*'
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CertificateMapEntryResult {
        /// A set of Certificates defines for the given hostname.
        /// There can be defined up to fifteen certificates in each Certificate Map Entry.
        /// Each certificate must match pattern projects/*/locations/*/certificates/*.
        pub certificates: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Creation timestamp of a Certificate Map Entry. Timestamp in RFC3339 UTC "Zulu" format,
        /// with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A human-readable description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A Hostname (FQDN, e.g. example.com) or a wildcard hostname expression (*.example.com)
        /// for a set of hostnames with common suffix. Used as Server Name Indication (SNI) for
        /// selecting a proper certificate.
        pub hostname: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of labels associated with a Certificate Map Entry.
        /// An object containing a list of "key": value pairs.
        /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map entry that is inputted into the cetrificate map
        ///
        ///
        /// - - -
        pub map: pulumi_gestalt_rust::Output<String>,
        /// A predefined matcher for particular cases, other than SNI selection
        pub matcher: pulumi_gestalt_rust::Output<Option<String>>,
        /// A user-defined name of the Certificate Map Entry. Certificate Map Entry
        /// names must be unique globally and match pattern
        /// 'projects/*/locations/*/certificateMaps/*/certificateMapEntries/*'
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A serving state of this Certificate Map Entry.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Update timestamp of a Certificate Map Entry. Timestamp in RFC3339 UTC "Zulu" format,
        /// with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CertificateMapEntryArgs,
    ) -> CertificateMapEntryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificates_binding = args.certificates.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let hostname_binding = args.hostname.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let map_binding = args.map.get_output(context).get_inner();
        let matcher_binding = args.matcher.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:certificatemanager/certificateMapEntry:CertificateMapEntry"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificates".into(),
                    value: &certificates_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "map".into(),
                    value: &map_binding,
                },
                register_interface::ObjectField {
                    name: "matcher".into(),
                    value: &matcher_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CertificateMapEntryResult {
            certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificates"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            map: pulumi_gestalt_rust::__private::into_domain(o.extract_field("map")),
            matcher: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("matcher"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
