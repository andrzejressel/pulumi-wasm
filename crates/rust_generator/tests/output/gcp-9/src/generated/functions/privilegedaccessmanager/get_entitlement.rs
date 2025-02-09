#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_entitlement {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEntitlementArgs {
        /// ID of the Entitlement resource. This is the last part of the Entitlement's full name which is of the format `{parent}/locations/{location}/entitlements/{entitlement_id}`.
        #[builder(into, default)]
        pub entitlement_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the Entitlement resource.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project or folder or organization that contains the resource. Format: projects/{project-id|project-number} or folders/{folder-number}  or organizations/{organization-number}
        #[builder(into, default)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEntitlementResult {
        pub additional_notification_targets: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::privilegedaccessmanager::GetEntitlementAdditionalNotificationTarget,
            >,
        >,
        pub approval_workflows: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::privilegedaccessmanager::GetEntitlementApprovalWorkflow,
            >,
        >,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub eligible_users: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::privilegedaccessmanager::GetEntitlementEligibleUser,
            >,
        >,
        pub entitlement_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        pub max_request_duration: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub parent: pulumi_gestalt_rust::Output<Option<String>>,
        pub privileged_accesses: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::privilegedaccessmanager::GetEntitlementPrivilegedAccess,
            >,
        >,
        pub requester_justification_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::privilegedaccessmanager::GetEntitlementRequesterJustificationConfig,
            >,
        >,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEntitlementArgs,
    ) -> GetEntitlementResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let entitlement_id_binding = args.entitlement_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:privilegedaccessmanager/getEntitlement:getEntitlement".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entitlementId".into(),
                    value: entitlement_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEntitlementResult {
            additional_notification_targets: o
                .get_field("additionalNotificationTargets"),
            approval_workflows: o.get_field("approvalWorkflows"),
            create_time: o.get_field("createTime"),
            eligible_users: o.get_field("eligibleUsers"),
            entitlement_id: o.get_field("entitlementId"),
            etag: o.get_field("etag"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            max_request_duration: o.get_field("maxRequestDuration"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            privileged_accesses: o.get_field("privilegedAccesses"),
            requester_justification_configs: o
                .get_field("requesterJustificationConfigs"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
        }
    }
}
