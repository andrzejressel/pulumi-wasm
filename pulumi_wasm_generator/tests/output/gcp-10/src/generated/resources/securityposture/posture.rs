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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod posture {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PostureArgs {
        /// Description of the posture.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Location of the resource, eg: global.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The parent of the resource, an organization. Format should be `organizations/{organization_id}`.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
        /// List of policy sets for the posture.
        /// Structure is documented below.
        #[builder(into)]
        pub policy_sets: pulumi_wasm_rust::Output<
            Vec<super::super::types::securityposture::PosturePolicySet>,
        >,
        /// Id of the posture. It is an immutable field.
        #[builder(into)]
        pub posture_id: pulumi_wasm_rust::Output<String>,
        /// State of the posture. Update to state field should not be triggered along with
        /// with other field updates.
        /// Possible values are: `DEPRECATED`, `DRAFT`, `ACTIVE`.
        #[builder(into)]
        pub state: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PostureResult {
        /// Time the Posture was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Description of the posture.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// For Resource freshness validation (https://google.aip.dev/154)
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Location of the resource, eg: global.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the posture.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent of the resource, an organization. Format should be `organizations/{organization_id}`.
        pub parent: pulumi_wasm_rust::Output<String>,
        /// List of policy sets for the posture.
        /// Structure is documented below.
        pub policy_sets: pulumi_wasm_rust::Output<
            Vec<super::super::types::securityposture::PosturePolicySet>,
        >,
        /// Id of the posture. It is an immutable field.
        pub posture_id: pulumi_wasm_rust::Output<String>,
        /// If set, there are currently changes in flight to the posture.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// Revision_id of the posture.
        pub revision_id: pulumi_wasm_rust::Output<String>,
        /// State of the posture. Update to state field should not be triggered along with
        /// with other field updates.
        /// Possible values are: `DEPRECATED`, `DRAFT`, `ACTIVE`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Time the Posture was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PostureArgs) -> PostureResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let parent_binding = args.parent.get_inner();
        let policy_sets_binding = args.policy_sets.get_inner();
        let posture_id_binding = args.posture_id.get_inner();
        let state_binding = args.state.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securityposture/posture:Posture".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "policySets".into(),
                },
                register_interface::ResultField {
                    name: "postureId".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "revisionId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
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
        PostureResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            policy_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policySets").unwrap(),
            ),
            posture_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postureId").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            revision_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revisionId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
