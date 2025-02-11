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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod automation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
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
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The delivery_pipeline for the resource
        #[builder(into)]
        pub delivery_pipeline: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Description of the 'Automation'. Max length is 255 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the
        /// following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and
        /// dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a
        /// lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values
        /// are additionally constrained to be <= 63 characters. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the `Automation`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. List of Automation rules associated with the Automation resource. Must have at least one rule and limited to 250 rules per Delivery Pipeline. Note: the order of the rules here is not the same as the order of execution.
        /// Structure is documented below.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::clouddeploy::AutomationRule>,
        >,
        /// Required. Selected resources to which the automation will be applied.
        /// Structure is documented below.
        #[builder(into)]
        pub selector: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::clouddeploy::AutomationSelector,
        >,
        /// Required. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources.
        #[builder(into)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. When Suspended, automation is deactivated from execution.
        #[builder(into, default)]
        pub suspended: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
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
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Output only. Time at which the automation was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The delivery_pipeline for the resource
        pub delivery_pipeline: pulumi_gestalt_rust::Output<String>,
        /// Optional. Description of the 'Automation'. Max length is 255 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. The weak etag of the `Automation` resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Optional. Labels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the
        /// following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and
        /// dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a
        /// lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values
        /// are additionally constrained to be <= 63 characters. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the `Automation`.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. List of Automation rules associated with the Automation resource. Must have at least one rule and limited to 250 rules per Delivery Pipeline. Note: the order of the rules here is not the same as the order of execution.
        /// Structure is documented below.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::clouddeploy::AutomationRule>,
        >,
        /// Required. Selected resources to which the automation will be applied.
        /// Structure is documented below.
        pub selector: pulumi_gestalt_rust::Output<
            super::super::types::clouddeploy::AutomationSelector,
        >,
        /// Required. Email address of the user-managed IAM service account that creates Cloud Deploy release and rollout resources.
        pub service_account: pulumi_gestalt_rust::Output<String>,
        /// Optional. When Suspended, automation is deactivated from execution.
        pub suspended: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Output only. Unique identifier of the `Automation`.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. Time at which the automation was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AutomationArgs,
    ) -> AutomationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let delivery_pipeline_binding = args.delivery_pipeline.get_output(context);
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let selector_binding = args.selector.get_output(context);
        let service_account_binding = args.service_account.get_output(context);
        let suspended_binding = args.suspended.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:clouddeploy/automation:Automation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deliveryPipeline".into(),
                    value: &delivery_pipeline_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: &rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selector".into(),
                    value: &selector_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suspended".into(),
                    value: &suspended_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AutomationResult {
            annotations: o.get_field("annotations"),
            create_time: o.get_field("createTime"),
            delivery_pipeline: o.get_field("deliveryPipeline"),
            description: o.get_field("description"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            rules: o.get_field("rules"),
            selector: o.get_field("selector"),
            service_account: o.get_field("serviceAccount"),
            suspended: o.get_field("suspended"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
