/// An Entitlement defines the eligibility of a set of users to obtain a predefined access for some time possibly after going through an approval workflow.
///
///
///
/// ## Example Usage
///
/// ### Privileged Access Manager Entitlement Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let tfentitlement = entitlement::create(
///         "tfentitlement",
///         EntitlementArgs::builder()
///             .additional_notification_targets(
///                 EntitlementAdditionalNotificationTargets::builder()
///                     .adminEmailRecipients(vec!["user@example.com",])
///                     .requesterEmailRecipients(vec!["user@example.com",])
///                     .build_struct(),
///             )
///             .approval_workflow(
///                 EntitlementApprovalWorkflow::builder()
///                     .manualApprovals(
///                         EntitlementApprovalWorkflowManualApprovals::builder()
///                             .requireApproverJustification(true)
///                             .steps(
///                                 vec![
///                                     EntitlementApprovalWorkflowManualApprovalsStep::builder()
///                                     .approvalsNeeded(1)
///                                     .approverEmailRecipients(vec!["user@example.com",])
///                                     .approvers(EntitlementApprovalWorkflowManualApprovalsStepApprovers::builder()
///                                     .principals(vec!["group:test@google.com",]).build_struct())
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .eligible_users(
///                 vec![
///                     EntitlementEligibleUser::builder()
///                     .principals(vec!["group:test@google.com",]).build_struct(),
///                 ],
///             )
///             .entitlement_id("example-entitlement")
///             .location("global")
///             .max_request_duration("43200s")
///             .parent("projects/my-project-name")
///             .privileged_access(
///                 EntitlementPrivilegedAccess::builder()
///                     .gcpIamAccess(
///                         EntitlementPrivilegedAccessGcpIamAccess::builder()
///                             .resource(
///                                 "//cloudresourcemanager.googleapis.com/projects/my-project-name",
///                             )
///                             .resourceType("cloudresourcemanager.googleapis.com/Project")
///                             .roleBindings(
///                                 vec![
///                                     EntitlementPrivilegedAccessGcpIamAccessRoleBinding::builder()
///                                     .conditionExpression("request.time < timestamp(\"2024-04-23T18:30:00.000Z\")")
///                                     .role("roles/storage.admin").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .requester_justification_config(
///                 EntitlementRequesterJustificationConfig::builder()
///                     .unstructured(
///                         EntitlementRequesterJustificationConfigUnstructured::builder()
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Entitlement can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/entitlements/{{entitlement_id}}`
///
/// When using the `pulumi import` command, Entitlement can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:privilegedaccessmanager/entitlement:entitlement default {{parent}}/locations/{{location}}/entitlements/{{entitlement_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod entitlement {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct entitlementArgs {
        /// AdditionalNotificationTargets includes email addresses to be notified.
        #[builder(into, default)]
        pub additional_notification_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::privilegedaccessmanager::EntitlementAdditionalNotificationTargets,
            >,
        >,
        /// The approvals needed before access will be granted to a requester. No approvals will be needed if this field is null.
        /// Different types of approval workflows that can be used to gate privileged access granting.
        #[builder(into, default)]
        pub approval_workflow: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::privilegedaccessmanager::EntitlementApprovalWorkflow,
            >,
        >,
        /// Who can create Grants using Entitlement. This list should contain at most one entry
        /// Structure is documented below.
        #[builder(into)]
        pub eligible_users: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::privilegedaccessmanager::EntitlementEligibleUser>,
        >,
        /// The ID to use for this Entitlement. This will become the last part of the resource name.
        /// This value should be 4-63 characters, and valid characters are "[a-z]", "[0-9]", and "-". The first character should be from [a-z].
        /// This value should be unique among all other Entitlements under the specified `parent`.
        #[builder(into)]
        pub entitlement_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The region of the Entitlement resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The maximum amount of time for which access would be granted for a request.
        /// A requester can choose to ask for access for less than this duration but never more.
        /// Format: calculate the time in seconds and concatenate it with 's' i.e. 2 hours = "7200s", 45 minutes = "2700s"
        #[builder(into)]
        pub max_request_duration: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Format: projects/{project-id|project-number} or organizations/{organization-number} or folders/{folder-number}
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Privileged access that this service can be used to gate.
        /// Structure is documented below.
        #[builder(into)]
        pub privileged_access: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::privilegedaccessmanager::EntitlementPrivilegedAccess,
        >,
        /// Defines the ways in which a requester should provide the justification while requesting for access.
        /// Structure is documented below.
        #[builder(into)]
        pub requester_justification_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::privilegedaccessmanager::EntitlementRequesterJustificationConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct entitlementResult {
        /// AdditionalNotificationTargets includes email addresses to be notified.
        pub additional_notification_targets: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::privilegedaccessmanager::EntitlementAdditionalNotificationTargets,
            >,
        >,
        /// The approvals needed before access will be granted to a requester. No approvals will be needed if this field is null.
        /// Different types of approval workflows that can be used to gate privileged access granting.
        pub approval_workflow: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::privilegedaccessmanager::EntitlementApprovalWorkflow,
            >,
        >,
        /// Output only. Create time stamp. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z"
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Who can create Grants using Entitlement. This list should contain at most one entry
        /// Structure is documented below.
        pub eligible_users: pulumi_gestalt_rust::Output<
            Vec<super::super::types::privilegedaccessmanager::EntitlementEligibleUser>,
        >,
        /// The ID to use for this Entitlement. This will become the last part of the resource name.
        /// This value should be 4-63 characters, and valid characters are "[a-z]", "[0-9]", and "-". The first character should be from [a-z].
        /// This value should be unique among all other Entitlements under the specified `parent`.
        pub entitlement_id: pulumi_gestalt_rust::Output<String>,
        /// For Resource freshness validation (https://google.aip.dev/154)
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The region of the Entitlement resource.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum amount of time for which access would be granted for a request.
        /// A requester can choose to ask for access for less than this duration but never more.
        /// Format: calculate the time in seconds and concatenate it with 's' i.e. 2 hours = "7200s", 45 minutes = "2700s"
        pub max_request_duration: pulumi_gestalt_rust::Output<String>,
        /// Output Only. The entitlement's name follows a hierarchical structure, comprising the organization, folder, or project, alongside the region and a unique entitlement ID.
        /// Formats: organizations/{organization-number}/locations/{region}/entitlements/{entitlement-id}, folders/{folder-number}/locations/{region}/entitlements/{entitlement-id}, and projects/{project-id|project-number}/locations/{region}/entitlements/{entitlement-id}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Format: projects/{project-id|project-number} or organizations/{organization-number} or folders/{folder-number}
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Privileged access that this service can be used to gate.
        /// Structure is documented below.
        pub privileged_access: pulumi_gestalt_rust::Output<
            super::super::types::privilegedaccessmanager::EntitlementPrivilegedAccess,
        >,
        /// Defines the ways in which a requester should provide the justification while requesting for access.
        /// Structure is documented below.
        pub requester_justification_config: pulumi_gestalt_rust::Output<
            super::super::types::privilegedaccessmanager::EntitlementRequesterJustificationConfig,
        >,
        /// Output only. The current state of the Entitlement.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. Update time stamp. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: entitlementArgs,
    ) -> entitlementResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let additional_notification_targets_binding = args
            .additional_notification_targets
            .get_output(context)
            .get_inner();
        let approval_workflow_binding = args
            .approval_workflow
            .get_output(context)
            .get_inner();
        let eligible_users_binding = args.eligible_users.get_output(context).get_inner();
        let entitlement_id_binding = args.entitlement_id.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let max_request_duration_binding = args
            .max_request_duration
            .get_output(context)
            .get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let privileged_access_binding = args
            .privileged_access
            .get_output(context)
            .get_inner();
        let requester_justification_config_binding = args
            .requester_justification_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:privilegedaccessmanager/entitlement:entitlement".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalNotificationTargets".into(),
                    value: &additional_notification_targets_binding,
                },
                register_interface::ObjectField {
                    name: "approvalWorkflow".into(),
                    value: &approval_workflow_binding,
                },
                register_interface::ObjectField {
                    name: "eligibleUsers".into(),
                    value: &eligible_users_binding,
                },
                register_interface::ObjectField {
                    name: "entitlementId".into(),
                    value: &entitlement_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maxRequestDuration".into(),
                    value: &max_request_duration_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "privilegedAccess".into(),
                    value: &privileged_access_binding,
                },
                register_interface::ObjectField {
                    name: "requesterJustificationConfig".into(),
                    value: &requester_justification_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        entitlementResult {
            additional_notification_targets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalNotificationTargets"),
            ),
            approval_workflow: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("approvalWorkflow"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            eligible_users: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eligibleUsers"),
            ),
            entitlement_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("entitlementId"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            max_request_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxRequestDuration"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            privileged_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privilegedAccess"),
            ),
            requester_justification_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requesterJustificationConfig"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
