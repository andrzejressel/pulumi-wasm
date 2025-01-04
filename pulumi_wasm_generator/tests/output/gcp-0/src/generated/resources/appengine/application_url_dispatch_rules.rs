/// Rules to match an HTTP request and dispatch that request to a service.
///
///
/// To get more information about ApplicationUrlDispatchRules, see:
///
/// * [API documentation](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps#UrlDispatchRule)
///
/// ## Example Usage
///
/// ### App Engine Application Url Dispatch Rules Basic
///
///
/// ```yaml
/// resources:
///   webService:
///     type: gcp:appengine:ApplicationUrlDispatchRules
///     name: web_service
///     properties:
///       dispatchRules:
///         - domain: '*'
///           path: /*
///           service: default
///         - domain: '*'
///           path: /admin/*
///           service: ${adminV3.service}
///   adminV3:
///     type: gcp:appengine:StandardAppVersion
///     name: admin_v3
///     properties:
///       versionId: v3
///       service: admin
///       runtime: nodejs20
///       entrypoint:
///         shell: node ./app.js
///       deployment:
///         zip:
///           sourceUrl: https://storage.googleapis.com/${bucket.name}/${object.name}
///       envVariables:
///         port: '8080'
///       deleteServiceOnDestroy: true
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: appengine-test-bucket
///       location: US
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: hello-world.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: ./test-fixtures/hello-world.zip
/// ```
///
/// ## Import
///
/// ApplicationUrlDispatchRules can be imported using any of these accepted formats:
///
/// * `{{project}}`
///
/// When using the `pulumi import` command, ApplicationUrlDispatchRules can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:appengine/applicationUrlDispatchRules:ApplicationUrlDispatchRules default {{project}}
/// ```
///
pub mod application_url_dispatch_rules {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationUrlDispatchRulesArgs {
        /// Rules to match an HTTP request and dispatch that request to a service.
        /// Structure is documented below.
        #[builder(into)]
        pub dispatch_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::appengine::ApplicationUrlDispatchRulesDispatchRule>,
        >,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApplicationUrlDispatchRulesResult {
        /// Rules to match an HTTP request and dispatch that request to a service.
        /// Structure is documented below.
        pub dispatch_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::appengine::ApplicationUrlDispatchRulesDispatchRule>,
        >,
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ApplicationUrlDispatchRulesArgs,
    ) -> ApplicationUrlDispatchRulesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dispatch_rules_binding = args.dispatch_rules.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:appengine/applicationUrlDispatchRules:ApplicationUrlDispatchRules"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dispatchRules".into(),
                    value: &dispatch_rules_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dispatchRules".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationUrlDispatchRulesResult {
            dispatch_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dispatchRules").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
