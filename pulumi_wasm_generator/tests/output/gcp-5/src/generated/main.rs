pub mod deploymentmanager {
    include!("resources/deploymentmanager/deployment.rs");
}
pub mod developerconnect {
    include!("resources/developerconnect/connection.rs");
    include!("resources/developerconnect/git_repository_link.rs");
}
pub mod diagflow {
    include!("resources/diagflow/agent.rs");
    include!("resources/diagflow/cx_agent.rs");
    include!("resources/diagflow/cx_entity_type.rs");
    include!("resources/diagflow/cx_environment.rs");
    include!("resources/diagflow/cx_flow.rs");
    include!("resources/diagflow/cx_intent.rs");
    include!("resources/diagflow/cx_page.rs");
    include!("resources/diagflow/cx_security_settings.rs");
    include!("resources/diagflow/cx_test_case.rs");
    include!("resources/diagflow/cx_version.rs");
    include!("resources/diagflow/cx_webhook.rs");
    include!("resources/diagflow/entity_type.rs");
    include!("resources/diagflow/fulfillment.rs");
    include!("resources/diagflow/intent.rs");
}
pub mod discoveryengine {
    include!("resources/discoveryengine/chat_engine.rs");
    include!("resources/discoveryengine/data_store.rs");
    include!("resources/discoveryengine/schema.rs");
    include!("resources/discoveryengine/search_engine.rs");
    include!("resources/discoveryengine/target_site.rs");
}
pub mod dns {
    include!("resources/dns/dns_managed_zone_iam_binding.rs");
    include!("resources/dns/dns_managed_zone_iam_member.rs");
    include!("resources/dns/dns_managed_zone_iam_policy.rs");
    include!("resources/dns/managed_zone.rs");
    include!("resources/dns/policy.rs");
    include!("resources/dns/record_set.rs");
    include!("resources/dns/response_policy.rs");
    include!("resources/dns/response_policy_rule.rs");
}
pub mod edgecontainer {
    include!("resources/edgecontainer/cluster.rs");
    include!("resources/edgecontainer/node_pool.rs");
    include!("resources/edgecontainer/vpn_connection.rs");
}
pub mod edgenetwork {
    include!("resources/edgenetwork/network.rs");
    include!("resources/edgenetwork/subnet.rs");
}
pub mod endpoints {
    include!("resources/endpoints/consumers_iam_binding.rs");
    include!("resources/endpoints/consumers_iam_member.rs");
    include!("resources/endpoints/consumers_iam_policy.rs");
    include!("resources/endpoints/service.rs");
    include!("resources/endpoints/service_iam_binding.rs");
    include!("resources/endpoints/service_iam_member.rs");
    include!("resources/endpoints/service_iam_policy.rs");
}
pub mod essentialcontacts {
    include!("resources/essentialcontacts/contact.rs");
    include!("resources/essentialcontacts/document_ai_processor.rs");
    include!("resources/essentialcontacts/document_ai_processor_default_version.rs");
    include!("resources/essentialcontacts/document_ai_warehouse_document_schema.rs");
    include!("resources/essentialcontacts/document_ai_warehouse_location.rs");
}
pub mod eventarc {
    include!("resources/eventarc/channel.rs");
    include!("resources/eventarc/google_channel_config.rs");
    include!("resources/eventarc/trigger.rs");
}
pub mod functions {
    pub mod dns {
        include!("functions/dns/get_keys.rs");
        include!("functions/dns/get_managed_zone.rs");
        include!("functions/dns/get_managed_zone_iam_policy.rs");
        include!("functions/dns/get_managed_zones.rs");
        include!("functions/dns/get_record_set.rs");
    }
    pub mod endpoints {
        include!("functions/endpoints/get_service_consumers_iam_policy.rs");
        include!("functions/endpoints/get_service_iam_policy.rs");
    }
}
pub mod types {
    pub mod deploymentmanager {
        include!("types/deploymentmanager/deployment_label.rs");
        include!("types/deploymentmanager/deployment_target.rs");
        include!("types/deploymentmanager/deployment_target_config.rs");
        include!("types/deploymentmanager/deployment_target_import.rs");
    }
    pub mod developerconnect {
        include!("types/developerconnect/connection_github_config.rs");
        include!(
            "types/developerconnect/connection_github_config_authorizer_credential.rs"
        );
        include!("types/developerconnect/connection_installation_state.rs");
    }
    pub mod diagflow {
        include!("types/diagflow/cx_agent_advanced_settings.rs");
        include!(
            "types/diagflow/cx_agent_advanced_settings_audio_export_gcs_destination.rs"
        );
        include!("types/diagflow/cx_agent_advanced_settings_dtmf_settings.rs");
        include!("types/diagflow/cx_agent_advanced_settings_logging_settings.rs");
        include!("types/diagflow/cx_agent_advanced_settings_speech_settings.rs");
        include!("types/diagflow/cx_agent_git_integration_settings.rs");
        include!("types/diagflow/cx_agent_git_integration_settings_github_settings.rs");
        include!("types/diagflow/cx_agent_speech_to_text_settings.rs");
        include!("types/diagflow/cx_agent_text_to_speech_settings.rs");
        include!("types/diagflow/cx_entity_type_entity.rs");
        include!("types/diagflow/cx_entity_type_excluded_phrase.rs");
        include!("types/diagflow/cx_environment_version_config.rs");
        include!("types/diagflow/cx_flow_advanced_settings.rs");
        include!(
            "types/diagflow/cx_flow_advanced_settings_audio_export_gcs_destination.rs"
        );
        include!("types/diagflow/cx_flow_advanced_settings_dtmf_settings.rs");
        include!("types/diagflow/cx_flow_advanced_settings_logging_settings.rs");
        include!("types/diagflow/cx_flow_advanced_settings_speech_settings.rs");
        include!("types/diagflow/cx_flow_event_handler.rs");
        include!("types/diagflow/cx_flow_event_handler_trigger_fulfillment.rs");
        include!(
            "types/diagflow/cx_flow_event_handler_trigger_fulfillment_conditional_case.rs"
        );
        include!("types/diagflow/cx_flow_event_handler_trigger_fulfillment_message.rs");
        include!(
            "types/diagflow/cx_flow_event_handler_trigger_fulfillment_message_conversation_success.rs"
        );
        include!(
            "types/diagflow/cx_flow_event_handler_trigger_fulfillment_message_live_agent_handoff.rs"
        );
        include!(
            "types/diagflow/cx_flow_event_handler_trigger_fulfillment_message_output_audio_text.rs"
        );
        include!(
            "types/diagflow/cx_flow_event_handler_trigger_fulfillment_message_play_audio.rs"
        );
        include!(
            "types/diagflow/cx_flow_event_handler_trigger_fulfillment_message_telephony_transfer_call.rs"
        );
        include!(
            "types/diagflow/cx_flow_event_handler_trigger_fulfillment_message_text.rs"
        );
        include!(
            "types/diagflow/cx_flow_event_handler_trigger_fulfillment_set_parameter_action.rs"
        );
        include!("types/diagflow/cx_flow_nlu_settings.rs");
        include!("types/diagflow/cx_flow_transition_route.rs");
        include!("types/diagflow/cx_flow_transition_route_trigger_fulfillment.rs");
        include!(
            "types/diagflow/cx_flow_transition_route_trigger_fulfillment_conditional_case.rs"
        );
        include!(
            "types/diagflow/cx_flow_transition_route_trigger_fulfillment_message.rs"
        );
        include!(
            "types/diagflow/cx_flow_transition_route_trigger_fulfillment_message_conversation_success.rs"
        );
        include!(
            "types/diagflow/cx_flow_transition_route_trigger_fulfillment_message_live_agent_handoff.rs"
        );
        include!(
            "types/diagflow/cx_flow_transition_route_trigger_fulfillment_message_output_audio_text.rs"
        );
        include!(
            "types/diagflow/cx_flow_transition_route_trigger_fulfillment_message_play_audio.rs"
        );
        include!(
            "types/diagflow/cx_flow_transition_route_trigger_fulfillment_message_telephony_transfer_call.rs"
        );
        include!(
            "types/diagflow/cx_flow_transition_route_trigger_fulfillment_message_text.rs"
        );
        include!(
            "types/diagflow/cx_flow_transition_route_trigger_fulfillment_set_parameter_action.rs"
        );
        include!("types/diagflow/cx_intent_parameter.rs");
        include!("types/diagflow/cx_intent_training_phrase.rs");
        include!("types/diagflow/cx_intent_training_phrase_part.rs");
        include!("types/diagflow/cx_page_advanced_settings.rs");
        include!("types/diagflow/cx_page_advanced_settings_dtmf_settings.rs");
        include!("types/diagflow/cx_page_entry_fulfillment.rs");
        include!("types/diagflow/cx_page_entry_fulfillment_conditional_case.rs");
        include!("types/diagflow/cx_page_entry_fulfillment_message.rs");
        include!(
            "types/diagflow/cx_page_entry_fulfillment_message_conversation_success.rs"
        );
        include!(
            "types/diagflow/cx_page_entry_fulfillment_message_live_agent_handoff.rs"
        );
        include!(
            "types/diagflow/cx_page_entry_fulfillment_message_output_audio_text.rs"
        );
        include!("types/diagflow/cx_page_entry_fulfillment_message_play_audio.rs");
        include!(
            "types/diagflow/cx_page_entry_fulfillment_message_telephony_transfer_call.rs"
        );
        include!("types/diagflow/cx_page_entry_fulfillment_message_text.rs");
        include!("types/diagflow/cx_page_entry_fulfillment_set_parameter_action.rs");
        include!("types/diagflow/cx_page_event_handler.rs");
        include!("types/diagflow/cx_page_event_handler_trigger_fulfillment.rs");
        include!(
            "types/diagflow/cx_page_event_handler_trigger_fulfillment_conditional_case.rs"
        );
        include!("types/diagflow/cx_page_event_handler_trigger_fulfillment_message.rs");
        include!(
            "types/diagflow/cx_page_event_handler_trigger_fulfillment_message_conversation_success.rs"
        );
        include!(
            "types/diagflow/cx_page_event_handler_trigger_fulfillment_message_live_agent_handoff.rs"
        );
        include!(
            "types/diagflow/cx_page_event_handler_trigger_fulfillment_message_output_audio_text.rs"
        );
        include!(
            "types/diagflow/cx_page_event_handler_trigger_fulfillment_message_play_audio.rs"
        );
        include!(
            "types/diagflow/cx_page_event_handler_trigger_fulfillment_message_telephony_transfer_call.rs"
        );
        include!(
            "types/diagflow/cx_page_event_handler_trigger_fulfillment_message_text.rs"
        );
        include!(
            "types/diagflow/cx_page_event_handler_trigger_fulfillment_set_parameter_action.rs"
        );
        include!("types/diagflow/cx_page_form.rs");
        include!("types/diagflow/cx_page_form_parameter.rs");
        include!("types/diagflow/cx_page_form_parameter_advanced_settings.rs");
        include!(
            "types/diagflow/cx_page_form_parameter_advanced_settings_dtmf_settings.rs"
        );
        include!("types/diagflow/cx_page_form_parameter_fill_behavior.rs");
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_initial_prompt_fulfillment.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_initial_prompt_fulfillment_conditional_case.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_initial_prompt_fulfillment_message.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_initial_prompt_fulfillment_message_conversation_success.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_initial_prompt_fulfillment_message_live_agent_handoff.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_initial_prompt_fulfillment_message_output_audio_text.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_initial_prompt_fulfillment_message_play_audio.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_initial_prompt_fulfillment_message_telephony_transfer_call.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_initial_prompt_fulfillment_message_text.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_initial_prompt_fulfillment_set_parameter_action.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler_trigger_fulfillment.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler_trigger_fulfillment_conditional_case.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler_trigger_fulfillment_message.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler_trigger_fulfillment_message_conversation_success.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler_trigger_fulfillment_message_live_agent_handoff.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler_trigger_fulfillment_message_output_audio_text.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler_trigger_fulfillment_message_play_audio.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler_trigger_fulfillment_message_telephony_transfer_call.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler_trigger_fulfillment_message_text.rs"
        );
        include!(
            "types/diagflow/cx_page_form_parameter_fill_behavior_reprompt_event_handler_trigger_fulfillment_set_parameter_action.rs"
        );
        include!("types/diagflow/cx_page_transition_route.rs");
        include!("types/diagflow/cx_page_transition_route_trigger_fulfillment.rs");
        include!(
            "types/diagflow/cx_page_transition_route_trigger_fulfillment_conditional_case.rs"
        );
        include!(
            "types/diagflow/cx_page_transition_route_trigger_fulfillment_message.rs"
        );
        include!(
            "types/diagflow/cx_page_transition_route_trigger_fulfillment_message_conversation_success.rs"
        );
        include!(
            "types/diagflow/cx_page_transition_route_trigger_fulfillment_message_live_agent_handoff.rs"
        );
        include!(
            "types/diagflow/cx_page_transition_route_trigger_fulfillment_message_output_audio_text.rs"
        );
        include!(
            "types/diagflow/cx_page_transition_route_trigger_fulfillment_message_play_audio.rs"
        );
        include!(
            "types/diagflow/cx_page_transition_route_trigger_fulfillment_message_telephony_transfer_call.rs"
        );
        include!(
            "types/diagflow/cx_page_transition_route_trigger_fulfillment_message_text.rs"
        );
        include!(
            "types/diagflow/cx_page_transition_route_trigger_fulfillment_set_parameter_action.rs"
        );
        include!("types/diagflow/cx_security_settings_audio_export_settings.rs");
        include!("types/diagflow/cx_security_settings_insights_export_settings.rs");
        include!("types/diagflow/cx_test_case_last_test_result.rs");
        include!("types/diagflow/cx_test_case_last_test_result_conversation_turn.rs");
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_user_input.rs"
        );
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_user_input_input.rs"
        );
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_user_input_input_dtmf.rs"
        );
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_user_input_input_event.rs"
        );
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_user_input_input_text.rs"
        );
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_virtual_agent_output.rs"
        );
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_virtual_agent_output_current_page.rs"
        );
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_virtual_agent_output_difference.rs"
        );
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_virtual_agent_output_status.rs"
        );
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_virtual_agent_output_text_response.rs"
        );
        include!(
            "types/diagflow/cx_test_case_last_test_result_conversation_turn_virtual_agent_output_triggered_intent.rs"
        );
        include!("types/diagflow/cx_test_case_test_case_conversation_turn.rs");
        include!(
            "types/diagflow/cx_test_case_test_case_conversation_turn_user_input.rs"
        );
        include!(
            "types/diagflow/cx_test_case_test_case_conversation_turn_user_input_input.rs"
        );
        include!(
            "types/diagflow/cx_test_case_test_case_conversation_turn_user_input_input_dtmf.rs"
        );
        include!(
            "types/diagflow/cx_test_case_test_case_conversation_turn_user_input_input_event.rs"
        );
        include!(
            "types/diagflow/cx_test_case_test_case_conversation_turn_user_input_input_text.rs"
        );
        include!(
            "types/diagflow/cx_test_case_test_case_conversation_turn_virtual_agent_output.rs"
        );
        include!(
            "types/diagflow/cx_test_case_test_case_conversation_turn_virtual_agent_output_current_page.rs"
        );
        include!(
            "types/diagflow/cx_test_case_test_case_conversation_turn_virtual_agent_output_text_response.rs"
        );
        include!(
            "types/diagflow/cx_test_case_test_case_conversation_turn_virtual_agent_output_triggered_intent.rs"
        );
        include!("types/diagflow/cx_test_case_test_config.rs");
        include!("types/diagflow/cx_version_nlu_setting.rs");
        include!("types/diagflow/cx_webhook_generic_web_service.rs");
        include!("types/diagflow/cx_webhook_service_directory.rs");
        include!("types/diagflow/cx_webhook_service_directory_generic_web_service.rs");
        include!("types/diagflow/entity_type_entity.rs");
        include!("types/diagflow/fulfillment_feature.rs");
        include!("types/diagflow/fulfillment_generic_web_service.rs");
        include!("types/diagflow/intent_followup_intent_info.rs");
    }
    pub mod discoveryengine {
        include!("types/discoveryengine/chat_engine_chat_engine_config.rs");
        include!(
            "types/discoveryengine/chat_engine_chat_engine_config_agent_creation_config.rs"
        );
        include!("types/discoveryengine/chat_engine_chat_engine_metadata.rs");
        include!("types/discoveryengine/chat_engine_common_config.rs");
        include!("types/discoveryengine/data_store_document_processing_config.rs");
        include!(
            "types/discoveryengine/data_store_document_processing_config_chunking_config.rs"
        );
        include!(
            "types/discoveryengine/data_store_document_processing_config_chunking_config_layout_based_chunking_config.rs"
        );
        include!(
            "types/discoveryengine/data_store_document_processing_config_default_parsing_config.rs"
        );
        include!(
            "types/discoveryengine/data_store_document_processing_config_default_parsing_config_digital_parsing_config.rs"
        );
        include!(
            "types/discoveryengine/data_store_document_processing_config_default_parsing_config_layout_parsing_config.rs"
        );
        include!(
            "types/discoveryengine/data_store_document_processing_config_default_parsing_config_ocr_parsing_config.rs"
        );
        include!(
            "types/discoveryengine/data_store_document_processing_config_parsing_config_override.rs"
        );
        include!(
            "types/discoveryengine/data_store_document_processing_config_parsing_config_override_digital_parsing_config.rs"
        );
        include!(
            "types/discoveryengine/data_store_document_processing_config_parsing_config_override_layout_parsing_config.rs"
        );
        include!(
            "types/discoveryengine/data_store_document_processing_config_parsing_config_override_ocr_parsing_config.rs"
        );
        include!("types/discoveryengine/search_engine_common_config.rs");
        include!("types/discoveryengine/search_engine_search_engine_config.rs");
        include!("types/discoveryengine/target_site_failure_reason.rs");
        include!("types/discoveryengine/target_site_failure_reason_quota_failure.rs");
        include!("types/discoveryengine/target_site_site_verification_info.rs");
    }
    pub mod dns {
        include!("types/dns/dns_managed_zone_iam_binding_condition.rs");
        include!("types/dns/dns_managed_zone_iam_member_condition.rs");
        include!("types/dns/managed_zone_cloud_logging_config.rs");
        include!("types/dns/managed_zone_dnssec_config.rs");
        include!("types/dns/managed_zone_dnssec_config_default_key_spec.rs");
        include!("types/dns/managed_zone_forwarding_config.rs");
        include!("types/dns/managed_zone_forwarding_config_target_name_server.rs");
        include!("types/dns/managed_zone_peering_config.rs");
        include!("types/dns/managed_zone_peering_config_target_network.rs");
        include!("types/dns/managed_zone_private_visibility_config.rs");
        include!("types/dns/managed_zone_private_visibility_config_gke_cluster.rs");
        include!("types/dns/managed_zone_private_visibility_config_network.rs");
        include!("types/dns/managed_zone_service_directory_config.rs");
        include!("types/dns/managed_zone_service_directory_config_namespace.rs");
        include!("types/dns/policy_alternative_name_server_config.rs");
        include!(
            "types/dns/policy_alternative_name_server_config_target_name_server.rs"
        );
        include!("types/dns/policy_network.rs");
        include!("types/dns/record_set_routing_policy.rs");
        include!("types/dns/record_set_routing_policy_geo.rs");
        include!("types/dns/record_set_routing_policy_geo_health_checked_targets.rs");
        include!(
            "types/dns/record_set_routing_policy_geo_health_checked_targets_internal_load_balancer.rs"
        );
        include!("types/dns/record_set_routing_policy_primary_backup.rs");
        include!("types/dns/record_set_routing_policy_primary_backup_backup_geo.rs");
        include!(
            "types/dns/record_set_routing_policy_primary_backup_backup_geo_health_checked_targets.rs"
        );
        include!(
            "types/dns/record_set_routing_policy_primary_backup_backup_geo_health_checked_targets_internal_load_balancer.rs"
        );
        include!("types/dns/record_set_routing_policy_primary_backup_primary.rs");
        include!(
            "types/dns/record_set_routing_policy_primary_backup_primary_internal_load_balancer.rs"
        );
        include!("types/dns/record_set_routing_policy_wrr.rs");
        include!("types/dns/record_set_routing_policy_wrr_health_checked_targets.rs");
        include!(
            "types/dns/record_set_routing_policy_wrr_health_checked_targets_internal_load_balancer.rs"
        );
        include!("types/dns/response_policy_gke_cluster.rs");
        include!("types/dns/response_policy_network.rs");
        include!("types/dns/response_policy_rule_local_data.rs");
        include!("types/dns/response_policy_rule_local_data_local_data.rs");
        include!("types/dns/get_keys_key_signing_key.rs");
        include!("types/dns/get_keys_key_signing_key_digest.rs");
        include!("types/dns/get_keys_zone_signing_key.rs");
        include!("types/dns/get_keys_zone_signing_key_digest.rs");
        include!("types/dns/get_managed_zones_managed_zone.rs");
    }
    pub mod edgecontainer {
        include!("types/edgecontainer/cluster_authorization.rs");
        include!("types/edgecontainer/cluster_authorization_admin_users.rs");
        include!("types/edgecontainer/cluster_control_plane.rs");
        include!("types/edgecontainer/cluster_control_plane_encryption.rs");
        include!("types/edgecontainer/cluster_control_plane_encryption_kms_status.rs");
        include!("types/edgecontainer/cluster_control_plane_local.rs");
        include!("types/edgecontainer/cluster_control_plane_remote.rs");
        include!("types/edgecontainer/cluster_fleet.rs");
        include!("types/edgecontainer/cluster_maintenance_event.rs");
        include!("types/edgecontainer/cluster_maintenance_policy.rs");
        include!(
            "types/edgecontainer/cluster_maintenance_policy_maintenance_exclusion.rs"
        );
        include!(
            "types/edgecontainer/cluster_maintenance_policy_maintenance_exclusion_window.rs"
        );
        include!("types/edgecontainer/cluster_maintenance_policy_window.rs");
        include!(
            "types/edgecontainer/cluster_maintenance_policy_window_recurring_window.rs"
        );
        include!(
            "types/edgecontainer/cluster_maintenance_policy_window_recurring_window_window.rs"
        );
        include!("types/edgecontainer/cluster_networking.rs");
        include!("types/edgecontainer/cluster_system_addons_config.rs");
        include!("types/edgecontainer/cluster_system_addons_config_ingress.rs");
        include!("types/edgecontainer/node_pool_local_disk_encryption.rs");
        include!("types/edgecontainer/node_pool_node_config.rs");
        include!("types/edgecontainer/vpn_connection_detail.rs");
        include!("types/edgecontainer/vpn_connection_detail_cloud_router.rs");
        include!("types/edgecontainer/vpn_connection_detail_cloud_vpn.rs");
        include!("types/edgecontainer/vpn_connection_vpc_project.rs");
    }
    pub mod endpoints {
        include!("types/endpoints/consumers_iam_binding_condition.rs");
        include!("types/endpoints/consumers_iam_member_condition.rs");
        include!("types/endpoints/service_api.rs");
        include!("types/endpoints/service_api_method.rs");
        include!("types/endpoints/service_endpoint.rs");
        include!("types/endpoints/service_iam_binding_condition.rs");
        include!("types/endpoints/service_iam_member_condition.rs");
    }
    pub mod essentialcontacts {
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_date_time_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_enum_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_float_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_integer_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_map_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_property_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_property_type_options_property_definition.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_property_type_options_property_definition_date_time_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_property_type_options_property_definition_enum_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_property_type_options_property_definition_float_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_property_type_options_property_definition_integer_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_property_type_options_property_definition_map_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_property_type_options_property_definition_schema_source.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_property_type_options_property_definition_text_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_property_type_options_property_definition_timestamp_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_schema_source.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_text_type_options.rs"
        );
        include!(
            "types/essentialcontacts/document_ai_warehouse_document_schema_property_definition_timestamp_type_options.rs"
        );
    }
    pub mod eventarc {
        include!("types/eventarc/trigger_destination.rs");
        include!("types/eventarc/trigger_destination_cloud_run_service.rs");
        include!("types/eventarc/trigger_destination_gke.rs");
        include!("types/eventarc/trigger_destination_http_endpoint.rs");
        include!("types/eventarc/trigger_destination_network_config.rs");
        include!("types/eventarc/trigger_matching_criteria.rs");
        include!("types/eventarc/trigger_transport.rs");
        include!("types/eventarc/trigger_transport_pubsub.rs");
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
}

interface register-interface {
    use pulumi-engine.{engine};
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
        version: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record register-resource-result {
        fields: list<register-resource-result-field>
    }

    register: func(engine: borrow<engine>, request: register-resource-request) -> register-resource-result;

    record resource-invoke-result-field {
        name: string,
        output: output
    }

    record resource-invoke-request {
        token: string,
        version: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record resource-invoke-result {
        fields: list<resource-invoke-result-field>
    }

    invoke: func(engine: borrow<engine>, request: resource-invoke-request) -> resource-invoke-result;
}",
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        } }
    );
}
#[link_section = "pulumi_wasm_provider::gcp"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_GCP: [u8; 45] = *b"{\"version\":\"8.12.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "8.12.1".to_string()
}
