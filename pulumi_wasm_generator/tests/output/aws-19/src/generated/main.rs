pub mod sesv2 {
    include!("resources/sesv2/account_suppression_attributes.rs");
    include!("resources/sesv2/account_vdm_attributes.rs");
    include!("resources/sesv2/configuration_set.rs");
    include!("resources/sesv2/configuration_set_event_destination.rs");
    include!("resources/sesv2/contact_list.rs");
    include!("resources/sesv2/dedicated_ip_assignment.rs");
    include!("resources/sesv2/dedicated_ip_pool.rs");
    include!("resources/sesv2/email_identity.rs");
    include!("resources/sesv2/email_identity_feedback_attributes.rs");
    include!("resources/sesv2/email_identity_mail_from_attributes.rs");
    include!("resources/sesv2/email_identity_policy.rs");
}
pub mod sfn {
    include!("resources/sfn/activity.rs");
    include!("resources/sfn/alias.rs");
    include!("resources/sfn/state_machine.rs");
}
pub mod shield {
    include!("resources/shield/application_layer_automatic_response.rs");
    include!("resources/shield/drt_access_log_bucket_association.rs");
    include!("resources/shield/drt_access_role_arn_association.rs");
    include!("resources/shield/proactive_engagement.rs");
    include!("resources/shield/protection.rs");
    include!("resources/shield/protection_group.rs");
    include!("resources/shield/protection_health_check_association.rs");
    include!("resources/shield/subscription.rs");
}
pub mod signer {
    include!("resources/signer/signing_job.rs");
    include!("resources/signer/signing_profile.rs");
    include!("resources/signer/signing_profile_permission.rs");
}
pub mod simpledb {
    include!("resources/simpledb/domain.rs");
}
pub mod sns {
    include!("resources/sns/data_protection_policy.rs");
    include!("resources/sns/platform_application.rs");
    include!("resources/sns/sms_preferences.rs");
    include!("resources/sns/topic.rs");
    include!("resources/sns/topic_policy.rs");
    include!("resources/sns/topic_subscription.rs");
}
pub mod sqs {
    include!("resources/sqs/queue.rs");
    include!("resources/sqs/queue_policy.rs");
    include!("resources/sqs/redrive_allow_policy.rs");
    include!("resources/sqs/redrive_policy.rs");
}
pub mod ssm {
    include!("resources/ssm/activation.rs");
    include!("resources/ssm/association.rs");
    include!("resources/ssm/contacts_rotation.rs");
    include!("resources/ssm/default_patch_baseline.rs");
    include!("resources/ssm/document.rs");
    include!("resources/ssm/maintenance_window.rs");
    include!("resources/ssm/maintenance_window_target.rs");
    include!("resources/ssm/maintenance_window_task.rs");
    include!("resources/ssm/parameter.rs");
    include!("resources/ssm/patch_baseline.rs");
    include!("resources/ssm/patch_group.rs");
    include!("resources/ssm/quicksetup_configuration_manager.rs");
    include!("resources/ssm/resource_data_sync.rs");
    include!("resources/ssm/service_setting.rs");
}
pub mod ssmcontacts {
    include!("resources/ssmcontacts/contact.rs");
    include!("resources/ssmcontacts/contact_channel.rs");
    include!("resources/ssmcontacts/plan.rs");
}
pub mod ssmincidents {
    include!("resources/ssmincidents/replication_set.rs");
    include!("resources/ssmincidents/response_plan.rs");
}
pub mod functions {
    pub mod sesv2 {
        include!("functions/sesv2/get_configuration_set.rs");
        include!("functions/sesv2/get_dedicated_ip_pool.rs");
        include!("functions/sesv2/get_email_identity.rs");
        include!("functions/sesv2/get_email_identity_mail_from_attributes.rs");
    }
    pub mod sfn {
        include!("functions/sfn/get_activity.rs");
        include!("functions/sfn/get_alias.rs");
        include!("functions/sfn/get_state_machine.rs");
        include!("functions/sfn/get_state_machine_versions.rs");
    }
    pub mod shield {
        include!("functions/shield/get_protection.rs");
    }
    pub mod signer {
        include!("functions/signer/get_signing_job.rs");
        include!("functions/signer/get_signing_profile.rs");
    }
    pub mod sns {
        include!("functions/sns/get_topic.rs");
    }
    pub mod sqs {
        include!("functions/sqs/get_queue.rs");
        include!("functions/sqs/get_queues.rs");
    }
    pub mod ssm {
        include!("functions/ssm/get_contacts_rotation.rs");
        include!("functions/ssm/get_document.rs");
        include!("functions/ssm/get_instances.rs");
        include!("functions/ssm/get_maintenance_windows.rs");
        include!("functions/ssm/get_parameter.rs");
        include!("functions/ssm/get_parameters_by_path.rs");
        include!("functions/ssm/get_patch_baseline.rs");
        include!("functions/ssm/get_patch_baselines.rs");
    }
    pub mod ssmcontacts {
        include!("functions/ssmcontacts/get_contact.rs");
        include!("functions/ssmcontacts/get_contact_channel.rs");
        include!("functions/ssmcontacts/get_plan.rs");
    }
    pub mod ssmincidents {
        include!("functions/ssmincidents/get_replication_set.rs");
        include!("functions/ssmincidents/get_response_plan.rs");
    }
    include!("functions/get_arn.rs");
    include!("functions/get_availability_zone.rs");
    include!("functions/get_availability_zones.rs");
    include!("functions/get_billing_service_account.rs");
    include!("functions/get_caller_identity.rs");
    include!("functions/get_default_tags.rs");
    include!("functions/get_ip_ranges.rs");
    include!("functions/get_partition.rs");
    include!("functions/get_region.rs");
    include!("functions/get_regions.rs");
    include!("functions/get_service.rs");
    include!("functions/get_service_principal.rs");
}
pub mod types {
    pub mod sesv2 {
        include!("types/sesv2/account_vdm_attributes_dashboard_attributes.rs");
        include!("types/sesv2/account_vdm_attributes_guardian_attributes.rs");
        include!("types/sesv2/configuration_set_delivery_options.rs");
        include!("types/sesv2/configuration_set_event_destination_event_destination.rs");
        include!(
            "types/sesv2/configuration_set_event_destination_event_destination_cloud_watch_destination.rs"
        );
        include!(
            "types/sesv2/configuration_set_event_destination_event_destination_cloud_watch_destination_dimension_configuration.rs"
        );
        include!(
            "types/sesv2/configuration_set_event_destination_event_destination_event_bridge_destination.rs"
        );
        include!(
            "types/sesv2/configuration_set_event_destination_event_destination_kinesis_firehose_destination.rs"
        );
        include!(
            "types/sesv2/configuration_set_event_destination_event_destination_pinpoint_destination.rs"
        );
        include!(
            "types/sesv2/configuration_set_event_destination_event_destination_sns_destination.rs"
        );
        include!("types/sesv2/configuration_set_reputation_options.rs");
        include!("types/sesv2/configuration_set_sending_options.rs");
        include!("types/sesv2/configuration_set_suppression_options.rs");
        include!("types/sesv2/configuration_set_tracking_options.rs");
        include!("types/sesv2/configuration_set_vdm_options.rs");
        include!("types/sesv2/configuration_set_vdm_options_dashboard_options.rs");
        include!("types/sesv2/configuration_set_vdm_options_guardian_options.rs");
        include!("types/sesv2/contact_list_topic.rs");
        include!("types/sesv2/email_identity_dkim_signing_attributes.rs");
        include!("types/sesv2/get_configuration_set_delivery_option.rs");
        include!("types/sesv2/get_configuration_set_reputation_option.rs");
        include!("types/sesv2/get_configuration_set_sending_option.rs");
        include!("types/sesv2/get_configuration_set_suppression_option.rs");
        include!("types/sesv2/get_configuration_set_tracking_option.rs");
        include!("types/sesv2/get_configuration_set_vdm_option.rs");
        include!("types/sesv2/get_configuration_set_vdm_option_dashboard_option.rs");
        include!("types/sesv2/get_configuration_set_vdm_option_guardian_option.rs");
        include!("types/sesv2/get_dedicated_ip_pool_dedicated_ip.rs");
        include!("types/sesv2/get_email_identity_dkim_signing_attribute.rs");
    }
    pub mod sfn {
        include!("types/sfn/activity_encryption_configuration.rs");
        include!("types/sfn/alias_routing_configuration.rs");
        include!("types/sfn/state_machine_encryption_configuration.rs");
        include!("types/sfn/state_machine_logging_configuration.rs");
        include!("types/sfn/state_machine_tracing_configuration.rs");
        include!("types/sfn/get_alias_routing_configuration.rs");
    }
    pub mod shield {
        include!("types/shield/application_layer_automatic_response_timeouts.rs");
        include!("types/shield/drt_access_log_bucket_association_timeouts.rs");
        include!("types/shield/drt_access_role_arn_association_timeouts.rs");
        include!("types/shield/proactive_engagement_emergency_contact.rs");
    }
    pub mod signer {
        include!("types/signer/signing_job_destination.rs");
        include!("types/signer/signing_job_destination_s_3.rs");
        include!("types/signer/signing_job_revocation_record.rs");
        include!("types/signer/signing_job_signed_object.rs");
        include!("types/signer/signing_job_signed_object_s_3.rs");
        include!("types/signer/signing_job_source.rs");
        include!("types/signer/signing_job_source_s_3.rs");
        include!("types/signer/signing_profile_revocation_record.rs");
        include!("types/signer/signing_profile_signature_validity_period.rs");
        include!("types/signer/signing_profile_signing_material.rs");
        include!("types/signer/get_signing_job_revocation_record.rs");
        include!("types/signer/get_signing_job_signed_object.rs");
        include!("types/signer/get_signing_job_signed_object_s_3.rs");
        include!("types/signer/get_signing_job_source.rs");
        include!("types/signer/get_signing_job_source_s_3.rs");
        include!("types/signer/get_signing_profile_revocation_record.rs");
        include!("types/signer/get_signing_profile_signature_validity_period.rs");
    }
    pub mod ssm {
        include!("types/ssm/association_output_location.rs");
        include!("types/ssm/association_target.rs");
        include!("types/ssm/contacts_rotation_recurrence.rs");
        include!("types/ssm/contacts_rotation_recurrence_daily_setting.rs");
        include!("types/ssm/contacts_rotation_recurrence_monthly_setting.rs");
        include!(
            "types/ssm/contacts_rotation_recurrence_monthly_setting_hand_off_time.rs"
        );
        include!("types/ssm/contacts_rotation_recurrence_shift_coverage.rs");
        include!(
            "types/ssm/contacts_rotation_recurrence_shift_coverage_coverage_time.rs"
        );
        include!(
            "types/ssm/contacts_rotation_recurrence_shift_coverage_coverage_time_end.rs"
        );
        include!(
            "types/ssm/contacts_rotation_recurrence_shift_coverage_coverage_time_start.rs"
        );
        include!("types/ssm/contacts_rotation_recurrence_weekly_setting.rs");
        include!(
            "types/ssm/contacts_rotation_recurrence_weekly_setting_hand_off_time.rs"
        );
        include!("types/ssm/document_attachments_source.rs");
        include!("types/ssm/document_parameter.rs");
        include!("types/ssm/maintenance_window_target_target.rs");
        include!("types/ssm/maintenance_window_task_target.rs");
        include!("types/ssm/maintenance_window_task_task_invocation_parameters.rs");
        include!(
            "types/ssm/maintenance_window_task_task_invocation_parameters_automation_parameters.rs"
        );
        include!(
            "types/ssm/maintenance_window_task_task_invocation_parameters_automation_parameters_parameter.rs"
        );
        include!(
            "types/ssm/maintenance_window_task_task_invocation_parameters_lambda_parameters.rs"
        );
        include!(
            "types/ssm/maintenance_window_task_task_invocation_parameters_run_command_parameters.rs"
        );
        include!(
            "types/ssm/maintenance_window_task_task_invocation_parameters_run_command_parameters_cloudwatch_config.rs"
        );
        include!(
            "types/ssm/maintenance_window_task_task_invocation_parameters_run_command_parameters_notification_config.rs"
        );
        include!(
            "types/ssm/maintenance_window_task_task_invocation_parameters_run_command_parameters_parameter.rs"
        );
        include!(
            "types/ssm/maintenance_window_task_task_invocation_parameters_step_functions_parameters.rs"
        );
        include!("types/ssm/patch_baseline_approval_rule.rs");
        include!("types/ssm/patch_baseline_approval_rule_patch_filter.rs");
        include!("types/ssm/patch_baseline_global_filter.rs");
        include!("types/ssm/patch_baseline_source.rs");
        include!(
            "types/ssm/quicksetup_configuration_manager_configuration_definition.rs"
        );
        include!("types/ssm/quicksetup_configuration_manager_status_summary.rs");
        include!("types/ssm/quicksetup_configuration_manager_timeouts.rs");
        include!("types/ssm/resource_data_sync_s_3_destination.rs");
        include!("types/ssm/get_contacts_rotation_recurrence.rs");
        include!("types/ssm/get_contacts_rotation_recurrence_daily_setting.rs");
        include!("types/ssm/get_contacts_rotation_recurrence_monthly_setting.rs");
        include!(
            "types/ssm/get_contacts_rotation_recurrence_monthly_setting_hand_off_time.rs"
        );
        include!("types/ssm/get_contacts_rotation_recurrence_shift_coverage.rs");
        include!(
            "types/ssm/get_contacts_rotation_recurrence_shift_coverage_coverage_time.rs"
        );
        include!(
            "types/ssm/get_contacts_rotation_recurrence_shift_coverage_coverage_time_end.rs"
        );
        include!(
            "types/ssm/get_contacts_rotation_recurrence_shift_coverage_coverage_time_start.rs"
        );
        include!("types/ssm/get_contacts_rotation_recurrence_weekly_setting.rs");
        include!(
            "types/ssm/get_contacts_rotation_recurrence_weekly_setting_hand_off_time.rs"
        );
        include!("types/ssm/get_instances_filter.rs");
        include!("types/ssm/get_maintenance_windows_filter.rs");
        include!("types/ssm/get_patch_baseline_approval_rule.rs");
        include!("types/ssm/get_patch_baseline_approval_rule_patch_filter.rs");
        include!("types/ssm/get_patch_baseline_global_filter.rs");
        include!("types/ssm/get_patch_baseline_source.rs");
        include!("types/ssm/get_patch_baselines_baseline_identity.rs");
        include!("types/ssm/get_patch_baselines_filter.rs");
    }
    pub mod ssmcontacts {
        include!("types/ssmcontacts/contact_channel_delivery_address.rs");
        include!("types/ssmcontacts/plan_stage.rs");
        include!("types/ssmcontacts/plan_stage_target.rs");
        include!("types/ssmcontacts/plan_stage_target_channel_target_info.rs");
        include!("types/ssmcontacts/plan_stage_target_contact_target_info.rs");
        include!("types/ssmcontacts/get_contact_channel_delivery_address.rs");
        include!("types/ssmcontacts/get_plan_stage.rs");
        include!("types/ssmcontacts/get_plan_stage_target.rs");
        include!("types/ssmcontacts/get_plan_stage_target_channel_target_info.rs");
        include!("types/ssmcontacts/get_plan_stage_target_contact_target_info.rs");
    }
    pub mod ssmincidents {
        include!("types/ssmincidents/replication_set_region.rs");
        include!("types/ssmincidents/response_plan_action.rs");
        include!("types/ssmincidents/response_plan_action_ssm_automation.rs");
        include!("types/ssmincidents/response_plan_action_ssm_automation_parameter.rs");
        include!("types/ssmincidents/response_plan_incident_template.rs");
        include!(
            "types/ssmincidents/response_plan_incident_template_notification_target.rs"
        );
        include!("types/ssmincidents/response_plan_integration.rs");
        include!("types/ssmincidents/response_plan_integration_pagerduty.rs");
        include!("types/ssmincidents/get_replication_set_region.rs");
        include!("types/ssmincidents/get_response_plan_action.rs");
        include!("types/ssmincidents/get_response_plan_action_ssm_automation.rs");
        include!(
            "types/ssmincidents/get_response_plan_action_ssm_automation_parameter.rs"
        );
        include!("types/ssmincidents/get_response_plan_incident_template.rs");
        include!(
            "types/ssmincidents/get_response_plan_incident_template_notification_target.rs"
        );
        include!("types/ssmincidents/get_response_plan_integration.rs");
        include!("types/ssmincidents/get_response_plan_integration_pagerduty.rs");
    }
    include!("types/get_availability_zone_filter.rs");
    include!("types/get_availability_zones_filter.rs");
    include!("types/get_regions_filter.rs");
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-aws {
    import output-interface;
}

interface pulumi-engine {
    resource engine {
        constructor(in-preview: bool);
    }
}

interface output-interface {
    use pulumi-engine.{engine};

    resource output {
        constructor(engine: borrow<engine>, value: string, secret: bool);
        map: func(function-name: string) -> output;
    }
    combine: func(outputs: list<borrow<output>>) -> output;

    resource register-output {
        extract-field: func(field-name: string) -> output;
    }
}

interface register-interface {
    use pulumi-engine.{engine};
    use output-interface.{output, register-output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record register-resource-request {
        %type: string,
        name: string,
        version: string,
        object: list<object-field>,
    }

    register: func(engine: borrow<engine>, request: register-resource-request) -> register-output;

    record resource-invoke-request {
        token: string,
        version: string,
        object: list<object-field>,
    }

    invoke: func(engine: borrow<engine>, request: resource-invoke-request) -> register-output;
}",
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        } }
    );
}
#[link_section = "pulumi_wasm_provider::aws"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_AWS: [u8; 45] = *b"{\"version\":\"6.66.2\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.66.2".to_string()
}
