/// Provides a resource to manage a Cloudflare Workers KV Pair.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workers_kv::create(
///         "example",
///         WorkersKvArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .key("test-key")
///             .namespace_id("${exampleNs.id}")
///             .value("test value")
///             .build_struct(),
///     );
///     let exampleNs = workers_kv_namespace::create(
///         "exampleNs",
///         WorkersKvNamespaceArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .title("test-namespace")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/workersKv:WorkersKv example <account_id>/<namespace_id>/<key_name>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workers_kv {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkersKvArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the KV pair. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Workers KV namespace in which you want to create the KV pair. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub namespace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Value of the KV pair.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkersKvResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the KV pair. **Modifying this attribute will force creation of a new resource.**
        pub key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Workers KV namespace in which you want to create the KV pair. **Modifying this attribute will force creation of a new resource.**
        pub namespace_id: pulumi_gestalt_rust::Output<String>,
        /// Value of the KV pair.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkersKvArgs,
    ) -> WorkersKvResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let key_binding = args.key.get_output(context).get_inner();
        let namespace_id_binding = args.namespace_id.get_output(context).get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/workersKv:WorkersKv".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceId".into(),
                    value: &namespace_id_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkersKvResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            namespace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namespaceId"),
            ),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
