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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod domain_mapping {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainMappingArgs {
        /// Relative name of the domain serving the application. Example: example.com.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether the domain creation should override any existing mappings for this domain.
        /// By default, overrides are rejected.
        /// Default value is `STRICT`.
        /// Possible values are: `STRICT`, `OVERRIDE`.
        #[builder(into, default)]
        pub override_strategy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// SSL configuration for this domain. If unconfigured, this domain will not serve with SSL.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ssl_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appengine::DomainMappingSslSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainMappingResult {
        /// Relative name of the domain serving the application. Example: example.com.
        ///
        ///
        /// - - -
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Relative name of the object affected by this record. Only applicable for CNAME records. Example: 'www'.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether the domain creation should override any existing mappings for this domain.
        /// By default, overrides are rejected.
        /// Default value is `STRICT`.
        /// Possible values are: `STRICT`, `OVERRIDE`.
        pub override_strategy: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The resource records required to configure this domain mapping. These records must be added to the domain's DNS
        /// configuration in order to serve the application via this domain mapping.
        /// Structure is documented below.
        pub resource_records: pulumi_wasm_rust::Output<
            Vec<super::super::types::appengine::DomainMappingResourceRecord>,
        >,
        /// SSL configuration for this domain. If unconfigured, this domain will not serve with SSL.
        /// Structure is documented below.
        pub ssl_settings: pulumi_wasm_rust::Output<
            super::super::types::appengine::DomainMappingSslSettings,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DomainMappingArgs,
    ) -> DomainMappingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let override_strategy_binding = args
            .override_strategy
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let ssl_settings_binding = args.ssl_settings.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:appengine/domainMapping:DomainMapping".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "overrideStrategy".into(),
                    value: &override_strategy_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "sslSettings".into(),
                    value: &ssl_settings_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DomainMappingResult {
            domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            override_strategy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("overrideStrategy"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            resource_records: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceRecords"),
            ),
            ssl_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sslSettings"),
            ),
        }
    }
}
