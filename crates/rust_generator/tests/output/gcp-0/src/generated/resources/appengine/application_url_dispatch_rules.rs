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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_url_dispatch_rules {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationUrlDispatchRulesArgs {
        /// Rules to match an HTTP request and dispatch that request to a service.
        /// Structure is documented below.
        #[builder(into)]
        pub dispatch_rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::appengine::ApplicationUrlDispatchRulesDispatchRule>,
        >,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApplicationUrlDispatchRulesResult {
        /// Rules to match an HTTP request and dispatch that request to a service.
        /// Structure is documented below.
        pub dispatch_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appengine::ApplicationUrlDispatchRulesDispatchRule>,
        >,
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationUrlDispatchRulesArgs,
    ) -> ApplicationUrlDispatchRulesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dispatch_rules_binding = args.dispatch_rules.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:appengine/applicationUrlDispatchRules:ApplicationUrlDispatchRules"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dispatchRules".into(),
                    value: &dispatch_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationUrlDispatchRulesResult {
            dispatch_rules: o.get_field("dispatchRules"),
            project: o.get_field("project"),
        }
    }
}
