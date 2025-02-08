/// Provides a resource for managing Cloudflare Pages domains.
///
/// > A DNS record for the domain is not automatically created. You need to create
///    a `cloudflare.Record` resource for the domain you want to use.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   my-domain:
///     type: cloudflare:PagesDomain
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       projectName: my-example-project
///       domain: example.com
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/pagesDomain:PagesDomain example <account_id>/<project_name>/<domain-name>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod pages_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PagesDomainArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Custom domain. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Pages Project. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub project_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PagesDomainResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Custom domain. **Modifying this attribute will force creation of a new resource.**
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// Name of the Pages Project. **Modifying this attribute will force creation of a new resource.**
        pub project_name: pulumi_gestalt_rust::Output<String>,
        /// Status of the custom domain.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PagesDomainArgs,
    ) -> PagesDomainResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let domain_binding = args.domain.get_output(context).get_inner();
        let project_name_binding = args.project_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/pagesDomain:PagesDomain".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "projectName".into(),
                    value: &project_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PagesDomainResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domain"),
            ),
            project_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("projectName"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
