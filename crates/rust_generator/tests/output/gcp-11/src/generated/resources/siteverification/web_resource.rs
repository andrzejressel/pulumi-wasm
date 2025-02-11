/// A web resource is a website or domain with verified ownership. Once your
/// ownership is verified you will be able to manage your website in the
/// [Google Search Console](https://www.google.com/webmasters/tools/).
///
/// > **Note:** The verification data (DNS `TXT` record, HTML file, `meta` tag, etc.)
/// must already exist before the web resource is created, and must be deleted before
/// the web resource is destroyed. The Google Site Verification API checks that the
/// verification data exists at creation time and does not exist at destruction time
/// and will fail if the required condition is not met.
///
///
/// To get more information about WebResource, see:
///
/// * [API documentation](https://developers.google.com/site-verification/v1)
/// * How-to Guides
///     * [Getting Started](https://developers.google.com/site-verification/v1/getting_started)
///
/// ## Example Usage
///
/// ### Site Verification Domain Record
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:dns:RecordSet
///     properties:
///       managedZone: example.com
///       name: www.example.com.
///       type: TXT
///       rrdatas:
///         - ${token.token}
///       ttl: 86400
///   exampleWebResource:
///     type: gcp:siteverification:WebResource
///     name: example
///     properties:
///       site:
///         type: ${token.type}
///         identifier: ${token.identifier}
///       verificationMethod: ${token.verificationMethod}
///     options:
///       dependsOn:
///         - ${example}
/// variables:
///   token:
///     fn::invoke:
///       function: gcp:siteverification:getToken
///       arguments:
///         type: INET_DOMAIN
///         identifier: www.example.com
///         verificationMethod: DNS_TXT
/// ```
///
/// ## Import
///
/// WebResource can be imported using any of these accepted formats:
///
/// * `webResource/{{web_resource_id}}`
///
/// * `{{web_resource_id}}`
///
/// When using the `pulumi import` command, WebResource can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:siteverification/webResource:WebResource default webResource/{{web_resource_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:siteverification/webResource:WebResource default {{web_resource_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebResourceArgs {
        /// Container for the address and type of a site for which a verification token will be verified.
        /// Structure is documented below.
        #[builder(into)]
        pub site: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::siteverification::WebResourceSite,
        >,
        /// The verification method for the Site Verification system to use to verify
        /// this site or domain.
        /// Possible values are: `ANALYTICS`, `DNS_CNAME`, `DNS_TXT`, `FILE`, `META`, `TAG_MANAGER`.
        #[builder(into)]
        pub verification_method: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebResourceResult {
        /// The email addresses of all direct, verified owners of this exact property. Indirect owners —
        /// for example verified owners of the containing domain—are not included in this list.
        pub owners: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Container for the address and type of a site for which a verification token will be verified.
        /// Structure is documented below.
        pub site: pulumi_gestalt_rust::Output<
            super::super::types::siteverification::WebResourceSite,
        >,
        /// The verification method for the Site Verification system to use to verify
        /// this site or domain.
        /// Possible values are: `ANALYTICS`, `DNS_CNAME`, `DNS_TXT`, `FILE`, `META`, `TAG_MANAGER`.
        pub verification_method: pulumi_gestalt_rust::Output<String>,
        /// The string used to identify this web resource.
        pub web_resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebResourceArgs,
    ) -> WebResourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let site_binding = args.site.get_output(context);
        let verification_method_binding = args.verification_method.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:siteverification/webResource:WebResource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "site".into(),
                    value: &site_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "verificationMethod".into(),
                    value: &verification_method_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebResourceResult {
            owners: o.get_field("owners"),
            site: o.get_field("site"),
            verification_method: o.get_field("verificationMethod"),
            web_resource_id: o.get_field("webResourceId"),
        }
    }
}
