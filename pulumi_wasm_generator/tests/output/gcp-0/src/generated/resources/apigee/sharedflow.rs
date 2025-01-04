/// You can combine policies and resources into a shared flow that you can consume from multiple API proxies, and even from other shared flows. Although it's like a proxy, a shared flow has no endpoint. It can be used only from an API proxy or shared flow that's in the same organization as the shared flow itself.
///
///
/// To get more information about SharedFlow, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.sharedflows)
/// * How-to Guides
///     * [Sharedflows](https://cloud.google.com/apigee/docs/resources)
///
/// ## Import
///
/// SharedFlow can be imported using any of these accepted formats:
///
/// * `{{org_id}}/sharedflows/{{name}}`
///
/// * `{{org_id}}/{{name}}`
///
/// When using the `pulumi import` command, SharedFlow can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/sharedflow:Sharedflow default {{org_id}}/sharedflows/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/sharedflow:Sharedflow default {{org_id}}/{{name}}
/// ```
///
pub mod sharedflow {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedflowArgs {
        /// Path to the config zip bundle.
        ///
        /// - - -
        #[builder(into)]
        pub config_bundle: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub detect_md5hash: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the shared flow.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apigee Organization name associated with the Apigee instance.
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SharedflowResult {
        /// Path to the config zip bundle.
        ///
        /// - - -
        pub config_bundle: pulumi_wasm_rust::Output<String>,
        pub detect_md5hash: pulumi_wasm_rust::Output<Option<String>>,
        /// The id of the most recently created revision for this shared flow.
        pub latest_revision_id: pulumi_wasm_rust::Output<String>,
        /// (Computed) Base 64 MD5 hash of the uploaded data. It is speculative as remote does not return hash of the bundle. Remote changes are detected using returned last_modified timestamp.
        pub md5hash: pulumi_wasm_rust::Output<String>,
        /// Metadata describing the shared flow.
        /// Structure is documented below.
        pub meta_datas: pulumi_wasm_rust::Output<
            Vec<super::super::types::apigee::SharedflowMetaData>,
        >,
        /// The ID of the shared flow.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Apigee Organization name associated with the Apigee instance.
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// A list of revisions of this shared flow.
        pub revisions: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SharedflowArgs) -> SharedflowResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let config_bundle_binding = args.config_bundle.get_inner();
        let detect_md5hash_binding = args.detect_md5hash.get_inner();
        let name_binding = args.name.get_inner();
        let org_id_binding = args.org_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/sharedflow:Sharedflow".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configBundle".into(),
                    value: &config_bundle_binding,
                },
                register_interface::ObjectField {
                    name: "detectMd5hash".into(),
                    value: &detect_md5hash_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configBundle".into(),
                },
                register_interface::ResultField {
                    name: "detectMd5hash".into(),
                },
                register_interface::ResultField {
                    name: "latestRevisionId".into(),
                },
                register_interface::ResultField {
                    name: "md5hash".into(),
                },
                register_interface::ResultField {
                    name: "metaDatas".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
                },
                register_interface::ResultField {
                    name: "revisions".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SharedflowResult {
            config_bundle: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configBundle").unwrap(),
            ),
            detect_md5hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectMd5hash").unwrap(),
            ),
            latest_revision_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestRevisionId").unwrap(),
            ),
            md5hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("md5hash").unwrap(),
            ),
            meta_datas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metaDatas").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
            revisions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revisions").unwrap(),
            ),
        }
    }
}
