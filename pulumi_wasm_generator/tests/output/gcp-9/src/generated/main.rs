pub mod oracledatabase {
    include!("resources/oracledatabase/autonomous_database.rs");
    include!("resources/oracledatabase/cloud_exadata_infrastructure.rs");
    include!("resources/oracledatabase/cloud_vm_cluster.rs");
}
pub mod organizations {
    include!("resources/organizations/access_approval_settings.rs");
    include!("resources/organizations/folder.rs");
    include!("resources/organizations/iam_binding.rs");
    include!("resources/organizations/iam_custom_role.rs");
    include!("resources/organizations/iam_member.rs");
    include!("resources/organizations/iam_policy.rs");
    include!("resources/organizations/iam_audit_config.rs");
    include!("resources/organizations/policy.rs");
    include!("resources/organizations/project.rs");
}
pub mod orgpolicy {
    include!("resources/orgpolicy/custom_constraint.rs");
    include!("resources/orgpolicy/policy.rs");
}
pub mod osconfig {
    include!("resources/osconfig/guest_policies.rs");
    include!("resources/osconfig/os_policy_assignment.rs");
    include!("resources/osconfig/patch_deployment.rs");
}
pub mod oslogin {
    include!("resources/oslogin/ssh_public_key.rs");
}
pub mod parallelstore {
    include!("resources/parallelstore/instance.rs");
}
pub mod privilegedaccessmanager {
    include!("resources/privilegedaccessmanager/entitlement.rs");
}
pub mod projects {
    include!("resources/projects/access_approval_settings.rs");
    include!("resources/projects/api_key.rs");
    include!("resources/projects/default_service_accounts.rs");
    include!("resources/projects/iam_audit_config.rs");
    include!("resources/projects/iam_binding.rs");
    include!("resources/projects/iam_custom_role.rs");
    include!("resources/projects/iam_member.rs");
    include!("resources/projects/iam_policy.rs");
    include!("resources/projects/iam_member_remove.rs");
    include!("resources/projects/organization_policy.rs");
    include!("resources/projects/service.rs");
    include!("resources/projects/service_identity.rs");
    include!("resources/projects/usage_export_bucket.rs");
}
pub mod pubsub {
    include!("resources/pubsub/lite_reservation.rs");
    include!("resources/pubsub/lite_subscription.rs");
    include!("resources/pubsub/lite_topic.rs");
    include!("resources/pubsub/schema.rs");
    include!("resources/pubsub/schema_iam_binding.rs");
    include!("resources/pubsub/schema_iam_member.rs");
    include!("resources/pubsub/schema_iam_policy.rs");
    include!("resources/pubsub/subscription.rs");
    include!("resources/pubsub/subscription_iam_binding.rs");
    include!("resources/pubsub/subscription_iam_member.rs");
    include!("resources/pubsub/subscription_iam_policy.rs");
    include!("resources/pubsub/topic.rs");
    include!("resources/pubsub/topic_iam_binding.rs");
    include!("resources/pubsub/topic_iam_member.rs");
    include!("resources/pubsub/topic_iam_policy.rs");
}
pub mod recaptcha {
    include!("resources/recaptcha/enterprise_key.rs");
}
pub mod functions {
    pub mod oracledatabase {
        include!("functions/oracledatabase/get_autonomous_database.rs");
        include!("functions/oracledatabase/get_autonomous_databases.rs");
        include!("functions/oracledatabase/get_cloud_exadata_infrastructure.rs");
        include!("functions/oracledatabase/get_cloud_exadata_infrastructures.rs");
        include!("functions/oracledatabase/get_cloud_vm_cluster.rs");
        include!("functions/oracledatabase/get_cloud_vm_clusters.rs");
        include!("functions/oracledatabase/get_db_nodes.rs");
        include!("functions/oracledatabase/get_db_servers.rs");
    }
    pub mod organizations {
        include!("functions/organizations/get_active_folder.rs");
        include!("functions/organizations/get_billing_account.rs");
        include!("functions/organizations/get_client_config.rs");
        include!("functions/organizations/get_client_open_id_user_info.rs");
        include!("functions/organizations/get_folder.rs");
        include!("functions/organizations/get_folders.rs");
        include!("functions/organizations/get_iam_policy.rs");
        include!("functions/organizations/get_organization.rs");
        include!("functions/organizations/get_project.rs");
    }
    pub mod privilegedaccessmanager {
        include!("functions/privilegedaccessmanager/get_entitlement.rs");
    }
    pub mod projects {
        include!("functions/projects/get_iam_policy.rs");
        include!("functions/projects/get_organization_policy.rs");
        include!("functions/projects/get_project.rs");
        include!("functions/projects/get_project_service.rs");
    }
    pub mod pubsub {
        include!("functions/pubsub/get_schema_iam_policy.rs");
        include!("functions/pubsub/get_subscription.rs");
        include!("functions/pubsub/get_subscription_iam_policy.rs");
        include!("functions/pubsub/get_topic.rs");
        include!("functions/pubsub/get_topic_iam_policy.rs");
    }
}
pub mod types {
    pub mod oracledatabase {
        include!("types/oracledatabase/autonomous_database_properties.rs");
        include!("types/oracledatabase/autonomous_database_properties_apex_detail.rs");
        include!(
            "types/oracledatabase/autonomous_database_properties_connection_string.rs"
        );
        include!(
            "types/oracledatabase/autonomous_database_properties_connection_string_all_connection_string.rs"
        );
        include!(
            "types/oracledatabase/autonomous_database_properties_connection_string_profile.rs"
        );
        include!(
            "types/oracledatabase/autonomous_database_properties_connection_url.rs"
        );
        include!(
            "types/oracledatabase/autonomous_database_properties_customer_contact.rs"
        );
        include!(
            "types/oracledatabase/autonomous_database_properties_local_standby_db.rs"
        );
        include!(
            "types/oracledatabase/autonomous_database_properties_scheduled_operation_detail.rs"
        );
        include!(
            "types/oracledatabase/autonomous_database_properties_scheduled_operation_detail_start_time.rs"
        );
        include!(
            "types/oracledatabase/autonomous_database_properties_scheduled_operation_detail_stop_time.rs"
        );
        include!("types/oracledatabase/cloud_exadata_infrastructure_properties.rs");
        include!(
            "types/oracledatabase/cloud_exadata_infrastructure_properties_customer_contact.rs"
        );
        include!(
            "types/oracledatabase/cloud_exadata_infrastructure_properties_maintenance_window.rs"
        );
        include!("types/oracledatabase/cloud_vm_cluster_properties.rs");
        include!(
            "types/oracledatabase/cloud_vm_cluster_properties_diagnostics_data_collection_options.rs"
        );
        include!("types/oracledatabase/cloud_vm_cluster_properties_time_zone.rs");
        include!("types/oracledatabase/get_autonomous_database_property.rs");
        include!("types/oracledatabase/get_autonomous_database_property_apex_detail.rs");
        include!(
            "types/oracledatabase/get_autonomous_database_property_connection_string.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_database_property_connection_string_all_connection_string.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_database_property_connection_string_profile.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_database_property_connection_url.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_database_property_customer_contact.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_database_property_local_standby_db.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_database_property_scheduled_operation_detail.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_database_property_scheduled_operation_detail_start_time.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_database_property_scheduled_operation_detail_stop_time.rs"
        );
        include!("types/oracledatabase/get_autonomous_databases_autonomous_database.rs");
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property_apex_detail.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property_connection_string.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property_connection_string_all_connection_string.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property_connection_string_profile.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property_connection_url.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property_customer_contact.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property_local_standby_db.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property_scheduled_operation_detail.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property_scheduled_operation_detail_start_time.rs"
        );
        include!(
            "types/oracledatabase/get_autonomous_databases_autonomous_database_property_scheduled_operation_detail_stop_time.rs"
        );
        include!("types/oracledatabase/get_cloud_exadata_infrastructure_property.rs");
        include!(
            "types/oracledatabase/get_cloud_exadata_infrastructure_property_customer_contact.rs"
        );
        include!(
            "types/oracledatabase/get_cloud_exadata_infrastructure_property_maintenance_window.rs"
        );
        include!(
            "types/oracledatabase/get_cloud_exadata_infrastructures_cloud_exadata_infrastructure.rs"
        );
        include!(
            "types/oracledatabase/get_cloud_exadata_infrastructures_cloud_exadata_infrastructure_property.rs"
        );
        include!(
            "types/oracledatabase/get_cloud_exadata_infrastructures_cloud_exadata_infrastructure_property_customer_contact.rs"
        );
        include!(
            "types/oracledatabase/get_cloud_exadata_infrastructures_cloud_exadata_infrastructure_property_maintenance_window.rs"
        );
        include!("types/oracledatabase/get_cloud_vm_cluster_property.rs");
        include!(
            "types/oracledatabase/get_cloud_vm_cluster_property_diagnostics_data_collection_option.rs"
        );
        include!("types/oracledatabase/get_cloud_vm_cluster_property_time_zone.rs");
        include!("types/oracledatabase/get_cloud_vm_clusters_cloud_vm_cluster.rs");
        include!(
            "types/oracledatabase/get_cloud_vm_clusters_cloud_vm_cluster_property.rs"
        );
        include!(
            "types/oracledatabase/get_cloud_vm_clusters_cloud_vm_cluster_property_diagnostics_data_collection_option.rs"
        );
        include!(
            "types/oracledatabase/get_cloud_vm_clusters_cloud_vm_cluster_property_time_zone.rs"
        );
        include!("types/oracledatabase/get_db_nodes_db_node.rs");
        include!("types/oracledatabase/get_db_nodes_db_node_property.rs");
        include!("types/oracledatabase/get_db_servers_db_server.rs");
        include!("types/oracledatabase/get_db_servers_db_server_property.rs");
    }
    pub mod organizations {
        include!("types/organizations/access_approval_settings_enrolled_service.rs");
        include!("types/organizations/iam_binding_condition.rs");
        include!("types/organizations/iam_member_condition.rs");
        include!("types/organizations/iam_audit_config_audit_log_config.rs");
        include!("types/organizations/policy_boolean_policy.rs");
        include!("types/organizations/policy_list_policy.rs");
        include!("types/organizations/policy_list_policy_allow.rs");
        include!("types/organizations/policy_list_policy_deny.rs");
        include!("types/organizations/policy_restore_policy.rs");
        include!("types/organizations/get_folders_folder.rs");
        include!("types/organizations/get_iam_policy_audit_config.rs");
        include!("types/organizations/get_iam_policy_audit_config_audit_log_config.rs");
        include!("types/organizations/get_iam_policy_binding.rs");
        include!("types/organizations/get_iam_policy_binding_condition.rs");
    }
    pub mod orgpolicy {
        include!("types/orgpolicy/policy_dry_run_spec.rs");
        include!("types/orgpolicy/policy_dry_run_spec_rule.rs");
        include!("types/orgpolicy/policy_dry_run_spec_rule_condition.rs");
        include!("types/orgpolicy/policy_dry_run_spec_rule_values.rs");
        include!("types/orgpolicy/policy_spec.rs");
        include!("types/orgpolicy/policy_spec_rule.rs");
        include!("types/orgpolicy/policy_spec_rule_condition.rs");
        include!("types/orgpolicy/policy_spec_rule_values.rs");
    }
    pub mod osconfig {
        include!("types/osconfig/guest_policies_assignment.rs");
        include!("types/osconfig/guest_policies_assignment_group_label.rs");
        include!("types/osconfig/guest_policies_assignment_os_type.rs");
        include!("types/osconfig/guest_policies_package.rs");
        include!("types/osconfig/guest_policies_package_repository.rs");
        include!("types/osconfig/guest_policies_package_repository_apt.rs");
        include!("types/osconfig/guest_policies_package_repository_goo.rs");
        include!("types/osconfig/guest_policies_package_repository_yum.rs");
        include!("types/osconfig/guest_policies_package_repository_zypper.rs");
        include!("types/osconfig/guest_policies_recipe.rs");
        include!("types/osconfig/guest_policies_recipe_artifact.rs");
        include!("types/osconfig/guest_policies_recipe_artifact_gcs.rs");
        include!("types/osconfig/guest_policies_recipe_artifact_remote.rs");
        include!("types/osconfig/guest_policies_recipe_install_step.rs");
        include!(
            "types/osconfig/guest_policies_recipe_install_step_archive_extraction.rs"
        );
        include!(
            "types/osconfig/guest_policies_recipe_install_step_dpkg_installation.rs"
        );
        include!("types/osconfig/guest_policies_recipe_install_step_file_copy.rs");
        include!("types/osconfig/guest_policies_recipe_install_step_file_exec.rs");
        include!(
            "types/osconfig/guest_policies_recipe_install_step_msi_installation.rs"
        );
        include!(
            "types/osconfig/guest_policies_recipe_install_step_rpm_installation.rs"
        );
        include!("types/osconfig/guest_policies_recipe_install_step_script_run.rs");
        include!("types/osconfig/guest_policies_recipe_update_step.rs");
        include!(
            "types/osconfig/guest_policies_recipe_update_step_archive_extraction.rs"
        );
        include!(
            "types/osconfig/guest_policies_recipe_update_step_dpkg_installation.rs"
        );
        include!("types/osconfig/guest_policies_recipe_update_step_file_copy.rs");
        include!("types/osconfig/guest_policies_recipe_update_step_file_exec.rs");
        include!("types/osconfig/guest_policies_recipe_update_step_msi_installation.rs");
        include!("types/osconfig/guest_policies_recipe_update_step_rpm_installation.rs");
        include!("types/osconfig/guest_policies_recipe_update_step_script_run.rs");
        include!("types/osconfig/os_policy_assignment_instance_filter.rs");
        include!(
            "types/osconfig/os_policy_assignment_instance_filter_exclusion_label.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_instance_filter_inclusion_label.rs"
        );
        include!("types/osconfig/os_policy_assignment_instance_filter_inventory.rs");
        include!("types/osconfig/os_policy_assignment_os_policy.rs");
        include!("types/osconfig/os_policy_assignment_os_policy_resource_group.rs");
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_inventory_filter.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_exec.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_exec_enforce.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_exec_enforce_file.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_exec_enforce_file_gcs.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_exec_enforce_file_remote.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_exec_validate.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_exec_validate_file.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_exec_validate_file_gcs.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_exec_validate_file_remote.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_file.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_file_file.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_file_file_gcs.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_file_file_remote.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_apt.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_deb.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_deb_source.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_deb_source_gcs.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_deb_source_remote.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_googet.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_msi.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_msi_source.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_msi_source_gcs.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_msi_source_remote.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_rpm.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_rpm_source.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_rpm_source_gcs.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_rpm_source_remote.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_yum.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_pkg_zypper.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_repository.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_repository_apt.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_repository_goo.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_repository_yum.rs"
        );
        include!(
            "types/osconfig/os_policy_assignment_os_policy_resource_group_resource_repository_zypper.rs"
        );
        include!("types/osconfig/os_policy_assignment_rollout.rs");
        include!("types/osconfig/os_policy_assignment_rollout_disruption_budget.rs");
        include!("types/osconfig/patch_deployment_instance_filter.rs");
        include!("types/osconfig/patch_deployment_instance_filter_group_label.rs");
        include!("types/osconfig/patch_deployment_one_time_schedule.rs");
        include!("types/osconfig/patch_deployment_patch_config.rs");
        include!("types/osconfig/patch_deployment_patch_config_apt.rs");
        include!("types/osconfig/patch_deployment_patch_config_goo.rs");
        include!("types/osconfig/patch_deployment_patch_config_post_step.rs");
        include!(
            "types/osconfig/patch_deployment_patch_config_post_step_linux_exec_step_config.rs"
        );
        include!(
            "types/osconfig/patch_deployment_patch_config_post_step_linux_exec_step_config_gcs_object.rs"
        );
        include!(
            "types/osconfig/patch_deployment_patch_config_post_step_windows_exec_step_config.rs"
        );
        include!(
            "types/osconfig/patch_deployment_patch_config_post_step_windows_exec_step_config_gcs_object.rs"
        );
        include!("types/osconfig/patch_deployment_patch_config_pre_step.rs");
        include!(
            "types/osconfig/patch_deployment_patch_config_pre_step_linux_exec_step_config.rs"
        );
        include!(
            "types/osconfig/patch_deployment_patch_config_pre_step_linux_exec_step_config_gcs_object.rs"
        );
        include!(
            "types/osconfig/patch_deployment_patch_config_pre_step_windows_exec_step_config.rs"
        );
        include!(
            "types/osconfig/patch_deployment_patch_config_pre_step_windows_exec_step_config_gcs_object.rs"
        );
        include!("types/osconfig/patch_deployment_patch_config_windows_update.rs");
        include!("types/osconfig/patch_deployment_patch_config_yum.rs");
        include!("types/osconfig/patch_deployment_patch_config_zypper.rs");
        include!("types/osconfig/patch_deployment_recurring_schedule.rs");
        include!("types/osconfig/patch_deployment_recurring_schedule_monthly.rs");
        include!(
            "types/osconfig/patch_deployment_recurring_schedule_monthly_week_day_of_month.rs"
        );
        include!("types/osconfig/patch_deployment_recurring_schedule_time_of_day.rs");
        include!("types/osconfig/patch_deployment_recurring_schedule_time_zone.rs");
        include!("types/osconfig/patch_deployment_recurring_schedule_weekly.rs");
        include!("types/osconfig/patch_deployment_rollout.rs");
        include!("types/osconfig/patch_deployment_rollout_disruption_budget.rs");
    }
    pub mod privilegedaccessmanager {
        include!(
            "types/privilegedaccessmanager/entitlement_additional_notification_targets.rs"
        );
        include!("types/privilegedaccessmanager/entitlement_approval_workflow.rs");
        include!(
            "types/privilegedaccessmanager/entitlement_approval_workflow_manual_approvals.rs"
        );
        include!(
            "types/privilegedaccessmanager/entitlement_approval_workflow_manual_approvals_step.rs"
        );
        include!(
            "types/privilegedaccessmanager/entitlement_approval_workflow_manual_approvals_step_approvers.rs"
        );
        include!("types/privilegedaccessmanager/entitlement_eligible_user.rs");
        include!("types/privilegedaccessmanager/entitlement_privileged_access.rs");
        include!(
            "types/privilegedaccessmanager/entitlement_privileged_access_gcp_iam_access.rs"
        );
        include!(
            "types/privilegedaccessmanager/entitlement_privileged_access_gcp_iam_access_role_binding.rs"
        );
        include!(
            "types/privilegedaccessmanager/entitlement_requester_justification_config.rs"
        );
        include!(
            "types/privilegedaccessmanager/entitlement_requester_justification_config_not_mandatory.rs"
        );
        include!(
            "types/privilegedaccessmanager/entitlement_requester_justification_config_unstructured.rs"
        );
        include!(
            "types/privilegedaccessmanager/get_entitlement_additional_notification_target.rs"
        );
        include!("types/privilegedaccessmanager/get_entitlement_approval_workflow.rs");
        include!(
            "types/privilegedaccessmanager/get_entitlement_approval_workflow_manual_approval.rs"
        );
        include!(
            "types/privilegedaccessmanager/get_entitlement_approval_workflow_manual_approval_step.rs"
        );
        include!(
            "types/privilegedaccessmanager/get_entitlement_approval_workflow_manual_approval_step_approver.rs"
        );
        include!("types/privilegedaccessmanager/get_entitlement_eligible_user.rs");
        include!("types/privilegedaccessmanager/get_entitlement_privileged_access.rs");
        include!(
            "types/privilegedaccessmanager/get_entitlement_privileged_access_gcp_iam_access.rs"
        );
        include!(
            "types/privilegedaccessmanager/get_entitlement_privileged_access_gcp_iam_access_role_binding.rs"
        );
        include!(
            "types/privilegedaccessmanager/get_entitlement_requester_justification_config.rs"
        );
        include!(
            "types/privilegedaccessmanager/get_entitlement_requester_justification_config_not_mandatory.rs"
        );
        include!(
            "types/privilegedaccessmanager/get_entitlement_requester_justification_config_unstructured.rs"
        );
    }
    pub mod projects {
        include!("types/projects/access_approval_settings_enrolled_service.rs");
        include!("types/projects/api_key_restrictions.rs");
        include!("types/projects/api_key_restrictions_android_key_restrictions.rs");
        include!(
            "types/projects/api_key_restrictions_android_key_restrictions_allowed_application.rs"
        );
        include!("types/projects/api_key_restrictions_api_target.rs");
        include!("types/projects/api_key_restrictions_browser_key_restrictions.rs");
        include!("types/projects/api_key_restrictions_ios_key_restrictions.rs");
        include!("types/projects/api_key_restrictions_server_key_restrictions.rs");
        include!("types/projects/iam_audit_config_audit_log_config.rs");
        include!("types/projects/iam_binding_condition.rs");
        include!("types/projects/iam_member_condition.rs");
        include!("types/projects/organization_policy_boolean_policy.rs");
        include!("types/projects/organization_policy_list_policy.rs");
        include!("types/projects/organization_policy_list_policy_allow.rs");
        include!("types/projects/organization_policy_list_policy_deny.rs");
        include!("types/projects/organization_policy_restore_policy.rs");
        include!("types/projects/get_organization_policy_boolean_policy.rs");
        include!("types/projects/get_organization_policy_list_policy.rs");
        include!("types/projects/get_organization_policy_list_policy_allow.rs");
        include!("types/projects/get_organization_policy_list_policy_deny.rs");
        include!("types/projects/get_organization_policy_restore_policy.rs");
        include!("types/projects/get_project_project.rs");
    }
    pub mod pubsub {
        include!("types/pubsub/lite_subscription_delivery_config.rs");
        include!("types/pubsub/lite_topic_partition_config.rs");
        include!("types/pubsub/lite_topic_partition_config_capacity.rs");
        include!("types/pubsub/lite_topic_reservation_config.rs");
        include!("types/pubsub/lite_topic_retention_config.rs");
        include!("types/pubsub/schema_iam_binding_condition.rs");
        include!("types/pubsub/schema_iam_member_condition.rs");
        include!("types/pubsub/subscription_bigquery_config.rs");
        include!("types/pubsub/subscription_cloud_storage_config.rs");
        include!("types/pubsub/subscription_cloud_storage_config_avro_config.rs");
        include!("types/pubsub/subscription_dead_letter_policy.rs");
        include!("types/pubsub/subscription_expiration_policy.rs");
        include!("types/pubsub/subscription_iam_binding_condition.rs");
        include!("types/pubsub/subscription_iam_member_condition.rs");
        include!("types/pubsub/subscription_push_config.rs");
        include!("types/pubsub/subscription_push_config_no_wrapper.rs");
        include!("types/pubsub/subscription_push_config_oidc_token.rs");
        include!("types/pubsub/subscription_retry_policy.rs");
        include!("types/pubsub/topic_iam_binding_condition.rs");
        include!("types/pubsub/topic_iam_member_condition.rs");
        include!("types/pubsub/topic_ingestion_data_source_settings.rs");
        include!("types/pubsub/topic_ingestion_data_source_settings_aws_kinesis.rs");
        include!("types/pubsub/topic_ingestion_data_source_settings_cloud_storage.rs");
        include!(
            "types/pubsub/topic_ingestion_data_source_settings_cloud_storage_avro_format.rs"
        );
        include!(
            "types/pubsub/topic_ingestion_data_source_settings_cloud_storage_pubsub_avro_format.rs"
        );
        include!(
            "types/pubsub/topic_ingestion_data_source_settings_cloud_storage_text_format.rs"
        );
        include!(
            "types/pubsub/topic_ingestion_data_source_settings_platform_logs_settings.rs"
        );
        include!("types/pubsub/topic_message_storage_policy.rs");
        include!("types/pubsub/topic_schema_settings.rs");
        include!("types/pubsub/get_subscription_bigquery_config.rs");
        include!("types/pubsub/get_subscription_cloud_storage_config.rs");
        include!("types/pubsub/get_subscription_cloud_storage_config_avro_config.rs");
        include!("types/pubsub/get_subscription_dead_letter_policy.rs");
        include!("types/pubsub/get_subscription_expiration_policy.rs");
        include!("types/pubsub/get_subscription_push_config.rs");
        include!("types/pubsub/get_subscription_push_config_no_wrapper.rs");
        include!("types/pubsub/get_subscription_push_config_oidc_token.rs");
        include!("types/pubsub/get_subscription_retry_policy.rs");
        include!("types/pubsub/get_topic_ingestion_data_source_setting.rs");
        include!("types/pubsub/get_topic_ingestion_data_source_setting_aws_kinese.rs");
        include!(
            "types/pubsub/get_topic_ingestion_data_source_setting_cloud_storage.rs"
        );
        include!(
            "types/pubsub/get_topic_ingestion_data_source_setting_cloud_storage_avro_format.rs"
        );
        include!(
            "types/pubsub/get_topic_ingestion_data_source_setting_cloud_storage_pubsub_avro_format.rs"
        );
        include!(
            "types/pubsub/get_topic_ingestion_data_source_setting_cloud_storage_text_format.rs"
        );
        include!(
            "types/pubsub/get_topic_ingestion_data_source_setting_platform_logs_setting.rs"
        );
        include!("types/pubsub/get_topic_message_storage_policy.rs");
        include!("types/pubsub/get_topic_schema_setting.rs");
    }
    pub mod recaptcha {
        include!("types/recaptcha/enterprise_key_android_settings.rs");
        include!("types/recaptcha/enterprise_key_ios_settings.rs");
        include!("types/recaptcha/enterprise_key_testing_options.rs");
        include!("types/recaptcha/enterprise_key_waf_settings.rs");
        include!("types/recaptcha/enterprise_key_web_settings.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-gcp {
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
