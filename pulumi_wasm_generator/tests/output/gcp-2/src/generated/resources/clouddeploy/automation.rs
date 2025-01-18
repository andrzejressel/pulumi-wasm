/// An `Automation` enables the automation of manually driven actions for a Delivery Pipeline, which includes Release promotion amongst Targets, Rollout repair and Rollout deployment strategy advancement.
///
///
/// To get more information about Automation, see:
///
/// * [API documentation](https://cloud.google.com/deploy/docs/api/reference/rest/v1/projects.locations.deliveryPipelines.automations)
/// * How-to Guides
///     * [Automate your deployment](https://cloud.google.com/deploy/docs/automation)
///
/// ## Example Usage
///
/// ### Clouddeploy Automation Basic
///
///
/// ```yaml
/// resources:
///   b-automation:
///     type: gcp:clouddeploy:Automation
///     properties:
///       name: cd-automation
///       project: ${pipeline.project}
///       location: ${pipeline.location}
///       deliveryPipeline: ${pipeline.name}
///       serviceAccount: my@service-account.com
///       selector:
///         targets:
///           - id: '*'
///       suspended: false
///       rules:
///         - promoteReleaseRule:
///             id: promote-release
///   pipeline:
///     type: gcp:clouddeploy:DeliveryPipeline
///     properties:
///       name: cd-pipeline
///       location: us-central1
///       serialPipeline:
///         stages:
///           - targetId: test
///             profiles: []
/// ```
/// ### Clouddeploy Automation Full
///
///
/// ```yaml
/// resources:
///   f-automation:
///     type: gcp:clouddeploy:Automation
///     properties:
///       name: cd-automation
///       location: us-central1
///       deliveryPipeline: ${pipeline.name}
///       serviceAccount: my@service-account.com
///       annotations:
///         my_first_annotation: example-annotation-1
///         my_second_annotation: example-annotation-2
///       labels:
///         my_first_label: example-label-1
///         my_second_label: example-label-2
///       description: automation resource
///       selector:
///         targets:
///           - id: test
///             labels:
///               foo: bar
///       suspended: true
///       rules:
///         - promoteReleaseRule:
///             id: promote-release
///             wait: 200s
///             destinationTargetId: '@next'
///             destinationPhase: stable
///         - advanceRolloutRule:
///             id: advance-rollout
///             sourcePhases:
///               - deploy
///             wait: 200s
///   pipeline:
///     type: gcp:clouddeploy:DeliveryPipeline
///     properties:
///       name: cd-pipeline
///       location: us-central1
///       serialPipeline:
///         stages:
///           - targetId: test
///             profiles:
///               - test-profile
/// ```
///
/// ## Import
///
/// Automation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/deliveryPipelines/{{delivery_pipeline}}/automations/{{name}}`
///
/// * `{{project}}/{{location}}/{{delivery_pipeline}}/{{name}}`
///
/// * `{{location}}/{{delivery_pipeline}}/{{name}}`
///
/// When using the `pulumi import` command, Automation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/automation:Automation default projects/{{project}}/locations/{{location}}/deliveryPipelines/{{delivery_pipeline}}/automations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/automation:Automation default {{project}}/{{location}}/{{delivery_pipeline}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/automation:Automation default {{location}}/{{delivery_pipeline}}/{{name}}
/// ```
///
pub mod automation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutomationArgs {
        /// Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations
        /// must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an
        /// optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less,
        /// beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'),
        /// and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS
        /// labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See
        /// https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more
        /// details. **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field 'effective_annotations' for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The delivery_pipeline for the resource
        #[builder(into)]
        pub delivery_pipeline: pulumi_wasm_rust::Output<String>,
        /// Optional. Description of the 'Automation'. Max length is 255 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the
        /// following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and
        /// dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a
        /// lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values
        /// are additionally constrained to be <= 63 characters. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the `Automation`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. List of Automation rules associated with the Automation resource. Must have at least one rule and limited to 250 rules per Delivery Pipeline. Note: the order of the rules here is not the same as the order of execution.
        /// Structure is documented below.
        #[builder(into)]
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::clouddeploy::AutomationRule>,
        >,
        /// Required. Selected resources to which the automation will be applied.
        /// Structure is documented below.
        #[builder(into)]
        pub selector: pulumi_wasm_rust::Output<
            super::super::types::clouddeploy::AutomationSelector,
        >,
        /// Required. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources.
        #[builder(into)]
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// Optional. When Suspended, automation is deactivated from execution.
        #[builder(into, default)]
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct AutomationResult {
        /// Optional. User annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. Annotations
        /// must meet the following constraints: * Annotations are key/value pairs. * Valid annotation keys have two segments: an
        /// optional prefix and name, separated by a slash ('/'). * The name segment is required and must be 63 characters or less,
        /// beginning and ending with an alphanumeric character ('[a-z0-9A-Z]') with dashes ('-'), underscores ('_'), dots ('.'),
        /// and alphanumerics between. * The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS
        /// labels separated by dots('.'), not longer than 253 characters in total, followed by a slash ('/'). See
        /// https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/#syntax-and-character-set for more
        /// details. **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field 'effective_annotations' for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Output only. Time at which the automation was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The delivery_pipeline for the resource
        pub delivery_pipeline: pulumi_wasm_rust::Output<String>,
        /// Optional. Description of the 'Automation'. Max length is 255 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. The weak etag of the `Automation` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the
        /// following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and
        /// dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a
        /// lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values
        /// are additionally constrained to be <= 63 characters. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the `Automation`.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. List of Automation rules associated with the Automation resource. Must have at least one rule and limited to 250 rules per Delivery Pipeline. Note: the order of the rules here is not the same as the order of execution.
        /// Structure is documented below.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::clouddeploy::AutomationRule>,
        >,
        /// Required. Selected resources to which the automation will be applied.
        /// Structure is documented below.
        pub selector: pulumi_wasm_rust::Output<
            super::super::types::clouddeploy::AutomationSelector,
        >,
        /// Required. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources.
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// Optional. When Suspended, automation is deactivated from execution.
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
        /// Output only. Unique identifier of the `Automation`.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. Time at which the automation was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AutomationArgs) -> AutomationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let delivery_pipeline_binding = args.delivery_pipeline.get_inner();
        let description_binding = args.description.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let rules_binding = args.rules.get_inner();
        let selector_binding = args.selector.get_inner();
        let service_account_binding = args.service_account.get_inner();
        let suspended_binding = args.suspended.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:clouddeploy/automation:Automation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "deliveryPipeline".into(),
                    value: &delivery_pipeline_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "selector".into(),
                    value: &selector_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "suspended".into(),
                    value: &suspended_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deliveryPipeline".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "selector".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "suspended".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AutomationResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            delivery_pipeline: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryPipeline").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            selector: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selector").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            suspended: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suspended").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
