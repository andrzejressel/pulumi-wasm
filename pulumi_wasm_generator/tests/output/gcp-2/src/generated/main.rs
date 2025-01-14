pub mod blockchainnodeengine {
    include!("resources/blockchainnodeengine/blockchain_nodes.rs");
}
pub mod certificateauthority {
    include!("resources/certificateauthority/authority.rs");
    include!("resources/certificateauthority/ca_pool.rs");
    include!("resources/certificateauthority/ca_pool_iam_binding.rs");
    include!("resources/certificateauthority/ca_pool_iam_member.rs");
    include!("resources/certificateauthority/ca_pool_iam_policy.rs");
    include!("resources/certificateauthority/certificate.rs");
    include!("resources/certificateauthority/certificate_template.rs");
    include!("resources/certificateauthority/certificate_template_iam_binding.rs");
    include!("resources/certificateauthority/certificate_template_iam_member.rs");
    include!("resources/certificateauthority/certificate_template_iam_policy.rs");
}
pub mod certificatemanager {
    include!("resources/certificatemanager/certificate.rs");
    include!("resources/certificatemanager/certificate_issuance_config.rs");
    include!("resources/certificatemanager/certificate_map.rs");
    include!("resources/certificatemanager/certificate_map_entry.rs");
    include!("resources/certificatemanager/dns_authorization.rs");
    include!("resources/certificatemanager/trust_config.rs");
}
pub mod cloudasset {
    include!("resources/cloudasset/folder_feed.rs");
    include!("resources/cloudasset/organization_feed.rs");
    include!("resources/cloudasset/project_feed.rs");
}
pub mod cloudbuild {
    include!("resources/cloudbuild/bitbucket_server_config.rs");
    include!("resources/cloudbuild/trigger.rs");
    include!("resources/cloudbuild/worker_pool.rs");
}
pub mod cloudbuildv2 {
    include!("resources/cloudbuildv2/connection.rs");
    include!("resources/cloudbuildv2/connection_iam_binding.rs");
    include!("resources/cloudbuildv2/connection_iam_member.rs");
    include!("resources/cloudbuildv2/connection_iam_policy.rs");
    include!("resources/cloudbuildv2/repository.rs");
}
pub mod clouddeploy {
    include!("resources/clouddeploy/automation.rs");
    include!("resources/clouddeploy/custom_target_type.rs");
    include!("resources/clouddeploy/custom_target_type_iam_binding.rs");
    include!("resources/clouddeploy/custom_target_type_iam_member.rs");
    include!("resources/clouddeploy/custom_target_type_iam_policy.rs");
    include!("resources/clouddeploy/delivery_pipeline.rs");
    include!("resources/clouddeploy/delivery_pipeline_iam_binding.rs");
    include!("resources/clouddeploy/delivery_pipeline_iam_member.rs");
    include!("resources/clouddeploy/delivery_pipeline_iam_policy.rs");
    include!("resources/clouddeploy/target.rs");
    include!("resources/clouddeploy/target_iam_binding.rs");
    include!("resources/clouddeploy/target_iam_member.rs");
    include!("resources/clouddeploy/target_iam_policy.rs");
}
pub mod clouddomains {
    include!("resources/clouddomains/registration.rs");
}
pub mod cloudfunctions {
    include!("resources/cloudfunctions/function.rs");
    include!("resources/cloudfunctions/function_iam_binding.rs");
    include!("resources/cloudfunctions/function_iam_member.rs");
    include!("resources/cloudfunctions/function_iam_policy.rs");
}
pub mod cloudfunctionsv2 {
    include!("resources/cloudfunctionsv2/function.rs");
    include!("resources/cloudfunctionsv2/function_iam_binding.rs");
    include!("resources/cloudfunctionsv2/function_iam_member.rs");
    include!("resources/cloudfunctionsv2/function_iam_policy.rs");
}
pub mod functions {
    pub mod certificateauthority {
        include!("functions/certificateauthority/get_authority.rs");
        include!("functions/certificateauthority/get_ca_pool_iam_policy.rs");
        include!(
            "functions/certificateauthority/get_certificate_template_iam_policy.rs"
        );
    }
    pub mod certificatemanager {
        include!("functions/certificatemanager/get_certificate_map.rs");
        include!("functions/certificatemanager/get_certificates.rs");
    }
    pub mod cloudasset {
        include!("functions/cloudasset/get_resources_search_all.rs");
        include!("functions/cloudasset/get_search_all_resources.rs");
    }
    pub mod cloudbuild {
        include!("functions/cloudbuild/get_trigger.rs");
    }
    pub mod cloudbuildv2 {
        include!("functions/cloudbuildv2/get_connection_iam_policy.rs");
    }
    pub mod clouddeploy {
        include!("functions/clouddeploy/get_custom_target_type_iam_policy.rs");
        include!("functions/clouddeploy/get_delivery_pipeline_iam_policy.rs");
        include!("functions/clouddeploy/get_target_iam_policy.rs");
    }
    pub mod cloudfunctions {
        include!("functions/cloudfunctions/get_function.rs");
        include!("functions/cloudfunctions/get_function_iam_policy.rs");
    }
    pub mod cloudfunctionsv2 {
        include!("functions/cloudfunctionsv2/get_function.rs");
        include!("functions/cloudfunctionsv2/get_function_iam_policy.rs");
    }
}
pub mod types {
    pub mod blockchainnodeengine {
        include!("types/blockchainnodeengine/blockchain_nodes_connection_info.rs");
        include!(
            "types/blockchainnodeengine/blockchain_nodes_connection_info_endpoint_info.rs"
        );
        include!("types/blockchainnodeengine/blockchain_nodes_ethereum_details.rs");
        include!(
            "types/blockchainnodeengine/blockchain_nodes_ethereum_details_additional_endpoint.rs"
        );
        include!(
            "types/blockchainnodeengine/blockchain_nodes_ethereum_details_geth_details.rs"
        );
        include!(
            "types/blockchainnodeengine/blockchain_nodes_ethereum_details_validator_config.rs"
        );
    }
    pub mod certificateauthority {
        include!("types/certificateauthority/authority_access_url.rs");
        include!("types/certificateauthority/authority_config.rs");
        include!("types/certificateauthority/authority_config_subject_config.rs");
        include!(
            "types/certificateauthority/authority_config_subject_config_subject.rs"
        );
        include!(
            "types/certificateauthority/authority_config_subject_config_subject_alt_name.rs"
        );
        include!("types/certificateauthority/authority_config_subject_key_id.rs");
        include!("types/certificateauthority/authority_config_x_509_config.rs");
        include!(
            "types/certificateauthority/authority_config_x_509_config_additional_extension.rs"
        );
        include!(
            "types/certificateauthority/authority_config_x_509_config_additional_extension_object_id.rs"
        );
        include!(
            "types/certificateauthority/authority_config_x_509_config_ca_options.rs"
        );
        include!(
            "types/certificateauthority/authority_config_x_509_config_key_usage.rs"
        );
        include!(
            "types/certificateauthority/authority_config_x_509_config_key_usage_base_key_usage.rs"
        );
        include!(
            "types/certificateauthority/authority_config_x_509_config_key_usage_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/authority_config_x_509_config_key_usage_unknown_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/authority_config_x_509_config_name_constraints.rs"
        );
        include!(
            "types/certificateauthority/authority_config_x_509_config_policy_id.rs"
        );
        include!("types/certificateauthority/authority_key_spec.rs");
        include!("types/certificateauthority/authority_subordinate_config.rs");
        include!(
            "types/certificateauthority/authority_subordinate_config_pem_issuer_chain.rs"
        );
        include!("types/certificateauthority/ca_pool_iam_binding_condition.rs");
        include!("types/certificateauthority/ca_pool_iam_member_condition.rs");
        include!("types/certificateauthority/ca_pool_issuance_policy.rs");
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_allowed_issuance_modes.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_allowed_key_type.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_allowed_key_type_elliptic_curve.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_allowed_key_type_rsa.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_baseline_values.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_baseline_values_additional_extension.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_baseline_values_additional_extension_object_id.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_baseline_values_ca_options.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_baseline_values_key_usage.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_baseline_values_key_usage_base_key_usage.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_baseline_values_key_usage_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_baseline_values_key_usage_unknown_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_baseline_values_name_constraints.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_baseline_values_policy_id.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_identity_constraints.rs"
        );
        include!(
            "types/certificateauthority/ca_pool_issuance_policy_identity_constraints_cel_expression.rs"
        );
        include!("types/certificateauthority/ca_pool_publishing_options.rs");
        include!("types/certificateauthority/certificate_certificate_description.rs");
        include!(
            "types/certificateauthority/certificate_certificate_description_authority_key_id.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_cert_fingerprint.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_public_key.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_subject_description.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_subject_description_subject.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_subject_description_subject_alt_name.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_subject_description_subject_alt_name_custom_san.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_subject_description_subject_alt_name_custom_san_obect_id.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_subject_key_id.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_x_509_description.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_x_509_description_additional_extension.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_x_509_description_additional_extension_object_id.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_x_509_description_ca_option.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_x_509_description_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_x_509_description_key_usage_base_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_x_509_description_key_usage_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_x_509_description_key_usage_unknown_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_x_509_description_name_constraint.rs"
        );
        include!(
            "types/certificateauthority/certificate_certificate_description_x_509_description_policy_id.rs"
        );
        include!("types/certificateauthority/certificate_config.rs");
        include!("types/certificateauthority/certificate_config_public_key.rs");
        include!("types/certificateauthority/certificate_config_subject_config.rs");
        include!(
            "types/certificateauthority/certificate_config_subject_config_subject.rs"
        );
        include!(
            "types/certificateauthority/certificate_config_subject_config_subject_alt_name.rs"
        );
        include!("types/certificateauthority/certificate_config_subject_key_id.rs");
        include!("types/certificateauthority/certificate_config_x_509_config.rs");
        include!(
            "types/certificateauthority/certificate_config_x_509_config_additional_extension.rs"
        );
        include!(
            "types/certificateauthority/certificate_config_x_509_config_additional_extension_object_id.rs"
        );
        include!(
            "types/certificateauthority/certificate_config_x_509_config_ca_options.rs"
        );
        include!(
            "types/certificateauthority/certificate_config_x_509_config_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_config_x_509_config_key_usage_base_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_config_x_509_config_key_usage_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_config_x_509_config_key_usage_unknown_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_config_x_509_config_name_constraints.rs"
        );
        include!(
            "types/certificateauthority/certificate_config_x_509_config_policy_id.rs"
        );
        include!("types/certificateauthority/certificate_revocation_detail.rs");
        include!(
            "types/certificateauthority/certificate_template_iam_binding_condition.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_iam_member_condition.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_identity_constraints.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_identity_constraints_cel_expression.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_passthrough_extensions.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_passthrough_extensions_additional_extension.rs"
        );
        include!("types/certificateauthority/certificate_template_predefined_values.rs");
        include!(
            "types/certificateauthority/certificate_template_predefined_values_additional_extension.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_predefined_values_additional_extension_object_id.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_predefined_values_ca_options.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_predefined_values_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_predefined_values_key_usage_base_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_predefined_values_key_usage_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_predefined_values_key_usage_unknown_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/certificate_template_predefined_values_policy_id.rs"
        );
        include!("types/certificateauthority/get_authority_access_url.rs");
        include!("types/certificateauthority/get_authority_config.rs");
        include!("types/certificateauthority/get_authority_config_subject_config.rs");
        include!(
            "types/certificateauthority/get_authority_config_subject_config_subject.rs"
        );
        include!(
            "types/certificateauthority/get_authority_config_subject_config_subject_alt_name.rs"
        );
        include!("types/certificateauthority/get_authority_config_subject_key_id.rs");
        include!("types/certificateauthority/get_authority_config_x_509_config.rs");
        include!(
            "types/certificateauthority/get_authority_config_x_509_config_additional_extension.rs"
        );
        include!(
            "types/certificateauthority/get_authority_config_x_509_config_additional_extension_object_id.rs"
        );
        include!(
            "types/certificateauthority/get_authority_config_x_509_config_ca_option.rs"
        );
        include!(
            "types/certificateauthority/get_authority_config_x_509_config_key_usage.rs"
        );
        include!(
            "types/certificateauthority/get_authority_config_x_509_config_key_usage_base_key_usage.rs"
        );
        include!(
            "types/certificateauthority/get_authority_config_x_509_config_key_usage_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/get_authority_config_x_509_config_key_usage_unknown_extended_key_usage.rs"
        );
        include!(
            "types/certificateauthority/get_authority_config_x_509_config_name_constraint.rs"
        );
        include!(
            "types/certificateauthority/get_authority_config_x_509_config_policy_id.rs"
        );
        include!("types/certificateauthority/get_authority_key_spec.rs");
        include!("types/certificateauthority/get_authority_subordinate_config.rs");
        include!(
            "types/certificateauthority/get_authority_subordinate_config_pem_issuer_chain.rs"
        );
    }
    pub mod certificatemanager {
        include!(
            "types/certificatemanager/certificate_issuance_config_certificate_authority_config.rs"
        );
        include!(
            "types/certificatemanager/certificate_issuance_config_certificate_authority_config_certificate_authority_service_config.rs"
        );
        include!("types/certificatemanager/certificate_managed.rs");
        include!(
            "types/certificatemanager/certificate_managed_authorization_attempt_info.rs"
        );
        include!("types/certificatemanager/certificate_managed_provisioning_issue.rs");
        include!("types/certificatemanager/certificate_map_gclb_target.rs");
        include!("types/certificatemanager/certificate_map_gclb_target_ip_config.rs");
        include!("types/certificatemanager/certificate_self_managed.rs");
        include!("types/certificatemanager/dns_authorization_dns_resource_record.rs");
        include!("types/certificatemanager/trust_config_allowlisted_certificate.rs");
        include!("types/certificatemanager/trust_config_trust_store.rs");
        include!("types/certificatemanager/trust_config_trust_store_intermediate_ca.rs");
        include!("types/certificatemanager/trust_config_trust_store_trust_anchor.rs");
        include!("types/certificatemanager/get_certificate_map_gclb_target.rs");
        include!(
            "types/certificatemanager/get_certificate_map_gclb_target_ip_config.rs"
        );
        include!("types/certificatemanager/get_certificates_certificate.rs");
        include!("types/certificatemanager/get_certificates_certificate_managed.rs");
        include!(
            "types/certificatemanager/get_certificates_certificate_managed_authorization_attempt_info.rs"
        );
        include!(
            "types/certificatemanager/get_certificates_certificate_managed_provisioning_issue.rs"
        );
    }
    pub mod cloudasset {
        include!("types/cloudasset/folder_feed_condition.rs");
        include!("types/cloudasset/folder_feed_feed_output_config.rs");
        include!(
            "types/cloudasset/folder_feed_feed_output_config_pubsub_destination.rs"
        );
        include!("types/cloudasset/organization_feed_condition.rs");
        include!("types/cloudasset/organization_feed_feed_output_config.rs");
        include!(
            "types/cloudasset/organization_feed_feed_output_config_pubsub_destination.rs"
        );
        include!("types/cloudasset/project_feed_condition.rs");
        include!("types/cloudasset/project_feed_feed_output_config.rs");
        include!(
            "types/cloudasset/project_feed_feed_output_config_pubsub_destination.rs"
        );
        include!("types/cloudasset/get_resources_search_all_result.rs");
        include!("types/cloudasset/get_search_all_resources_result.rs");
    }
    pub mod cloudbuild {
        include!("types/cloudbuild/bitbucket_server_config_connected_repository.rs");
        include!("types/cloudbuild/bitbucket_server_config_secrets.rs");
        include!("types/cloudbuild/trigger_approval_config.rs");
        include!("types/cloudbuild/trigger_bitbucket_server_trigger_config.rs");
        include!(
            "types/cloudbuild/trigger_bitbucket_server_trigger_config_pull_request.rs"
        );
        include!("types/cloudbuild/trigger_bitbucket_server_trigger_config_push.rs");
        include!("types/cloudbuild/trigger_build.rs");
        include!("types/cloudbuild/trigger_build_artifacts.rs");
        include!("types/cloudbuild/trigger_build_artifacts_maven_artifact.rs");
        include!("types/cloudbuild/trigger_build_artifacts_npm_package.rs");
        include!("types/cloudbuild/trigger_build_artifacts_objects.rs");
        include!("types/cloudbuild/trigger_build_artifacts_objects_timing.rs");
        include!("types/cloudbuild/trigger_build_artifacts_python_package.rs");
        include!("types/cloudbuild/trigger_build_available_secrets.rs");
        include!("types/cloudbuild/trigger_build_available_secrets_secret_manager.rs");
        include!("types/cloudbuild/trigger_build_options.rs");
        include!("types/cloudbuild/trigger_build_options_volume.rs");
        include!("types/cloudbuild/trigger_build_secret.rs");
        include!("types/cloudbuild/trigger_build_source.rs");
        include!("types/cloudbuild/trigger_build_source_repo_source.rs");
        include!("types/cloudbuild/trigger_build_source_storage_source.rs");
        include!("types/cloudbuild/trigger_build_step.rs");
        include!("types/cloudbuild/trigger_build_step_volume.rs");
        include!("types/cloudbuild/trigger_git_file_source.rs");
        include!("types/cloudbuild/trigger_github.rs");
        include!("types/cloudbuild/trigger_github_pull_request.rs");
        include!("types/cloudbuild/trigger_github_push.rs");
        include!("types/cloudbuild/trigger_pubsub_config.rs");
        include!("types/cloudbuild/trigger_repository_event_config.rs");
        include!("types/cloudbuild/trigger_repository_event_config_pull_request.rs");
        include!("types/cloudbuild/trigger_repository_event_config_push.rs");
        include!("types/cloudbuild/trigger_source_to_build.rs");
        include!("types/cloudbuild/trigger_trigger_template.rs");
        include!("types/cloudbuild/trigger_webhook_config.rs");
        include!("types/cloudbuild/worker_pool_network_config.rs");
        include!("types/cloudbuild/worker_pool_private_service_connect.rs");
        include!("types/cloudbuild/worker_pool_worker_config.rs");
        include!("types/cloudbuild/get_trigger_approval_config.rs");
        include!("types/cloudbuild/get_trigger_bitbucket_server_trigger_config.rs");
        include!(
            "types/cloudbuild/get_trigger_bitbucket_server_trigger_config_pull_request.rs"
        );
        include!("types/cloudbuild/get_trigger_bitbucket_server_trigger_config_push.rs");
        include!("types/cloudbuild/get_trigger_build.rs");
        include!("types/cloudbuild/get_trigger_build_artifact.rs");
        include!("types/cloudbuild/get_trigger_build_artifact_maven_artifact.rs");
        include!("types/cloudbuild/get_trigger_build_artifact_npm_package.rs");
        include!("types/cloudbuild/get_trigger_build_artifact_object.rs");
        include!("types/cloudbuild/get_trigger_build_artifact_object_timing.rs");
        include!("types/cloudbuild/get_trigger_build_artifact_python_package.rs");
        include!("types/cloudbuild/get_trigger_build_available_secret.rs");
        include!(
            "types/cloudbuild/get_trigger_build_available_secret_secret_manager.rs"
        );
        include!("types/cloudbuild/get_trigger_build_option.rs");
        include!("types/cloudbuild/get_trigger_build_option_volume.rs");
        include!("types/cloudbuild/get_trigger_build_secret.rs");
        include!("types/cloudbuild/get_trigger_build_source.rs");
        include!("types/cloudbuild/get_trigger_build_source_repo_source.rs");
        include!("types/cloudbuild/get_trigger_build_source_storage_source.rs");
        include!("types/cloudbuild/get_trigger_build_step.rs");
        include!("types/cloudbuild/get_trigger_build_step_volume.rs");
        include!("types/cloudbuild/get_trigger_git_file_source.rs");
        include!("types/cloudbuild/get_trigger_github.rs");
        include!("types/cloudbuild/get_trigger_github_pull_request.rs");
        include!("types/cloudbuild/get_trigger_github_push.rs");
        include!("types/cloudbuild/get_trigger_pubsub_config.rs");
        include!("types/cloudbuild/get_trigger_repository_event_config.rs");
        include!("types/cloudbuild/get_trigger_repository_event_config_pull_request.rs");
        include!("types/cloudbuild/get_trigger_repository_event_config_push.rs");
        include!("types/cloudbuild/get_trigger_source_to_build.rs");
        include!("types/cloudbuild/get_trigger_trigger_template.rs");
        include!("types/cloudbuild/get_trigger_webhook_config.rs");
    }
    pub mod cloudbuildv2 {
        include!("types/cloudbuildv2/connection_bitbucket_cloud_config.rs");
        include!(
            "types/cloudbuildv2/connection_bitbucket_cloud_config_authorizer_credential.rs"
        );
        include!(
            "types/cloudbuildv2/connection_bitbucket_cloud_config_read_authorizer_credential.rs"
        );
        include!("types/cloudbuildv2/connection_bitbucket_data_center_config.rs");
        include!(
            "types/cloudbuildv2/connection_bitbucket_data_center_config_authorizer_credential.rs"
        );
        include!(
            "types/cloudbuildv2/connection_bitbucket_data_center_config_read_authorizer_credential.rs"
        );
        include!(
            "types/cloudbuildv2/connection_bitbucket_data_center_config_service_directory_config.rs"
        );
        include!("types/cloudbuildv2/connection_github_config.rs");
        include!("types/cloudbuildv2/connection_github_config_authorizer_credential.rs");
        include!("types/cloudbuildv2/connection_github_enterprise_config.rs");
        include!(
            "types/cloudbuildv2/connection_github_enterprise_config_service_directory_config.rs"
        );
        include!("types/cloudbuildv2/connection_gitlab_config.rs");
        include!("types/cloudbuildv2/connection_gitlab_config_authorizer_credential.rs");
        include!(
            "types/cloudbuildv2/connection_gitlab_config_read_authorizer_credential.rs"
        );
        include!(
            "types/cloudbuildv2/connection_gitlab_config_service_directory_config.rs"
        );
        include!("types/cloudbuildv2/connection_iam_binding_condition.rs");
        include!("types/cloudbuildv2/connection_iam_member_condition.rs");
        include!("types/cloudbuildv2/connection_installation_state.rs");
    }
    pub mod clouddeploy {
        include!("types/clouddeploy/automation_rule.rs");
        include!("types/clouddeploy/automation_rule_advance_rollout_rule.rs");
        include!("types/clouddeploy/automation_rule_promote_release_rule.rs");
        include!("types/clouddeploy/automation_selector.rs");
        include!("types/clouddeploy/automation_selector_target.rs");
        include!("types/clouddeploy/custom_target_type_custom_actions.rs");
        include!(
            "types/clouddeploy/custom_target_type_custom_actions_include_skaffold_module.rs"
        );
        include!(
            "types/clouddeploy/custom_target_type_custom_actions_include_skaffold_module_git.rs"
        );
        include!(
            "types/clouddeploy/custom_target_type_custom_actions_include_skaffold_module_google_cloud_build_repo.rs"
        );
        include!(
            "types/clouddeploy/custom_target_type_custom_actions_include_skaffold_module_google_cloud_storage.rs"
        );
        include!("types/clouddeploy/custom_target_type_iam_binding_condition.rs");
        include!("types/clouddeploy/custom_target_type_iam_member_condition.rs");
        include!("types/clouddeploy/delivery_pipeline_condition.rs");
        include!(
            "types/clouddeploy/delivery_pipeline_condition_pipeline_ready_condition.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_condition_targets_present_condition.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_condition_targets_type_condition.rs"
        );
        include!("types/clouddeploy/delivery_pipeline_iam_binding_condition.rs");
        include!("types/clouddeploy/delivery_pipeline_iam_member_condition.rs");
        include!("types/clouddeploy/delivery_pipeline_serial_pipeline.rs");
        include!("types/clouddeploy/delivery_pipeline_serial_pipeline_stage.rs");
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_deploy_parameter.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_canary_deployment.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_canary_deployment_postdeploy.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_canary_deployment_predeploy.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_custom_canary_deployment.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_custom_canary_deployment_phase_config.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_custom_canary_deployment_phase_config_postdeploy.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_custom_canary_deployment_phase_config_predeploy.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_runtime_config.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_runtime_config_cloud_run.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_runtime_config_kubernetes.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_runtime_config_kubernetes_gateway_service_mesh.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_runtime_config_kubernetes_gateway_service_mesh_route_destinations.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_canary_runtime_config_kubernetes_service_networking.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_standard.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_standard_postdeploy.rs"
        );
        include!(
            "types/clouddeploy/delivery_pipeline_serial_pipeline_stage_strategy_standard_predeploy.rs"
        );
        include!("types/clouddeploy/target_anthos_cluster.rs");
        include!("types/clouddeploy/target_associated_entity.rs");
        include!("types/clouddeploy/target_associated_entity_anthos_cluster.rs");
        include!("types/clouddeploy/target_associated_entity_gke_cluster.rs");
        include!("types/clouddeploy/target_custom_target.rs");
        include!("types/clouddeploy/target_execution_config.rs");
        include!("types/clouddeploy/target_gke.rs");
        include!("types/clouddeploy/target_iam_binding_condition.rs");
        include!("types/clouddeploy/target_iam_member_condition.rs");
        include!("types/clouddeploy/target_multi_target.rs");
        include!("types/clouddeploy/target_run.rs");
    }
    pub mod clouddomains {
        include!("types/clouddomains/registration_contact_settings.rs");
        include!("types/clouddomains/registration_contact_settings_admin_contact.rs");
        include!(
            "types/clouddomains/registration_contact_settings_admin_contact_postal_address.rs"
        );
        include!(
            "types/clouddomains/registration_contact_settings_registrant_contact.rs"
        );
        include!(
            "types/clouddomains/registration_contact_settings_registrant_contact_postal_address.rs"
        );
        include!(
            "types/clouddomains/registration_contact_settings_technical_contact.rs"
        );
        include!(
            "types/clouddomains/registration_contact_settings_technical_contact_postal_address.rs"
        );
        include!("types/clouddomains/registration_dns_settings.rs");
        include!("types/clouddomains/registration_dns_settings_custom_dns.rs");
        include!("types/clouddomains/registration_dns_settings_custom_dns_ds_record.rs");
        include!("types/clouddomains/registration_dns_settings_glue_record.rs");
        include!("types/clouddomains/registration_management_settings.rs");
        include!("types/clouddomains/registration_yearly_price.rs");
    }
    pub mod cloudfunctions {
        include!("types/cloudfunctions/function_event_trigger.rs");
        include!("types/cloudfunctions/function_event_trigger_failure_policy.rs");
        include!("types/cloudfunctions/function_iam_binding_condition.rs");
        include!("types/cloudfunctions/function_iam_member_condition.rs");
        include!("types/cloudfunctions/function_secret_environment_variable.rs");
        include!("types/cloudfunctions/function_secret_volume.rs");
        include!("types/cloudfunctions/function_secret_volume_version.rs");
        include!("types/cloudfunctions/function_source_repository.rs");
        include!("types/cloudfunctions/get_function_event_trigger.rs");
        include!("types/cloudfunctions/get_function_event_trigger_failure_policy.rs");
        include!("types/cloudfunctions/get_function_secret_environment_variable.rs");
        include!("types/cloudfunctions/get_function_secret_volume.rs");
        include!("types/cloudfunctions/get_function_secret_volume_version.rs");
        include!("types/cloudfunctions/get_function_source_repository.rs");
    }
    pub mod cloudfunctionsv2 {
        include!("types/cloudfunctionsv2/function_build_config.rs");
        include!(
            "types/cloudfunctionsv2/function_build_config_automatic_update_policy.rs"
        );
        include!(
            "types/cloudfunctionsv2/function_build_config_on_deploy_update_policy.rs"
        );
        include!("types/cloudfunctionsv2/function_build_config_source.rs");
        include!("types/cloudfunctionsv2/function_build_config_source_repo_source.rs");
        include!(
            "types/cloudfunctionsv2/function_build_config_source_storage_source.rs"
        );
        include!("types/cloudfunctionsv2/function_event_trigger.rs");
        include!("types/cloudfunctionsv2/function_event_trigger_event_filter.rs");
        include!("types/cloudfunctionsv2/function_iam_binding_condition.rs");
        include!("types/cloudfunctionsv2/function_iam_member_condition.rs");
        include!("types/cloudfunctionsv2/function_service_config.rs");
        include!(
            "types/cloudfunctionsv2/function_service_config_secret_environment_variable.rs"
        );
        include!("types/cloudfunctionsv2/function_service_config_secret_volume.rs");
        include!(
            "types/cloudfunctionsv2/function_service_config_secret_volume_version.rs"
        );
        include!("types/cloudfunctionsv2/get_function_build_config.rs");
        include!(
            "types/cloudfunctionsv2/get_function_build_config_automatic_update_policy.rs"
        );
        include!(
            "types/cloudfunctionsv2/get_function_build_config_on_deploy_update_policy.rs"
        );
        include!("types/cloudfunctionsv2/get_function_build_config_source.rs");
        include!(
            "types/cloudfunctionsv2/get_function_build_config_source_repo_source.rs"
        );
        include!(
            "types/cloudfunctionsv2/get_function_build_config_source_storage_source.rs"
        );
        include!("types/cloudfunctionsv2/get_function_event_trigger.rs");
        include!("types/cloudfunctionsv2/get_function_event_trigger_event_filter.rs");
        include!("types/cloudfunctionsv2/get_function_service_config.rs");
        include!(
            "types/cloudfunctionsv2/get_function_service_config_secret_environment_variable.rs"
        );
        include!("types/cloudfunctionsv2/get_function_service_config_secret_volume.rs");
        include!(
            "types/cloudfunctionsv2/get_function_service_config_secret_volume_version.rs"
        );
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
#[link_section = "pulumi_wasm_provider::gcp"]
#[no_mangle]
pub static PULUMI_WASM_PROVIDER_gcp: [u8; 6] = *b"8.12.1";
