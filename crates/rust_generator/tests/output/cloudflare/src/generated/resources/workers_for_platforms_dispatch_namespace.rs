/// The [Workers for Platforms](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/) resource allows you
/// to manage Cloudflare Workers for Platforms dispatch namespaces.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:WorkersForPlatformsDispatchNamespace
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
/// $ pulumi import cloudflare:index/workersForPlatformsDispatchNamespace:WorkersForPlatformsDispatchNamespace example <account_id>/<namespace_name>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workers_for_platforms_dispatch_namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersForPlatformsDispatchNamespaceArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Workers for Platforms namespace.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersForPlatformsDispatchNamespaceResult {
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkersForPlatformsDispatchNamespaceArgs,
    ) -> WorkersForPlatformsDispatchNamespaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/workersForPlatformsDispatchNamespace:WorkersForPlatformsDispatchNamespace"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkersForPlatformsDispatchNamespaceResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
