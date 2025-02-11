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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PagesDomainArgs,
    ) -> PagesDomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let project_name_binding = args.project_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/pagesDomain:PagesDomain".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: &domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectName".into(),
                    value: &project_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PagesDomainResult {
            account_id: o.get_field("accountId"),
            domain: o.get_field("domain"),
            project_name: o.get_field("projectName"),
            status: o.get_field("status"),
        }
    }
}
