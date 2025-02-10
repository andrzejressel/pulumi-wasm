/// DnsAuthorization represents a HTTP-reachable backend for a DnsAuthorization.
///
///
///
/// ## Example Usage
///
/// ### Certificate Manager Dns Authorization Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = dns_authorization::create(
///         "default",
///         DnsAuthorizationArgs::builder()
///             .description("The default dns")
///             .domain("subdomain.hashicorptest.com")
///             .location("global")
///             .name("dns-auth")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Certificate Manager Dns Authorization Regional
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = dns_authorization::create(
///         "default",
///         DnsAuthorizationArgs::builder()
///             .description("reginal dns")
///             .domain("subdomain.hashicorptest.com")
///             .location("us-central1")
///             .name("dns-auth")
///             .type_("PER_PROJECT_RECORD")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// DnsAuthorization can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/dnsAuthorizations/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, DnsAuthorization can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/dnsAuthorization:DnsAuthorization default projects/{{project}}/locations/{{location}}/dnsAuthorizations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/dnsAuthorization:DnsAuthorization default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/dnsAuthorization:DnsAuthorization default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dns_authorization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DnsAuthorizationArgs {
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A domain which is being authorized. A DnsAuthorization resource covers a
        /// single domain and its wildcard, e.g. authorization for "example.com" can
        /// be used to issue certificates for "example.com" and "*.example.com".
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set of label tags associated with the DNS Authorization resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Certificate Manager location. If not specified, "global" is used.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// type of DNS authorization. If unset during the resource creation, FIXED_RECORD will
        /// be used for global resources, and PER_PROJECT_RECORD will be used for other locations.
        /// FIXED_RECORD DNS authorization uses DNS-01 validation method
        /// PER_PROJECT_RECORD DNS authorization allows for independent management
        /// of Google-managed certificates with DNS authorization across multiple
        /// projects.
        /// Possible values are: `FIXED_RECORD`, `PER_PROJECT_RECORD`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DnsAuthorizationResult {
        /// A human-readable description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The structure describing the DNS Resource Record that needs to be added
        /// to DNS configuration for the authorization to be usable by
        /// certificate.
        /// Structure is documented below.
        pub dns_resource_records: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::certificatemanager::DnsAuthorizationDnsResourceRecord,
            >,
        >,
        /// A domain which is being authorized. A DnsAuthorization resource covers a
        /// single domain and its wildcard, e.g. authorization for "example.com" can
        /// be used to issue certificates for "example.com" and "*.example.com".
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of label tags associated with the DNS Authorization resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Certificate Manager location. If not specified, "global" is used.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// type of DNS authorization. If unset during the resource creation, FIXED_RECORD will
        /// be used for global resources, and PER_PROJECT_RECORD will be used for other locations.
        /// FIXED_RECORD DNS authorization uses DNS-01 validation method
        /// PER_PROJECT_RECORD DNS authorization allows for independent management
        /// of Google-managed certificates with DNS authorization across multiple
        /// projects.
        /// Possible values are: `FIXED_RECORD`, `PER_PROJECT_RECORD`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DnsAuthorizationArgs,
    ) -> DnsAuthorizationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:certificatemanager/dnsAuthorization:DnsAuthorization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DnsAuthorizationResult {
            description: o.get_field("description"),
            dns_resource_records: o.get_field("dnsResourceRecords"),
            domain: o.get_field("domain"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            type_: o.get_field("type"),
        }
    }
}
