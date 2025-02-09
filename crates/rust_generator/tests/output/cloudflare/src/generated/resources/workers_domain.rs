/// Creates a Worker Custom Domain.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workers_domain::create(
///         "example",
///         WorkersDomainArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .hostname("subdomain.example.com")
///             .service("my-service")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersDomain:WorkersDomain example <account_id>/<worker_domain_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workers_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersDomainArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Worker environment. Defaults to `production`.
        #[builder(into, default)]
        pub environment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Hostname of the Worker Domain.
        #[builder(into)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of worker script to attach the domain to.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersDomainResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Worker environment. Defaults to `production`.
        pub environment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Hostname of the Worker Domain.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// Name of worker script to attach the domain to.
        pub service: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkersDomainArgs,
    ) -> WorkersDomainResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let environment_binding_1 = args.environment.get_output(context);
        let environment_binding = environment_binding_1.get_inner();
        let hostname_binding_1 = args.hostname.get_output(context);
        let hostname_binding = hostname_binding_1.get_inner();
        let service_binding_1 = args.service.get_output(context);
        let service_binding = service_binding_1.get_inner();
        let zone_id_binding_1 = args.zone_id.get_output(context);
        let zone_id_binding = zone_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/workersDomain:WorkersDomain".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkersDomainResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            environment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environment"),
            ),
            hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("service"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
