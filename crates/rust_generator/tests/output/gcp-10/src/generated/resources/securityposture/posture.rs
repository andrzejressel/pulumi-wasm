/// A Posture represents a collection of policy set including its name, state, description
/// and policy sets. A policy set includes set of policies along with their definition.
/// A posture can be created at the organization level.
/// Every update to a deployed posture creates a new posture revision with an updated revision_id.
///
///
/// To get more information about Posture, see:
/// * How-to Guides
///     * [Create and deploy a posture](https://cloud.google.com/security-command-center/docs/how-to-use-security-posture)
///
/// ## Example Usage
///
/// ### Securityposture Posture Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let posture1 = posture::create(
///         "posture1",
///         PostureArgs::builder()
///             .description("a new posture")
///             .location("global")
///             .parent("organizations/123456789")
///             .policy_sets(
///                 vec![
///                     PosturePolicySet::builder().description("set of org policies")
///                     .policies(vec![PosturePolicySetPolicy::builder()
///                     .constraint(PosturePolicySetPolicyConstraint::builder()
///                     .orgPolicyConstraint(PosturePolicySetPolicyConstraintOrgPolicyConstraint::builder()
///                     .cannedConstraintId("storage.uniformBucketLevelAccess")
///                     .policyRules(vec![PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRule::builder()
///                     .condition(PosturePolicySetPolicyConstraintOrgPolicyConstraintPolicyRuleCondition::builder()
///                     .description("condition description")
///                     .expression("resource.matchTag('org_id/tag_key_short_name,'tag_value_short_name')")
///                     .title("a CEL condition").build_struct()).enforce(true)
///                     .build_struct(),]).build_struct()).build_struct())
///                     .policyId("canned_org_policy").build_struct(),
///                     PosturePolicySetPolicy::builder()
///                     .constraint(PosturePolicySetPolicyConstraint::builder()
///                     .orgPolicyConstraintCustom(PosturePolicySetPolicyConstraintOrgPolicyConstraintCustom::builder()
///                     .customConstraint(PosturePolicySetPolicyConstraintOrgPolicyConstraintCustomCustomConstraint::builder()
///                     .actionType("ALLOW")
///                     .condition("resource.management.autoUpgrade == false")
///                     .description("Only allow GKE NodePool resource to be created or updated if AutoUpgrade is not enabled where this custom constraint is enforced.")
///                     .displayName("Disable GKE auto upgrade").methodTypes(vec!["CREATE",
///                     "UPDATE",])
///                     .name("organizations/123456789/customConstraints/custom.disableGkeAutoUpgrade")
///                     .resourceTypes(vec!["container.googleapis.com/NodePool",])
///                     .build_struct())
///                     .policyRules(vec![PosturePolicySetPolicyConstraintOrgPolicyConstraintCustomPolicyRule::builder()
///                     .condition(PosturePolicySetPolicyConstraintOrgPolicyConstraintCustomPolicyRuleCondition::builder()
///                     .description("condition description")
///                     .expression("resource.matchTagId('tagKeys/key_id','tagValues/value_id')")
///                     .title("a CEL condition").build_struct()).enforce(true)
///                     .build_struct(),]).build_struct()).build_struct())
///                     .policyId("custom_org_policy").build_struct(),])
///                     .policySetId("org_policy_set").build_struct(),
///                     PosturePolicySet::builder().description("set of sha policies")
///                     .policies(vec![PosturePolicySetPolicy::builder()
///                     .constraint(PosturePolicySetPolicyConstraint::builder()
///                     .securityHealthAnalyticsModule(PosturePolicySetPolicyConstraintSecurityHealthAnalyticsModule::builder()
///                     .moduleEnablementState("ENABLED")
///                     .moduleName("BIGQUERY_TABLE_CMEK_DISABLED").build_struct())
///                     .build_struct()).description("enable BIGQUERY_TABLE_CMEK_DISABLED")
///                     .policyId("sha_builtin_module").build_struct(),
///                     PosturePolicySetPolicy::builder()
///                     .constraint(PosturePolicySetPolicyConstraint::builder()
///                     .securityHealthAnalyticsCustomModule(PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModule::builder()
///                     .config(PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModuleConfig::builder()
///                     .customOutput(PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModuleConfigCustomOutput::builder()
///                     .properties(vec![PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModuleConfigCustomOutputProperty::builder()
///                     .name("duration")
///                     .valueExpression(PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModuleConfigCustomOutputPropertyValueExpression::builder()
///                     .expression("resource.rotationPeriod").build_struct())
///                     .build_struct(),]).build_struct()).description("Custom Module")
///                     .predicate(PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModuleConfigPredicate::builder()
///                     .expression("resource.rotationPeriod > duration('2592000s')")
///                     .build_struct()).recommendation("Testing custom modules")
///                     .resourceSelector(PosturePolicySetPolicyConstraintSecurityHealthAnalyticsCustomModuleConfigResourceSelector::builder()
///                     .resourceTypes(vec!["cloudkms.googleapis.com/CryptoKey",])
///                     .build_struct()).severity("LOW").build_struct())
///                     .displayName("custom_SHA_policy").moduleEnablementState("ENABLED")
///                     .build_struct()).build_struct()).policyId("sha_custom_module")
///                     .build_struct(),]).policySetId("sha_policy_set").build_struct(),
///                 ],
///             )
///             .posture_id("posture_example")
///             .state("ACTIVE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Posture can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/postures/{{posture_id}}`
///
/// When using the `pulumi import` command, Posture can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securityposture/posture:Posture default {{parent}}/locations/{{location}}/postures/{{posture_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod posture {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PostureArgs {
        /// Description of the posture.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Location of the resource, eg: global.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The parent of the resource, an organization. Format should be `organizations/{organization_id}`.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of policy sets for the posture.
        /// Structure is documented below.
        #[builder(into)]
        pub policy_sets: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::securityposture::PosturePolicySet>,
        >,
        /// Id of the posture. It is an immutable field.
        #[builder(into)]
        pub posture_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// State of the posture. Update to state field should not be triggered along with
        /// with other field updates.
        /// Possible values are: `DEPRECATED`, `DRAFT`, `ACTIVE`.
        #[builder(into)]
        pub state: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PostureResult {
        /// Time the Posture was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the posture.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// For Resource freshness validation (https://google.aip.dev/154)
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Location of the resource, eg: global.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the posture.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent of the resource, an organization. Format should be `organizations/{organization_id}`.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// List of policy sets for the posture.
        /// Structure is documented below.
        pub policy_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::securityposture::PosturePolicySet>,
        >,
        /// Id of the posture. It is an immutable field.
        pub posture_id: pulumi_gestalt_rust::Output<String>,
        /// If set, there are currently changes in flight to the posture.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Revision_id of the posture.
        pub revision_id: pulumi_gestalt_rust::Output<String>,
        /// State of the posture. Update to state field should not be triggered along with
        /// with other field updates.
        /// Possible values are: `DEPRECATED`, `DRAFT`, `ACTIVE`.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Time the Posture was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PostureArgs,
    ) -> PostureResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let parent_binding_1 = args.parent.get_output(context);
        let parent_binding = parent_binding_1.get_inner();
        let policy_sets_binding_1 = args.policy_sets.get_output(context);
        let policy_sets_binding = policy_sets_binding_1.get_inner();
        let posture_id_binding_1 = args.posture_id.get_output(context);
        let posture_id_binding = posture_id_binding_1.get_inner();
        let state_binding_1 = args.state.get_output(context);
        let state_binding = state_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securityposture/posture:Posture".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "policySets".into(),
                    value: &policy_sets_binding,
                },
                register_interface::ObjectField {
                    name: "postureId".into(),
                    value: &posture_id_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PostureResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            policy_sets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policySets"),
            ),
            posture_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("postureId"),
            ),
            reconciling: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            revision_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revisionId"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
