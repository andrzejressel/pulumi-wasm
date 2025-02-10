/// The [Workers for Platforms](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/) resource allows you
/// to manage Cloudflare Workers for Platforms namespaces.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:WorkersForPlatformsNamespace
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       name: example-namespace
///   customerWorker1:
///     type: cloudflare:WorkersScript
///     name: customer_worker_1
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       name: customer-worker-1
///       content:
///         fn::invoke:
///           Function: std:file
///           Arguments:
///             input: script.js
///           Return: result
///       dispatchNamespace: ${example.name}
///       tags:
///         - free
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersForPlatformsNamespace:WorkersForPlatformsNamespace example <account_id>/<namespace_name>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workers_for_platforms_namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersForPlatformsNamespaceArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Workers for Platforms namespace.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersForPlatformsNamespaceResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Workers for Platforms namespace.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkersForPlatformsNamespaceArgs,
    ) -> WorkersForPlatformsNamespaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/workersForPlatformsNamespace:WorkersForPlatformsNamespace"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkersForPlatformsNamespaceResult {
            account_id: o.get_field("accountId"),
            name: o.get_field("name"),
        }
    }
}
