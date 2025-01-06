pub mod blueprint {
    include!("resources/blueprint/assignment.rs");
}
pub mod bot {
    include!("resources/bot/channel_alexa.rs");
    include!("resources/bot/channel_direct_line.rs");
    include!("resources/bot/channel_direct_line_speech.rs");
    include!("resources/bot/channel_email.rs");
    include!("resources/bot/channel_facebook.rs");
    include!("resources/bot/channel_line.rs");
    include!("resources/bot/channel_slack.rs");
    include!("resources/bot/channel_sms.rs");
    include!("resources/bot/channel_teams.rs");
    include!("resources/bot/channel_web_chat.rs");
    include!("resources/bot/channels_registration.rs");
    include!("resources/bot/connection.rs");
    include!("resources/bot/healthbot.rs");
    include!("resources/bot/service_azure_bot.rs");
    include!("resources/bot/web_app.rs");
}
pub mod cdn {
    include!("resources/cdn/endpoint.rs");
    include!("resources/cdn/endpoint_custom_domain.rs");
    include!("resources/cdn/frontdoor_custom_domain.rs");
    include!("resources/cdn/frontdoor_custom_domain_association.rs");
    include!("resources/cdn/frontdoor_endpoint.rs");
    include!("resources/cdn/frontdoor_firewall_policy.rs");
    include!("resources/cdn/frontdoor_origin.rs");
    include!("resources/cdn/frontdoor_origin_group.rs");
    include!("resources/cdn/frontdoor_profile.rs");
    include!("resources/cdn/frontdoor_route.rs");
    include!("resources/cdn/frontdoor_rule.rs");
    include!("resources/cdn/frontdoor_rule_set.rs");
    include!("resources/cdn/frontdoor_secret.rs");
    include!("resources/cdn/frontdoor_security_policy.rs");
    include!("resources/cdn/profile.rs");
}
pub mod chaosstudio {
    include!("resources/chaosstudio/capability.rs");
    include!("resources/chaosstudio/experiment.rs");
    include!("resources/chaosstudio/target.rs");
}
pub mod cognitive {
    include!("resources/cognitive/ai_services.rs");
    include!("resources/cognitive/account.rs");
    include!("resources/cognitive/account_customer_managed_key.rs");
    include!("resources/cognitive/account_rai_blocklist.rs");
    include!("resources/cognitive/deployment.rs");
}
pub mod communication {
    include!("resources/communication/email_service.rs");
    include!("resources/communication/email_service_domain.rs");
    include!("resources/communication/service.rs");
    include!("resources/communication/service_email_domain_association.rs");
}
pub mod compute {
    include!("resources/compute/automanage_configuration_assignment.rs");
    include!("resources/compute/availability_set.rs");
    include!("resources/compute/bastion_host.rs");
    include!("resources/compute/capacity_reservation.rs");
    include!("resources/compute/capacity_reservation_group.rs");
    include!("resources/compute/data_disk_attachment.rs");
    include!("resources/compute/dedicated_host.rs");
    include!("resources/compute/dedicated_host_group.rs");
    include!("resources/compute/disk_access.rs");
    include!("resources/compute/disk_encryption_set.rs");
    include!("resources/compute/extension.rs");
    include!("resources/compute/gallery_application.rs");
    include!("resources/compute/gallery_application_assignment.rs");
    include!("resources/compute/gallery_application_version.rs");
    include!("resources/compute/image.rs");
    include!("resources/compute/implicit_data_disk_from_source.rs");
    include!("resources/compute/linux_virtual_machine.rs");
    include!("resources/compute/linux_virtual_machine_scale_set.rs");
    include!("resources/compute/managed_disk.rs");
    include!("resources/compute/managed_disk_sas_token.rs");
    include!("resources/compute/orchestrated_virtual_machine_scale_set.rs");
    include!("resources/compute/packet_capture.rs");
    include!("resources/compute/restore_point.rs");
    include!("resources/compute/restore_point_collection.rs");
    include!("resources/compute/run_command.rs");
    include!("resources/compute/scale_set.rs");
    include!("resources/compute/scale_set_packet_capture.rs");
    include!("resources/compute/shared_image.rs");
    include!("resources/compute/shared_image_gallery.rs");
    include!("resources/compute/shared_image_version.rs");
    include!("resources/compute/snapshot.rs");
    include!("resources/compute/ssh_public_key.rs");
    include!("resources/compute/virtual_machine.rs");
    include!("resources/compute/virtual_machine_scale_set_extension.rs");
    include!("resources/compute/windows_virtual_machine.rs");
    include!("resources/compute/windows_virtual_machine_scale_set.rs");
}
pub mod confidentialledger {
    include!("resources/confidentialledger/ledger.rs");
}
pub mod connections {
    include!("resources/connections/api_connection.rs");
}
pub mod functions {
    pub mod blueprint {
        include!("functions/blueprint/get_definition.rs");
        include!("functions/blueprint/get_published_version.rs");
    }
    pub mod cdn {
        include!("functions/cdn/get_frontdoor_custom_domain.rs");
        include!("functions/cdn/get_frontdoor_endpoint.rs");
        include!("functions/cdn/get_frontdoor_firewall_policy.rs");
        include!("functions/cdn/get_frontdoor_origin_group.rs");
        include!("functions/cdn/get_frontdoor_profile.rs");
        include!("functions/cdn/get_frontdoor_rule_set.rs");
        include!("functions/cdn/get_frontdoor_secret.rs");
        include!("functions/cdn/get_profile.rs");
    }
    pub mod cognitive {
        include!("functions/cognitive/get_account.rs");
    }
    pub mod communication {
        include!("functions/communication/get_service.rs");
    }
    pub mod compute {
        include!("functions/compute/get_availability_set.rs");
        include!("functions/compute/get_bastion_host.rs");
        include!("functions/compute/get_confidential_ledger.rs");
        include!("functions/compute/get_dedicated_host.rs");
        include!("functions/compute/get_dedicated_host_group.rs");
        include!("functions/compute/get_disk_access.rs");
        include!("functions/compute/get_disk_encryption_set.rs");
        include!("functions/compute/get_image.rs");
        include!("functions/compute/get_images.rs");
        include!("functions/compute/get_managed_disk.rs");
        include!("functions/compute/get_orchestrated_virtual_machine_scale_set.rs");
        include!("functions/compute/get_platform_image.rs");
        include!("functions/compute/get_shared_image.rs");
        include!("functions/compute/get_shared_image_gallery.rs");
        include!("functions/compute/get_shared_image_version.rs");
        include!("functions/compute/get_shared_image_versions.rs");
        include!("functions/compute/get_snapshot.rs");
        include!("functions/compute/get_ssh_public_key.rs");
        include!("functions/compute/get_virtual_machine.rs");
        include!("functions/compute/get_virtual_machine_scale_set.rs");
    }
    pub mod connections {
        include!("functions/connections/get_managed_api.rs");
    }
}
pub mod types {
    pub mod blueprint {
        include!("types/blueprint/assignment_identity.rs");
    }
    pub mod bot {
        include!("types/bot/channel_direct_line_site.rs");
        include!("types/bot/channel_facebook_page.rs");
        include!("types/bot/channel_line_line_channel.rs");
        include!("types/bot/channel_web_chat_site.rs");
    }
    pub mod cdn {
        include!("types/cdn/endpoint_custom_domain_cdn_managed_https.rs");
        include!("types/cdn/endpoint_custom_domain_user_managed_https.rs");
        include!("types/cdn/endpoint_delivery_rule.rs");
        include!("types/cdn/endpoint_delivery_rule_cache_expiration_action.rs");
        include!("types/cdn/endpoint_delivery_rule_cache_key_query_string_action.rs");
        include!("types/cdn/endpoint_delivery_rule_cookies_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_device_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_http_version_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_modify_request_header_action.rs");
        include!("types/cdn/endpoint_delivery_rule_modify_response_header_action.rs");
        include!("types/cdn/endpoint_delivery_rule_post_arg_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_query_string_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_remote_address_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_request_body_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_request_header_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_request_method_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_request_scheme_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_request_uri_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_url_file_extension_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_url_file_name_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_url_path_condition.rs");
        include!("types/cdn/endpoint_delivery_rule_url_redirect_action.rs");
        include!("types/cdn/endpoint_delivery_rule_url_rewrite_action.rs");
        include!("types/cdn/endpoint_geo_filter.rs");
        include!("types/cdn/endpoint_global_delivery_rule.rs");
        include!("types/cdn/endpoint_global_delivery_rule_cache_expiration_action.rs");
        include!(
            "types/cdn/endpoint_global_delivery_rule_cache_key_query_string_action.rs"
        );
        include!(
            "types/cdn/endpoint_global_delivery_rule_modify_request_header_action.rs"
        );
        include!(
            "types/cdn/endpoint_global_delivery_rule_modify_response_header_action.rs"
        );
        include!("types/cdn/endpoint_global_delivery_rule_url_redirect_action.rs");
        include!("types/cdn/endpoint_global_delivery_rule_url_rewrite_action.rs");
        include!("types/cdn/endpoint_origin.rs");
        include!("types/cdn/frontdoor_custom_domain_tls.rs");
        include!("types/cdn/frontdoor_firewall_policy_custom_rule.rs");
        include!("types/cdn/frontdoor_firewall_policy_custom_rule_match_condition.rs");
        include!("types/cdn/frontdoor_firewall_policy_managed_rule.rs");
        include!("types/cdn/frontdoor_firewall_policy_managed_rule_exclusion.rs");
        include!("types/cdn/frontdoor_firewall_policy_managed_rule_override.rs");
        include!(
            "types/cdn/frontdoor_firewall_policy_managed_rule_override_exclusion.rs"
        );
        include!("types/cdn/frontdoor_firewall_policy_managed_rule_override_rule.rs");
        include!(
            "types/cdn/frontdoor_firewall_policy_managed_rule_override_rule_exclusion.rs"
        );
        include!("types/cdn/frontdoor_origin_group_health_probe.rs");
        include!("types/cdn/frontdoor_origin_group_load_balancing.rs");
        include!("types/cdn/frontdoor_origin_private_link.rs");
        include!("types/cdn/frontdoor_route_cache.rs");
        include!("types/cdn/frontdoor_rule_actions.rs");
        include!("types/cdn/frontdoor_rule_actions_request_header_action.rs");
        include!("types/cdn/frontdoor_rule_actions_response_header_action.rs");
        include!(
            "types/cdn/frontdoor_rule_actions_route_configuration_override_action.rs"
        );
        include!("types/cdn/frontdoor_rule_actions_url_redirect_action.rs");
        include!("types/cdn/frontdoor_rule_actions_url_rewrite_action.rs");
        include!("types/cdn/frontdoor_rule_conditions.rs");
        include!("types/cdn/frontdoor_rule_conditions_client_port_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_cookies_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_host_name_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_http_version_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_is_device_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_post_args_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_query_string_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_remote_address_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_request_body_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_request_header_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_request_method_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_request_scheme_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_request_uri_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_server_port_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_socket_address_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_ssl_protocol_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_url_file_extension_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_url_filename_condition.rs");
        include!("types/cdn/frontdoor_rule_conditions_url_path_condition.rs");
        include!("types/cdn/frontdoor_secret_secret.rs");
        include!("types/cdn/frontdoor_secret_secret_customer_certificate.rs");
        include!("types/cdn/frontdoor_security_policy_security_policies.rs");
        include!("types/cdn/frontdoor_security_policy_security_policies_firewall.rs");
        include!(
            "types/cdn/frontdoor_security_policy_security_policies_firewall_association.rs"
        );
        include!(
            "types/cdn/frontdoor_security_policy_security_policies_firewall_association_domain.rs"
        );
        include!("types/cdn/get_frontdoor_custom_domain_tl.rs");
        include!("types/cdn/get_frontdoor_origin_group_health_probe.rs");
        include!("types/cdn/get_frontdoor_origin_group_load_balancing.rs");
        include!("types/cdn/get_frontdoor_secret_secret.rs");
        include!("types/cdn/get_frontdoor_secret_secret_customer_certificate.rs");
    }
    pub mod chaosstudio {
        include!("types/chaosstudio/experiment_identity.rs");
        include!("types/chaosstudio/experiment_selector.rs");
        include!("types/chaosstudio/experiment_step.rs");
        include!("types/chaosstudio/experiment_step_branch.rs");
        include!("types/chaosstudio/experiment_step_branch_action.rs");
    }
    pub mod cognitive {
        include!("types/cognitive/ai_services_customer_managed_key.rs");
        include!("types/cognitive/ai_services_identity.rs");
        include!("types/cognitive/ai_services_network_acls.rs");
        include!("types/cognitive/ai_services_network_acls_virtual_network_rule.rs");
        include!("types/cognitive/ai_services_storage.rs");
        include!("types/cognitive/account_customer_managed_key.rs");
        include!("types/cognitive/account_identity.rs");
        include!("types/cognitive/account_network_acls.rs");
        include!("types/cognitive/account_network_acls_virtual_network_rule.rs");
        include!("types/cognitive/account_storage.rs");
        include!("types/cognitive/deployment_model.rs");
        include!("types/cognitive/deployment_sku.rs");
        include!("types/cognitive/get_account_identity.rs");
    }
    pub mod communication {
        include!("types/communication/email_service_domain_verification_record.rs");
        include!("types/communication/email_service_domain_verification_record_dkim.rs");
        include!(
            "types/communication/email_service_domain_verification_record_dkim_2.rs"
        );
        include!(
            "types/communication/email_service_domain_verification_record_dmarc.rs"
        );
        include!(
            "types/communication/email_service_domain_verification_record_domain.rs"
        );
        include!("types/communication/email_service_domain_verification_record_spf.rs");
    }
    pub mod compute {
        include!("types/compute/bastion_host_ip_configuration.rs");
        include!("types/compute/capacity_reservation_sku.rs");
        include!("types/compute/disk_encryption_set_identity.rs");
        include!("types/compute/extension_protected_settings_from_key_vault.rs");
        include!("types/compute/gallery_application_version_manage_action.rs");
        include!("types/compute/gallery_application_version_source.rs");
        include!("types/compute/gallery_application_version_target_region.rs");
        include!("types/compute/image_data_disk.rs");
        include!("types/compute/image_os_disk.rs");
        include!("types/compute/linux_virtual_machine_additional_capabilities.rs");
        include!("types/compute/linux_virtual_machine_admin_ssh_key.rs");
        include!("types/compute/linux_virtual_machine_boot_diagnostics.rs");
        include!("types/compute/linux_virtual_machine_gallery_application.rs");
        include!("types/compute/linux_virtual_machine_identity.rs");
        include!("types/compute/linux_virtual_machine_os_disk.rs");
        include!("types/compute/linux_virtual_machine_os_disk_diff_disk_settings.rs");
        include!("types/compute/linux_virtual_machine_os_image_notification.rs");
        include!("types/compute/linux_virtual_machine_plan.rs");
        include!(
            "types/compute/linux_virtual_machine_scale_set_additional_capabilities.rs"
        );
        include!("types/compute/linux_virtual_machine_scale_set_admin_ssh_key.rs");
        include!(
            "types/compute/linux_virtual_machine_scale_set_automatic_instance_repair.rs"
        );
        include!(
            "types/compute/linux_virtual_machine_scale_set_automatic_os_upgrade_policy.rs"
        );
        include!("types/compute/linux_virtual_machine_scale_set_boot_diagnostics.rs");
        include!("types/compute/linux_virtual_machine_scale_set_data_disk.rs");
        include!("types/compute/linux_virtual_machine_scale_set_extension.rs");
        include!(
            "types/compute/linux_virtual_machine_scale_set_extension_protected_settings_from_key_vault.rs"
        );
        include!("types/compute/linux_virtual_machine_scale_set_gallery_application.rs");
        include!("types/compute/linux_virtual_machine_scale_set_identity.rs");
        include!("types/compute/linux_virtual_machine_scale_set_network_interface.rs");
        include!(
            "types/compute/linux_virtual_machine_scale_set_network_interface_ip_configuration.rs"
        );
        include!(
            "types/compute/linux_virtual_machine_scale_set_network_interface_ip_configuration_public_ip_address.rs"
        );
        include!(
            "types/compute/linux_virtual_machine_scale_set_network_interface_ip_configuration_public_ip_address_ip_tag.rs"
        );
        include!("types/compute/linux_virtual_machine_scale_set_os_disk.rs");
        include!(
            "types/compute/linux_virtual_machine_scale_set_os_disk_diff_disk_settings.rs"
        );
        include!("types/compute/linux_virtual_machine_scale_set_plan.rs");
        include!(
            "types/compute/linux_virtual_machine_scale_set_rolling_upgrade_policy.rs"
        );
        include!("types/compute/linux_virtual_machine_scale_set_scale_in.rs");
        include!("types/compute/linux_virtual_machine_scale_set_secret.rs");
        include!("types/compute/linux_virtual_machine_scale_set_secret_certificate.rs");
        include!(
            "types/compute/linux_virtual_machine_scale_set_source_image_reference.rs"
        );
        include!("types/compute/linux_virtual_machine_scale_set_spot_restore.rs");
        include!(
            "types/compute/linux_virtual_machine_scale_set_termination_notification.rs"
        );
        include!("types/compute/linux_virtual_machine_secret.rs");
        include!("types/compute/linux_virtual_machine_secret_certificate.rs");
        include!("types/compute/linux_virtual_machine_source_image_reference.rs");
        include!("types/compute/linux_virtual_machine_termination_notification.rs");
        include!("types/compute/managed_disk_encryption_settings.rs");
        include!(
            "types/compute/managed_disk_encryption_settings_disk_encryption_key.rs"
        );
        include!("types/compute/managed_disk_encryption_settings_key_encryption_key.rs");
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_additional_capabilities.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_automatic_instance_repair.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_boot_diagnostics.rs"
        );
        include!("types/compute/orchestrated_virtual_machine_scale_set_data_disk.rs");
        include!("types/compute/orchestrated_virtual_machine_scale_set_extension.rs");
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_extension_protected_settings_from_key_vault.rs"
        );
        include!("types/compute/orchestrated_virtual_machine_scale_set_identity.rs");
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_network_interface.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_network_interface_ip_configuration.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_network_interface_ip_configuration_public_ip_address.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_network_interface_ip_configuration_public_ip_address_ip_tag.rs"
        );
        include!("types/compute/orchestrated_virtual_machine_scale_set_os_disk.rs");
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_os_disk_diff_disk_settings.rs"
        );
        include!("types/compute/orchestrated_virtual_machine_scale_set_os_profile.rs");
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_os_profile_linux_configuration.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_os_profile_linux_configuration_admin_ssh_key.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_os_profile_linux_configuration_secret.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_os_profile_linux_configuration_secret_certificate.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_os_profile_windows_configuration.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_os_profile_windows_configuration_additional_unattend_content.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_os_profile_windows_configuration_secret.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_os_profile_windows_configuration_secret_certificate.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_os_profile_windows_configuration_winrm_listener.rs"
        );
        include!("types/compute/orchestrated_virtual_machine_scale_set_plan.rs");
        include!("types/compute/orchestrated_virtual_machine_scale_set_priority_mix.rs");
        include!("types/compute/orchestrated_virtual_machine_scale_set_sku_profile.rs");
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_source_image_reference.rs"
        );
        include!(
            "types/compute/orchestrated_virtual_machine_scale_set_termination_notification.rs"
        );
        include!("types/compute/packet_capture_filter.rs");
        include!("types/compute/packet_capture_storage_location.rs");
        include!("types/compute/run_command_error_blob_managed_identity.rs");
        include!("types/compute/run_command_instance_view.rs");
        include!("types/compute/run_command_output_blob_managed_identity.rs");
        include!("types/compute/run_command_parameter.rs");
        include!("types/compute/run_command_protected_parameter.rs");
        include!("types/compute/run_command_source.rs");
        include!("types/compute/run_command_source_script_uri_managed_identity.rs");
        include!("types/compute/scale_set_boot_diagnostics.rs");
        include!("types/compute/scale_set_extension.rs");
        include!("types/compute/scale_set_identity.rs");
        include!("types/compute/scale_set_network_profile.rs");
        include!("types/compute/scale_set_network_profile_dns_settings.rs");
        include!("types/compute/scale_set_network_profile_ip_configuration.rs");
        include!(
            "types/compute/scale_set_network_profile_ip_configuration_public_ip_address_configuration.rs"
        );
        include!("types/compute/scale_set_os_profile.rs");
        include!("types/compute/scale_set_os_profile_linux_config.rs");
        include!("types/compute/scale_set_os_profile_linux_config_ssh_key.rs");
        include!("types/compute/scale_set_os_profile_secret.rs");
        include!("types/compute/scale_set_os_profile_secret_vault_certificate.rs");
        include!("types/compute/scale_set_os_profile_windows_config.rs");
        include!(
            "types/compute/scale_set_os_profile_windows_config_additional_unattend_config.rs"
        );
        include!("types/compute/scale_set_os_profile_windows_config_winrm.rs");
        include!("types/compute/scale_set_packet_capture_filter.rs");
        include!("types/compute/scale_set_packet_capture_machine_scope.rs");
        include!("types/compute/scale_set_packet_capture_storage_location.rs");
        include!("types/compute/scale_set_plan.rs");
        include!("types/compute/scale_set_rolling_upgrade_policy.rs");
        include!("types/compute/scale_set_sku.rs");
        include!("types/compute/scale_set_storage_profile_data_disk.rs");
        include!("types/compute/scale_set_storage_profile_image_reference.rs");
        include!("types/compute/scale_set_storage_profile_os_disk.rs");
        include!("types/compute/shared_image_gallery_sharing.rs");
        include!("types/compute/shared_image_gallery_sharing_community_gallery.rs");
        include!("types/compute/shared_image_identifier.rs");
        include!("types/compute/shared_image_purchase_plan.rs");
        include!("types/compute/shared_image_version_target_region.rs");
        include!("types/compute/snapshot_encryption_settings.rs");
        include!("types/compute/snapshot_encryption_settings_disk_encryption_key.rs");
        include!("types/compute/snapshot_encryption_settings_key_encryption_key.rs");
        include!("types/compute/virtual_machine_additional_capabilities.rs");
        include!("types/compute/virtual_machine_boot_diagnostics.rs");
        include!("types/compute/virtual_machine_identity.rs");
        include!("types/compute/virtual_machine_os_profile.rs");
        include!("types/compute/virtual_machine_os_profile_linux_config.rs");
        include!("types/compute/virtual_machine_os_profile_linux_config_ssh_key.rs");
        include!("types/compute/virtual_machine_os_profile_secret.rs");
        include!("types/compute/virtual_machine_os_profile_secret_vault_certificate.rs");
        include!("types/compute/virtual_machine_os_profile_windows_config.rs");
        include!(
            "types/compute/virtual_machine_os_profile_windows_config_additional_unattend_config.rs"
        );
        include!("types/compute/virtual_machine_os_profile_windows_config_winrm.rs");
        include!("types/compute/virtual_machine_plan.rs");
        include!(
            "types/compute/virtual_machine_scale_set_extension_protected_settings_from_key_vault.rs"
        );
        include!("types/compute/virtual_machine_storage_data_disk.rs");
        include!("types/compute/virtual_machine_storage_image_reference.rs");
        include!("types/compute/virtual_machine_storage_os_disk.rs");
        include!("types/compute/windows_virtual_machine_additional_capabilities.rs");
        include!("types/compute/windows_virtual_machine_additional_unattend_content.rs");
        include!("types/compute/windows_virtual_machine_boot_diagnostics.rs");
        include!("types/compute/windows_virtual_machine_gallery_application.rs");
        include!("types/compute/windows_virtual_machine_identity.rs");
        include!("types/compute/windows_virtual_machine_os_disk.rs");
        include!("types/compute/windows_virtual_machine_os_disk_diff_disk_settings.rs");
        include!("types/compute/windows_virtual_machine_os_image_notification.rs");
        include!("types/compute/windows_virtual_machine_plan.rs");
        include!(
            "types/compute/windows_virtual_machine_scale_set_additional_capabilities.rs"
        );
        include!(
            "types/compute/windows_virtual_machine_scale_set_additional_unattend_content.rs"
        );
        include!(
            "types/compute/windows_virtual_machine_scale_set_automatic_instance_repair.rs"
        );
        include!(
            "types/compute/windows_virtual_machine_scale_set_automatic_os_upgrade_policy.rs"
        );
        include!("types/compute/windows_virtual_machine_scale_set_boot_diagnostics.rs");
        include!("types/compute/windows_virtual_machine_scale_set_data_disk.rs");
        include!("types/compute/windows_virtual_machine_scale_set_extension.rs");
        include!(
            "types/compute/windows_virtual_machine_scale_set_extension_protected_settings_from_key_vault.rs"
        );
        include!(
            "types/compute/windows_virtual_machine_scale_set_gallery_application.rs"
        );
        include!("types/compute/windows_virtual_machine_scale_set_identity.rs");
        include!("types/compute/windows_virtual_machine_scale_set_network_interface.rs");
        include!(
            "types/compute/windows_virtual_machine_scale_set_network_interface_ip_configuration.rs"
        );
        include!(
            "types/compute/windows_virtual_machine_scale_set_network_interface_ip_configuration_public_ip_address.rs"
        );
        include!(
            "types/compute/windows_virtual_machine_scale_set_network_interface_ip_configuration_public_ip_address_ip_tag.rs"
        );
        include!("types/compute/windows_virtual_machine_scale_set_os_disk.rs");
        include!(
            "types/compute/windows_virtual_machine_scale_set_os_disk_diff_disk_settings.rs"
        );
        include!("types/compute/windows_virtual_machine_scale_set_plan.rs");
        include!(
            "types/compute/windows_virtual_machine_scale_set_rolling_upgrade_policy.rs"
        );
        include!("types/compute/windows_virtual_machine_scale_set_scale_in.rs");
        include!("types/compute/windows_virtual_machine_scale_set_secret.rs");
        include!(
            "types/compute/windows_virtual_machine_scale_set_secret_certificate.rs"
        );
        include!(
            "types/compute/windows_virtual_machine_scale_set_source_image_reference.rs"
        );
        include!("types/compute/windows_virtual_machine_scale_set_spot_restore.rs");
        include!(
            "types/compute/windows_virtual_machine_scale_set_termination_notification.rs"
        );
        include!("types/compute/windows_virtual_machine_scale_set_winrm_listener.rs");
        include!("types/compute/windows_virtual_machine_secret.rs");
        include!("types/compute/windows_virtual_machine_secret_certificate.rs");
        include!("types/compute/windows_virtual_machine_source_image_reference.rs");
        include!("types/compute/windows_virtual_machine_termination_notification.rs");
        include!("types/compute/windows_virtual_machine_winrm_listener.rs");
        include!("types/compute/get_bastion_host_ip_configuration.rs");
        include!(
            "types/compute/get_confidential_ledger_azuread_based_service_principal.rs"
        );
        include!(
            "types/compute/get_confidential_ledger_certificate_based_security_principal.rs"
        );
        include!("types/compute/get_disk_encryption_set_identity.rs");
        include!("types/compute/get_image_data_disk.rs");
        include!("types/compute/get_image_os_disk.rs");
        include!("types/compute/get_images_image.rs");
        include!("types/compute/get_images_image_data_disk.rs");
        include!("types/compute/get_images_image_os_disk.rs");
        include!("types/compute/get_managed_disk_encryption_setting.rs");
        include!(
            "types/compute/get_managed_disk_encryption_setting_disk_encryption_key.rs"
        );
        include!(
            "types/compute/get_managed_disk_encryption_setting_key_encryption_key.rs"
        );
        include!("types/compute/get_orchestrated_virtual_machine_scale_set_identity.rs");
        include!(
            "types/compute/get_orchestrated_virtual_machine_scale_set_network_interface.rs"
        );
        include!(
            "types/compute/get_orchestrated_virtual_machine_scale_set_network_interface_ip_configuration.rs"
        );
        include!(
            "types/compute/get_orchestrated_virtual_machine_scale_set_network_interface_ip_configuration_public_ip_address.rs"
        );
        include!(
            "types/compute/get_orchestrated_virtual_machine_scale_set_network_interface_ip_configuration_public_ip_address_ip_tag.rs"
        );
        include!("types/compute/get_shared_image_identifier.rs");
        include!("types/compute/get_shared_image_purchase_plan.rs");
        include!("types/compute/get_shared_image_version_target_region.rs");
        include!("types/compute/get_shared_image_versions_image.rs");
        include!("types/compute/get_shared_image_versions_image_target_region.rs");
        include!("types/compute/get_snapshot_encryption_setting.rs");
        include!("types/compute/get_snapshot_encryption_setting_disk_encryption_key.rs");
        include!("types/compute/get_snapshot_encryption_setting_key_encryption_key.rs");
        include!("types/compute/get_virtual_machine_identity.rs");
        include!("types/compute/get_virtual_machine_scale_set_identity.rs");
        include!("types/compute/get_virtual_machine_scale_set_instance.rs");
        include!("types/compute/get_virtual_machine_scale_set_network_interface.rs");
        include!(
            "types/compute/get_virtual_machine_scale_set_network_interface_ip_configuration.rs"
        );
        include!(
            "types/compute/get_virtual_machine_scale_set_network_interface_ip_configuration_public_ip_address.rs"
        );
        include!(
            "types/compute/get_virtual_machine_scale_set_network_interface_ip_configuration_public_ip_address_ip_tag.rs"
        );
    }
    pub mod confidentialledger {
        include!("types/confidentialledger/ledger_azuread_based_service_principal.rs");
        include!(
            "types/confidentialledger/ledger_certificate_based_security_principal.rs"
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
