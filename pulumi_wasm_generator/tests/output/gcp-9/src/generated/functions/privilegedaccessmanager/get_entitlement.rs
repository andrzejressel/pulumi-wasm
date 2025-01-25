pub mod get_entitlement {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEntitlementArgs {
        /// ID of the Entitlement resource. This is the last part of the Entitlement's full name which is of the format `{parent}/locations/{location}/entitlements/{entitlement_id}`.
        #[builder(into, default)]
        pub entitlement_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region of the Entitlement resource.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The project or folder or organization that contains the resource. Format: projects/{project-id|project-number} or folders/{folder-number}  or organizations/{organization-number}
        #[builder(into, default)]
        pub parent: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEntitlementResult {
        pub additional_notification_targets: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::privilegedaccessmanager::GetEntitlementAdditionalNotificationTarget,
            >,
        >,
        pub approval_workflows: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::privilegedaccessmanager::GetEntitlementApprovalWorkflow,
            >,
        >,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub eligible_users: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::privilegedaccessmanager::GetEntitlementEligibleUser,
            >,
        >,
        pub entitlement_id: pulumi_wasm_rust::Output<Option<String>>,
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        pub max_request_duration: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub parent: pulumi_wasm_rust::Output<Option<String>>,
        pub privileged_accesses: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::privilegedaccessmanager::GetEntitlementPrivilegedAccess,
            >,
        >,
        pub requester_justification_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::privilegedaccessmanager::GetEntitlementRequesterJustificationConfig,
            >,
        >,
        pub state: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEntitlementArgs,
    ) -> GetEntitlementResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let entitlement_id_binding = args.entitlement_id.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:privilegedaccessmanager/getEntitlement:getEntitlement".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "entitlementId".into(),
                    value: &entitlement_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalNotificationTargets".into(),
                },
                register_interface::ResultField {
                    name: "approvalWorkflows".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "eligibleUsers".into(),
                },
                register_interface::ResultField {
                    name: "entitlementId".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maxRequestDuration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "privilegedAccesses".into(),
                },
                register_interface::ResultField {
                    name: "requesterJustificationConfigs".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEntitlementResult {
            additional_notification_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalNotificationTargets").unwrap(),
            ),
            approval_workflows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvalWorkflows").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            eligible_users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eligibleUsers").unwrap(),
            ),
            entitlement_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entitlementId").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            max_request_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxRequestDuration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            privileged_accesses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privilegedAccesses").unwrap(),
            ),
            requester_justification_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requesterJustificationConfigs").unwrap(),
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
