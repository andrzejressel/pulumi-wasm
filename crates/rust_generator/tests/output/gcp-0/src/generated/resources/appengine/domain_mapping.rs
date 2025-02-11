/// A domain serving an App Engine application.
///
///
/// To get more information about DomainMapping, see:
///
/// * [API documentation](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.domainMappings)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/appengine/docs/standard/python/mapping-custom-domains)
///
/// ## Example Usage
///
/// ### App Engine Domain Mapping Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let domainMapping = domain_mapping::create(
///         "domainMapping",
///         DomainMappingArgs::builder()
///             .domain_name("verified-domain.com")
///             .ssl_settings(
///                 DomainMappingSslSettings::builder()
///                     .sslManagementType("AUTOMATIC")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// DomainMapping can be imported using any of these accepted formats:
///
/// * `apps/{{project}}/domainMappings/{{domain_name}}`
///
/// * `{{project}}/{{domain_name}}`
///
/// * `{{domain_name}}`
///
/// When using the `pulumi import` command, DomainMapping can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:appengine/domainMapping:DomainMapping default apps/{{project}}/domainMappings/{{domain_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/domainMapping:DomainMapping default {{project}}/{{domain_name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:appengine/domainMapping:DomainMapping default {{domain_name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_mapping {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainMappingArgs {
        /// Relative name of the domain serving the application. Example: example.com.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the domain creation should override any existing mappings for this domain.
        /// By default, overrides are rejected.
        /// Default value is `STRICT`.
        /// Possible values are: `STRICT`, `OVERRIDE`.
        #[builder(into, default)]
        pub override_strategy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// SSL configuration for this domain. If unconfigured, this domain will not serve with SSL.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ssl_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appengine::DomainMappingSslSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainMappingResult {
        /// Relative name of the domain serving the application. Example: example.com.
        ///
        ///
        /// - - -
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Relative name of the object affected by this record. Only applicable for CNAME records. Example: 'www'.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether the domain creation should override any existing mappings for this domain.
        /// By default, overrides are rejected.
        /// Default value is `STRICT`.
        /// Possible values are: `STRICT`, `OVERRIDE`.
        pub override_strategy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The resource records required to configure this domain mapping. These records must be added to the domain's DNS
        /// configuration in order to serve the application via this domain mapping.
        /// Structure is documented below.
        pub resource_records: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appengine::DomainMappingResourceRecord>,
        >,
        /// SSL configuration for this domain. If unconfigured, this domain will not serve with SSL.
        /// Structure is documented below.
        pub ssl_settings: pulumi_gestalt_rust::Output<
            super::super::types::appengine::DomainMappingSslSettings,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainMappingArgs,
    ) -> DomainMappingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let override_strategy_binding = args.override_strategy.get_output(context);
        let project_binding = args.project.get_output(context);
        let ssl_settings_binding = args.ssl_settings.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:appengine/domainMapping:DomainMapping".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overrideStrategy".into(),
                    value: &override_strategy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslSettings".into(),
                    value: &ssl_settings_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainMappingResult {
            domain_name: o.get_field("domainName"),
            name: o.get_field("name"),
            override_strategy: o.get_field("overrideStrategy"),
            project: o.get_field("project"),
            resource_records: o.get_field("resourceRecords"),
            ssl_settings: o.get_field("sslSettings"),
        }
    }
}
