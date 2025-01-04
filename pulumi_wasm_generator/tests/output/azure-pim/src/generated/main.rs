pub mod pim {
    include!("resources/pim/active_role_assignment.rs");
    include!("resources/pim/eligible_role_assignment.rs");
    include!("resources/pim/role_management_policy.rs");
}
pub mod functions {
    pub mod pim {
        include!("functions/pim/get_role_management_policy.rs");
    }
}
pub mod types {
    pub mod pim {
        include!("types/pim/active_role_assignment_schedule.rs");
        include!("types/pim/active_role_assignment_schedule_expiration.rs");
        include!("types/pim/active_role_assignment_ticket.rs");
        include!("types/pim/eligible_role_assignment_schedule.rs");
        include!("types/pim/eligible_role_assignment_schedule_expiration.rs");
        include!("types/pim/eligible_role_assignment_ticket.rs");
        include!("types/pim/role_management_policy_activation_rules.rs");
        include!("types/pim/role_management_policy_activation_rules_approval_stage.rs");
        include!(
            "types/pim/role_management_policy_activation_rules_approval_stage_primary_approver.rs"
        );
        include!("types/pim/role_management_policy_active_assignment_rules.rs");
        include!("types/pim/role_management_policy_eligible_assignment_rules.rs");
        include!("types/pim/role_management_policy_notification_rules.rs");
        include!(
            "types/pim/role_management_policy_notification_rules_active_assignments.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_active_assignments_admin_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_active_assignments_approver_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_active_assignments_assignee_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_activations.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_activations_admin_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_activations_approver_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_activations_assignee_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_assignments.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_assignments_admin_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_assignments_approver_notifications.rs"
        );
        include!(
            "types/pim/role_management_policy_notification_rules_eligible_assignments_assignee_notifications.rs"
        );
        include!("types/pim/get_role_management_policy_activation_rule.rs");
        include!(
            "types/pim/get_role_management_policy_activation_rule_approval_stage.rs"
        );
        include!(
            "types/pim/get_role_management_policy_activation_rule_approval_stage_primary_approver.rs"
        );
        include!("types/pim/get_role_management_policy_active_assignment_rule.rs");
        include!("types/pim/get_role_management_policy_eligible_assignment_rule.rs");
        include!("types/pim/get_role_management_policy_notification_rule.rs");
        include!(
            "types/pim/get_role_management_policy_notification_rule_active_assignment.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_active_assignment_admin_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_active_assignment_approver_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_active_assignment_assignee_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_activation.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_activation_admin_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_activation_approver_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_activation_assignee_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_assignment.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_assignment_admin_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_assignment_approver_notification.rs"
        );
        include!(
            "types/pim/get_role_management_policy_notification_rule_eligible_assignment_assignee_notification.rs"
        );
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-azure {
    import output-interface;
}

interface output-interface {

    resource output {
        constructor(value: string);
        map: func(function-name: string) -> output;
    }
    combine: func(outputs: list<borrow<output>>) -> output;
}


interface register-interface {
    use output-interface.{output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record result-field {
        name: string
    }

    record register-resource-result-field {
        name: string,
        output: output
    }

    record register-resource-request {
        %type: string,
        name: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record register-resource-result {
        fields: list<register-resource-result-field>
    }

    register: func(request: register-resource-request) -> register-resource-result;

    record resource-invoke-result-field {
        name: string,
        output: output
    }

    record resource-invoke-request {
        token: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record resource-invoke-result {
        fields: list<resource-invoke-result-field>
    }

    invoke: func(request: resource-invoke-request) -> resource-invoke-result;
}",
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        } }
    );
}
