pub mod scheduler {
    include!("resources/scheduler/schedule.rs");
    include!("resources/scheduler/schedule_group.rs");
}
pub mod schemas {
    include!("resources/schemas/discoverer.rs");
    include!("resources/schemas/registry.rs");
    include!("resources/schemas/registry_policy.rs");
    include!("resources/schemas/schema.rs");
}
pub mod secretsmanager {
    include!("resources/secretsmanager/secret.rs");
    include!("resources/secretsmanager/secret_policy.rs");
    include!("resources/secretsmanager/secret_rotation.rs");
    include!("resources/secretsmanager/secret_version.rs");
}
pub mod securityhub {
    include!("resources/securityhub/account.rs");
    include!("resources/securityhub/action_target.rs");
    include!("resources/securityhub/automation_rule.rs");
    include!("resources/securityhub/configuration_policy.rs");
    include!("resources/securityhub/configuration_policy_association.rs");
    include!("resources/securityhub/finding_aggregator.rs");
    include!("resources/securityhub/insight.rs");
    include!("resources/securityhub/invite_accepter.rs");
    include!("resources/securityhub/member.rs");
    include!("resources/securityhub/organization_admin_account.rs");
    include!("resources/securityhub/organization_configuration.rs");
    include!("resources/securityhub/product_subscription.rs");
    include!("resources/securityhub/standards_control.rs");
    include!("resources/securityhub/standards_control_association.rs");
    include!("resources/securityhub/standards_subscription.rs");
}
pub mod securitylake {
    include!("resources/securitylake/aws_log_source.rs");
    include!("resources/securitylake/custom_log_source.rs");
    include!("resources/securitylake/data_lake.rs");
    include!("resources/securitylake/subscriber.rs");
    include!("resources/securitylake/subscriber_notification.rs");
}
pub mod serverlessrepository {
    include!("resources/serverlessrepository/cloud_formation_stack.rs");
}
pub mod servicecatalog {
    include!("resources/servicecatalog/appregistry_application.rs");
    include!("resources/servicecatalog/appregistry_attribute_group.rs");
    include!("resources/servicecatalog/appregistry_attribute_group_association.rs");
    include!("resources/servicecatalog/budget_resource_association.rs");
    include!("resources/servicecatalog/constraint.rs");
    include!("resources/servicecatalog/organizations_access.rs");
    include!("resources/servicecatalog/portfolio.rs");
    include!("resources/servicecatalog/portfolio_share.rs");
    include!("resources/servicecatalog/principal_portfolio_association.rs");
    include!("resources/servicecatalog/product.rs");
    include!("resources/servicecatalog/product_portfolio_association.rs");
    include!("resources/servicecatalog/provisioned_product.rs");
    include!("resources/servicecatalog/provisioning_artifact.rs");
    include!("resources/servicecatalog/service_action.rs");
    include!("resources/servicecatalog/tag_option.rs");
    include!("resources/servicecatalog/tag_option_resource_association.rs");
}
pub mod servicediscovery {
    include!("resources/servicediscovery/http_namespace.rs");
    include!("resources/servicediscovery/instance.rs");
    include!("resources/servicediscovery/private_dns_namespace.rs");
    include!("resources/servicediscovery/public_dns_namespace.rs");
    include!("resources/servicediscovery/service.rs");
}
pub mod servicequotas {
    include!("resources/servicequotas/service_quota.rs");
    include!("resources/servicequotas/template.rs");
    include!("resources/servicequotas/template_association.rs");
}
pub mod ses {
    include!("resources/ses/active_receipt_rule_set.rs");
    include!("resources/ses/configuration_set.rs");
    include!("resources/ses/domain_dkim.rs");
    include!("resources/ses/domain_identity.rs");
    include!("resources/ses/domain_identity_verification.rs");
    include!("resources/ses/email_identity.rs");
    include!("resources/ses/event_destination.rs");
    include!("resources/ses/identity_notification_topic.rs");
    include!("resources/ses/identity_policy.rs");
    include!("resources/ses/mail_from.rs");
    include!("resources/ses/receipt_filter.rs");
    include!("resources/ses/receipt_rule.rs");
    include!("resources/ses/receipt_rule_set.rs");
    include!("resources/ses/template.rs");
}
pub mod functions {
    pub mod secretsmanager {
        include!("functions/secretsmanager/get_random_password.rs");
        include!("functions/secretsmanager/get_secret.rs");
        include!("functions/secretsmanager/get_secret_rotation.rs");
        include!("functions/secretsmanager/get_secret_version.rs");
        include!("functions/secretsmanager/get_secret_versions.rs");
        include!("functions/secretsmanager/get_secrets.rs");
    }
    pub mod securityhub {
        include!("functions/securityhub/get_standards_control_associations.rs");
    }
    pub mod serverlessrepository {
        include!("functions/serverlessrepository/get_application.rs");
    }
    pub mod servicecatalog {
        include!("functions/servicecatalog/get_appregistry_application.rs");
        include!("functions/servicecatalog/get_appregistry_attribute_group.rs");
        include!(
            "functions/servicecatalog/get_appregistry_attribute_group_associations.rs"
        );
        include!("functions/servicecatalog/get_constraint.rs");
        include!("functions/servicecatalog/get_launch_paths.rs");
        include!("functions/servicecatalog/get_portfolio.rs");
        include!("functions/servicecatalog/get_portfolio_constraints.rs");
        include!("functions/servicecatalog/get_product.rs");
        include!("functions/servicecatalog/get_provisioning_artifacts.rs");
    }
    pub mod servicediscovery {
        include!("functions/servicediscovery/get_dns_namespace.rs");
        include!("functions/servicediscovery/get_http_namespace.rs");
        include!("functions/servicediscovery/get_service.rs");
    }
    pub mod servicequotas {
        include!("functions/servicequotas/get_service.rs");
        include!("functions/servicequotas/get_service_quota.rs");
        include!("functions/servicequotas/get_templates.rs");
    }
    pub mod ses {
        include!("functions/ses/get_active_receipt_rule_set.rs");
        include!("functions/ses/get_domain_identity.rs");
        include!("functions/ses/get_email_identity.rs");
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
    pub mod scheduler {
        include!("types/scheduler/schedule_flexible_time_window.rs");
        include!("types/scheduler/schedule_target.rs");
        include!("types/scheduler/schedule_target_dead_letter_config.rs");
        include!("types/scheduler/schedule_target_ecs_parameters.rs");
        include!(
            "types/scheduler/schedule_target_ecs_parameters_capacity_provider_strategy.rs"
        );
        include!(
            "types/scheduler/schedule_target_ecs_parameters_network_configuration.rs"
        );
        include!(
            "types/scheduler/schedule_target_ecs_parameters_placement_constraint.rs"
        );
        include!("types/scheduler/schedule_target_ecs_parameters_placement_strategy.rs");
        include!("types/scheduler/schedule_target_eventbridge_parameters.rs");
        include!("types/scheduler/schedule_target_kinesis_parameters.rs");
        include!("types/scheduler/schedule_target_retry_policy.rs");
        include!("types/scheduler/schedule_target_sagemaker_pipeline_parameters.rs");
        include!(
            "types/scheduler/schedule_target_sagemaker_pipeline_parameters_pipeline_parameter.rs"
        );
        include!("types/scheduler/schedule_target_sqs_parameters.rs");
    }
    pub mod secretsmanager {
        include!("types/secretsmanager/secret_replica.rs");
        include!("types/secretsmanager/secret_rotation_rotation_rules.rs");
        include!("types/secretsmanager/get_secret_rotation_rotation_rule.rs");
        include!("types/secretsmanager/get_secret_versions_version.rs");
        include!("types/secretsmanager/get_secrets_filter.rs");
    }
    pub mod securityhub {
        include!("types/securityhub/automation_rule_action.rs");
        include!("types/securityhub/automation_rule_action_finding_fields_update.rs");
        include!(
            "types/securityhub/automation_rule_action_finding_fields_update_note.rs"
        );
        include!(
            "types/securityhub/automation_rule_action_finding_fields_update_related_finding.rs"
        );
        include!(
            "types/securityhub/automation_rule_action_finding_fields_update_severity.rs"
        );
        include!(
            "types/securityhub/automation_rule_action_finding_fields_update_workflow.rs"
        );
        include!("types/securityhub/automation_rule_criteria.rs");
        include!("types/securityhub/automation_rule_criteria_aws_account_id.rs");
        include!("types/securityhub/automation_rule_criteria_aws_account_name.rs");
        include!("types/securityhub/automation_rule_criteria_company_name.rs");
        include!(
            "types/securityhub/automation_rule_criteria_compliance_associated_standards_id.rs"
        );
        include!(
            "types/securityhub/automation_rule_criteria_compliance_security_control_id.rs"
        );
        include!("types/securityhub/automation_rule_criteria_compliance_status.rs");
        include!("types/securityhub/automation_rule_criteria_confidence.rs");
        include!("types/securityhub/automation_rule_criteria_created_at.rs");
        include!("types/securityhub/automation_rule_criteria_created_at_date_range.rs");
        include!("types/securityhub/automation_rule_criteria_criticality.rs");
        include!("types/securityhub/automation_rule_criteria_description.rs");
        include!("types/securityhub/automation_rule_criteria_first_observed_at.rs");
        include!(
            "types/securityhub/automation_rule_criteria_first_observed_at_date_range.rs"
        );
        include!("types/securityhub/automation_rule_criteria_generator_id.rs");
        include!("types/securityhub/automation_rule_criteria_id.rs");
        include!("types/securityhub/automation_rule_criteria_last_observed_at.rs");
        include!(
            "types/securityhub/automation_rule_criteria_last_observed_at_date_range.rs"
        );
        include!("types/securityhub/automation_rule_criteria_note_text.rs");
        include!("types/securityhub/automation_rule_criteria_note_updated_at.rs");
        include!(
            "types/securityhub/automation_rule_criteria_note_updated_at_date_range.rs"
        );
        include!("types/securityhub/automation_rule_criteria_note_updated_by.rs");
        include!("types/securityhub/automation_rule_criteria_product_arn.rs");
        include!("types/securityhub/automation_rule_criteria_product_name.rs");
        include!("types/securityhub/automation_rule_criteria_record_state.rs");
        include!("types/securityhub/automation_rule_criteria_related_findings_id.rs");
        include!(
            "types/securityhub/automation_rule_criteria_related_findings_product_arn.rs"
        );
        include!(
            "types/securityhub/automation_rule_criteria_resource_application_arn.rs"
        );
        include!(
            "types/securityhub/automation_rule_criteria_resource_application_name.rs"
        );
        include!("types/securityhub/automation_rule_criteria_resource_details_other.rs");
        include!("types/securityhub/automation_rule_criteria_resource_id.rs");
        include!("types/securityhub/automation_rule_criteria_resource_partition.rs");
        include!("types/securityhub/automation_rule_criteria_resource_region.rs");
        include!("types/securityhub/automation_rule_criteria_resource_tag.rs");
        include!("types/securityhub/automation_rule_criteria_resource_type.rs");
        include!("types/securityhub/automation_rule_criteria_severity_label.rs");
        include!("types/securityhub/automation_rule_criteria_source_url.rs");
        include!("types/securityhub/automation_rule_criteria_title.rs");
        include!("types/securityhub/automation_rule_criteria_type.rs");
        include!("types/securityhub/automation_rule_criteria_updated_at.rs");
        include!("types/securityhub/automation_rule_criteria_updated_at_date_range.rs");
        include!("types/securityhub/automation_rule_criteria_user_defined_field.rs");
        include!("types/securityhub/automation_rule_criteria_verification_state.rs");
        include!("types/securityhub/automation_rule_criteria_workflow_status.rs");
        include!("types/securityhub/configuration_policy_configuration_policy.rs");
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_bool.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_double.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_enum.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_enum_list.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_int.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_int_list.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_string.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_string_list.rs"
        );
        include!("types/securityhub/insight_filters.rs");
        include!("types/securityhub/insight_filters_aws_account_id.rs");
        include!("types/securityhub/insight_filters_company_name.rs");
        include!("types/securityhub/insight_filters_compliance_status.rs");
        include!("types/securityhub/insight_filters_confidence.rs");
        include!("types/securityhub/insight_filters_created_at.rs");
        include!("types/securityhub/insight_filters_created_at_date_range.rs");
        include!("types/securityhub/insight_filters_criticality.rs");
        include!("types/securityhub/insight_filters_description.rs");
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_confidence.rs"
        );
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_criticality.rs"
        );
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_related_findings_id.rs"
        );
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_related_findings_product_arn.rs"
        );
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_severity_label.rs"
        );
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_severity_original.rs"
        );
        include!("types/securityhub/insight_filters_finding_provider_fields_type.rs");
        include!("types/securityhub/insight_filters_first_observed_at.rs");
        include!("types/securityhub/insight_filters_first_observed_at_date_range.rs");
        include!("types/securityhub/insight_filters_generator_id.rs");
        include!("types/securityhub/insight_filters_id.rs");
        include!("types/securityhub/insight_filters_keyword.rs");
        include!("types/securityhub/insight_filters_last_observed_at.rs");
        include!("types/securityhub/insight_filters_last_observed_at_date_range.rs");
        include!("types/securityhub/insight_filters_malware_name.rs");
        include!("types/securityhub/insight_filters_malware_path.rs");
        include!("types/securityhub/insight_filters_malware_state.rs");
        include!("types/securityhub/insight_filters_malware_type.rs");
        include!("types/securityhub/insight_filters_network_destination_domain.rs");
        include!("types/securityhub/insight_filters_network_destination_ipv_4.rs");
        include!("types/securityhub/insight_filters_network_destination_ipv_6.rs");
        include!("types/securityhub/insight_filters_network_destination_port.rs");
        include!("types/securityhub/insight_filters_network_direction.rs");
        include!("types/securityhub/insight_filters_network_protocol.rs");
        include!("types/securityhub/insight_filters_network_source_domain.rs");
        include!("types/securityhub/insight_filters_network_source_ipv_4.rs");
        include!("types/securityhub/insight_filters_network_source_ipv_6.rs");
        include!("types/securityhub/insight_filters_network_source_mac.rs");
        include!("types/securityhub/insight_filters_network_source_port.rs");
        include!("types/securityhub/insight_filters_note_text.rs");
        include!("types/securityhub/insight_filters_note_updated_at.rs");
        include!("types/securityhub/insight_filters_note_updated_at_date_range.rs");
        include!("types/securityhub/insight_filters_note_updated_by.rs");
        include!("types/securityhub/insight_filters_process_launched_at.rs");
        include!("types/securityhub/insight_filters_process_launched_at_date_range.rs");
        include!("types/securityhub/insight_filters_process_name.rs");
        include!("types/securityhub/insight_filters_process_parent_pid.rs");
        include!("types/securityhub/insight_filters_process_path.rs");
        include!("types/securityhub/insight_filters_process_pid.rs");
        include!("types/securityhub/insight_filters_process_terminated_at.rs");
        include!(
            "types/securityhub/insight_filters_process_terminated_at_date_range.rs"
        );
        include!("types/securityhub/insight_filters_product_arn.rs");
        include!("types/securityhub/insight_filters_product_field.rs");
        include!("types/securityhub/insight_filters_product_name.rs");
        include!("types/securityhub/insight_filters_recommendation_text.rs");
        include!("types/securityhub/insight_filters_record_state.rs");
        include!("types/securityhub/insight_filters_related_findings_id.rs");
        include!("types/securityhub/insight_filters_related_findings_product_arn.rs");
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_iam_instance_profile_arn.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_image_id.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_ipv_4_address.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_ipv_6_address.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_key_name.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_launched_at.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_launched_at_date_range.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_subnet_id.rs"
        );
        include!("types/securityhub/insight_filters_resource_aws_ec_2_instance_type.rs");
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_vpc_id.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_iam_access_key_created_at.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_iam_access_key_created_at_date_range.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_iam_access_key_status.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_iam_access_key_user_name.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_s_3_bucket_owner_id.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_s_3_bucket_owner_name.rs"
        );
        include!("types/securityhub/insight_filters_resource_container_image_id.rs");
        include!("types/securityhub/insight_filters_resource_container_image_name.rs");
        include!("types/securityhub/insight_filters_resource_container_launched_at.rs");
        include!(
            "types/securityhub/insight_filters_resource_container_launched_at_date_range.rs"
        );
        include!("types/securityhub/insight_filters_resource_container_name.rs");
        include!("types/securityhub/insight_filters_resource_details_other.rs");
        include!("types/securityhub/insight_filters_resource_id.rs");
        include!("types/securityhub/insight_filters_resource_partition.rs");
        include!("types/securityhub/insight_filters_resource_region.rs");
        include!("types/securityhub/insight_filters_resource_tag.rs");
        include!("types/securityhub/insight_filters_resource_type.rs");
        include!("types/securityhub/insight_filters_severity_label.rs");
        include!("types/securityhub/insight_filters_source_url.rs");
        include!("types/securityhub/insight_filters_threat_intel_indicator_category.rs");
        include!(
            "types/securityhub/insight_filters_threat_intel_indicator_last_observed_at.rs"
        );
        include!(
            "types/securityhub/insight_filters_threat_intel_indicator_last_observed_at_date_range.rs"
        );
        include!("types/securityhub/insight_filters_threat_intel_indicator_source.rs");
        include!(
            "types/securityhub/insight_filters_threat_intel_indicator_source_url.rs"
        );
        include!("types/securityhub/insight_filters_threat_intel_indicator_type.rs");
        include!("types/securityhub/insight_filters_threat_intel_indicator_value.rs");
        include!("types/securityhub/insight_filters_title.rs");
        include!("types/securityhub/insight_filters_type.rs");
        include!("types/securityhub/insight_filters_updated_at.rs");
        include!("types/securityhub/insight_filters_updated_at_date_range.rs");
        include!("types/securityhub/insight_filters_user_defined_value.rs");
        include!("types/securityhub/insight_filters_verification_state.rs");
        include!("types/securityhub/insight_filters_workflow_status.rs");
        include!(
            "types/securityhub/organization_configuration_organization_configuration.rs"
        );
        include!(
            "types/securityhub/get_standards_control_associations_standards_control_association.rs"
        );
    }
    pub mod securitylake {
        include!("types/securitylake/aws_log_source_source.rs");
        include!("types/securitylake/custom_log_source_attribute.rs");
        include!("types/securitylake/custom_log_source_configuration.rs");
        include!(
            "types/securitylake/custom_log_source_configuration_crawler_configuration.rs"
        );
        include!(
            "types/securitylake/custom_log_source_configuration_provider_identity.rs"
        );
        include!("types/securitylake/custom_log_source_provider_detail.rs");
        include!("types/securitylake/data_lake_configuration.rs");
        include!(
            "types/securitylake/data_lake_configuration_encryption_configuration.rs"
        );
        include!(
            "types/securitylake/data_lake_configuration_lifecycle_configuration.rs"
        );
        include!(
            "types/securitylake/data_lake_configuration_lifecycle_configuration_expiration.rs"
        );
        include!(
            "types/securitylake/data_lake_configuration_lifecycle_configuration_transition.rs"
        );
        include!(
            "types/securitylake/data_lake_configuration_replication_configuration.rs"
        );
        include!("types/securitylake/data_lake_timeouts.rs");
        include!("types/securitylake/subscriber_notification_configuration.rs");
        include!(
            "types/securitylake/subscriber_notification_configuration_https_notification_configuration.rs"
        );
        include!(
            "types/securitylake/subscriber_notification_configuration_sqs_notification_configuration.rs"
        );
        include!("types/securitylake/subscriber_source.rs");
        include!("types/securitylake/subscriber_source_aws_log_source_resource.rs");
        include!("types/securitylake/subscriber_source_custom_log_source_resource.rs");
        include!(
            "types/securitylake/subscriber_source_custom_log_source_resource_attribute.rs"
        );
        include!(
            "types/securitylake/subscriber_source_custom_log_source_resource_provider.rs"
        );
        include!("types/securitylake/subscriber_subscriber_identity.rs");
        include!("types/securitylake/subscriber_timeouts.rs");
    }
    pub mod servicecatalog {
        include!("types/servicecatalog/product_provisioning_artifact_parameters.rs");
        include!("types/servicecatalog/provisioned_product_output.rs");
        include!("types/servicecatalog/provisioned_product_provisioning_parameter.rs");
        include!(
            "types/servicecatalog/provisioned_product_stack_set_provisioning_preferences.rs"
        );
        include!("types/servicecatalog/service_action_definition.rs");
        include!("types/servicecatalog/get_launch_paths_summary.rs");
        include!("types/servicecatalog/get_launch_paths_summary_constraint_summary.rs");
        include!("types/servicecatalog/get_portfolio_constraints_detail.rs");
        include!(
            "types/servicecatalog/get_provisioning_artifacts_provisioning_artifact_detail.rs"
        );
    }
    pub mod servicediscovery {
        include!("types/servicediscovery/service_dns_config.rs");
        include!("types/servicediscovery/service_dns_config_dns_record.rs");
        include!("types/servicediscovery/service_health_check_config.rs");
        include!("types/servicediscovery/service_health_check_custom_config.rs");
        include!("types/servicediscovery/get_service_dns_config.rs");
        include!("types/servicediscovery/get_service_dns_config_dns_record.rs");
        include!("types/servicediscovery/get_service_health_check_config.rs");
        include!("types/servicediscovery/get_service_health_check_custom_config.rs");
    }
    pub mod servicequotas {
        include!("types/servicequotas/service_quota_usage_metric.rs");
        include!("types/servicequotas/service_quota_usage_metric_metric_dimension.rs");
        include!("types/servicequotas/get_service_quota_usage_metric.rs");
        include!(
            "types/servicequotas/get_service_quota_usage_metric_metric_dimension.rs"
        );
        include!("types/servicequotas/get_templates_template.rs");
    }
    pub mod ses {
        include!("types/ses/configuration_set_delivery_options.rs");
        include!("types/ses/configuration_set_tracking_options.rs");
        include!("types/ses/event_destination_cloudwatch_destination.rs");
        include!("types/ses/event_destination_kinesis_destination.rs");
        include!("types/ses/event_destination_sns_destination.rs");
        include!("types/ses/receipt_rule_add_header_action.rs");
        include!("types/ses/receipt_rule_bounce_action.rs");
        include!("types/ses/receipt_rule_lambda_action.rs");
        include!("types/ses/receipt_rule_s_3_action.rs");
        include!("types/ses/receipt_rule_sns_action.rs");
        include!("types/ses/receipt_rule_stop_action.rs");
        include!("types/ses/receipt_rule_workmail_action.rs");
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
#[link_section = "pulumi_wasm_provider::aws"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
pub static PULUMI_WASM_PROVIDER_AWS: [u8; 45] = *b"{\"version\":\"6.66.2\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.66.2".to_string()
}
