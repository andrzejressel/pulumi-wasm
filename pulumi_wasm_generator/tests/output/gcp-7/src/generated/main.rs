pub mod iam {
    include!("resources/iam/access_boundary_policy.rs");
    include!("resources/iam/deny_policy.rs");
    include!("resources/iam/folders_policy_binding.rs");
    include!("resources/iam/organizations_policy_binding.rs");
    include!("resources/iam/principal_access_boundary_policy.rs");
    include!("resources/iam/projects_policy_binding.rs");
    include!("resources/iam/workforce_pool.rs");
    include!("resources/iam/workforce_pool_provider.rs");
    include!("resources/iam/workload_identity_pool.rs");
    include!("resources/iam/workload_identity_pool_provider.rs");
}
pub mod iap {
    include!("resources/iap/app_engine_service_iam_binding.rs");
    include!("resources/iap/app_engine_service_iam_member.rs");
    include!("resources/iap/app_engine_service_iam_policy.rs");
    include!("resources/iap/app_engine_version_iam_binding.rs");
    include!("resources/iap/app_engine_version_iam_member.rs");
    include!("resources/iap/app_engine_version_iam_policy.rs");
    include!("resources/iap/brand.rs");
    include!("resources/iap/client.rs");
    include!("resources/iap/settings.rs");
    include!("resources/iap/tunnel_dest_group.rs");
    include!("resources/iap/tunnel_dest_group_iam_binding.rs");
    include!("resources/iap/tunnel_dest_group_iam_member.rs");
    include!("resources/iap/tunnel_dest_group_iam_policy.rs");
    include!("resources/iap/tunnel_iam_binding.rs");
    include!("resources/iap/tunnel_iam_member.rs");
    include!("resources/iap/tunnel_iam_policy.rs");
    include!("resources/iap/tunnel_instance_iam_binding.rs");
    include!("resources/iap/tunnel_instance_iam_member.rs");
    include!("resources/iap/tunnel_instance_iam_policy.rs");
    include!("resources/iap/web_backend_service_iam_binding.rs");
    include!("resources/iap/web_backend_service_iam_member.rs");
    include!("resources/iap/web_backend_service_iam_policy.rs");
    include!("resources/iap/web_iam_binding.rs");
    include!("resources/iap/web_iam_member.rs");
    include!("resources/iap/web_iam_policy.rs");
    include!("resources/iap/web_region_backend_service_iam_binding.rs");
    include!("resources/iap/web_region_backend_service_iam_member.rs");
    include!("resources/iap/web_region_backend_service_iam_policy.rs");
    include!("resources/iap/web_type_app_enging_iam_binding.rs");
    include!("resources/iap/web_type_app_enging_iam_member.rs");
    include!("resources/iap/web_type_app_enging_iam_policy.rs");
    include!("resources/iap/web_type_compute_iam_binding.rs");
    include!("resources/iap/web_type_compute_iam_member.rs");
    include!("resources/iap/web_type_compute_iam_policy.rs");
}
pub mod identityplatform {
    include!("resources/identityplatform/config.rs");
    include!("resources/identityplatform/default_supported_idp_config.rs");
    include!("resources/identityplatform/inbound_saml_config.rs");
    include!("resources/identityplatform/oauth_idp_config.rs");
    include!("resources/identityplatform/tenant.rs");
    include!("resources/identityplatform/tenant_default_supported_idp_config.rs");
    include!("resources/identityplatform/tenant_inbound_saml_config.rs");
    include!("resources/identityplatform/tenant_oauth_idp_config.rs");
}
pub mod integrationconnectors {
    include!("resources/integrationconnectors/connection.rs");
    include!("resources/integrationconnectors/endpoint_attachment.rs");
    include!("resources/integrationconnectors/managed_zone.rs");
}
pub mod kms {
    include!("resources/kms/autokey_config.rs");
    include!("resources/kms/crypto_key.rs");
    include!("resources/kms/crypto_key_iam_binding.rs");
    include!("resources/kms/crypto_key_iam_member.rs");
    include!("resources/kms/crypto_key_iam_policy.rs");
    include!("resources/kms/crypto_key_version.rs");
    include!("resources/kms/ekm_connection.rs");
    include!("resources/kms/ekm_connection_iam_binding.rs");
    include!("resources/kms/ekm_connection_iam_member.rs");
    include!("resources/kms/ekm_connection_iam_policy.rs");
    include!("resources/kms/key_handle.rs");
    include!("resources/kms/key_ring.rs");
    include!("resources/kms/key_ring_iam_binding.rs");
    include!("resources/kms/key_ring_iam_member.rs");
    include!("resources/kms/key_ring_iam_policy.rs");
    include!("resources/kms/key_ring_import_job.rs");
    include!("resources/kms/secret_ciphertext.rs");
}
pub mod logging {
    include!("resources/logging/billing_account_bucket_config.rs");
    include!("resources/logging/billing_account_exclusion.rs");
    include!("resources/logging/billing_account_sink.rs");
    include!("resources/logging/folder_bucket_config.rs");
    include!("resources/logging/folder_exclusion.rs");
    include!("resources/logging/folder_settings.rs");
    include!("resources/logging/folder_sink.rs");
    include!("resources/logging/linked_dataset.rs");
    include!("resources/logging/log_scope.rs");
    include!("resources/logging/log_view.rs");
    include!("resources/logging/log_view_iam_binding.rs");
    include!("resources/logging/log_view_iam_member.rs");
    include!("resources/logging/log_view_iam_policy.rs");
    include!("resources/logging/metric.rs");
    include!("resources/logging/organization_bucket_config.rs");
    include!("resources/logging/organization_exclusion.rs");
    include!("resources/logging/organization_settings.rs");
    include!("resources/logging/organization_sink.rs");
    include!("resources/logging/project_bucket_config.rs");
    include!("resources/logging/project_exclusion.rs");
    include!("resources/logging/project_sink.rs");
}
pub mod looker {
    include!("resources/looker/instance.rs");
}
pub mod managedkafka {
    include!("resources/managedkafka/cluster.rs");
    include!("resources/managedkafka/topic.rs");
}
pub mod memcache {
    include!("resources/memcache/instance.rs");
}
pub mod functions {
    pub mod iam {
        include!("functions/iam/get_rule.rs");
        include!("functions/iam/get_testable_permissions.rs");
        include!("functions/iam/get_workload_identity_pool.rs");
        include!("functions/iam/get_workload_identity_pool_provider.rs");
    }
    pub mod iap {
        include!("functions/iap/get_app_engine_service_iam_policy.rs");
        include!("functions/iap/get_app_engine_version_iam_policy.rs");
        include!("functions/iap/get_client.rs");
        include!("functions/iap/get_tunnel_dest_group_iam_policy.rs");
        include!("functions/iap/get_tunnel_iam_policy.rs");
        include!("functions/iap/get_tunnel_instance_iam_policy.rs");
        include!("functions/iap/get_web_backend_service_iam_policy.rs");
        include!("functions/iap/get_web_iam_policy.rs");
        include!("functions/iap/get_web_region_backend_service_iam_policy.rs");
        include!("functions/iap/get_web_type_app_engine_iam_policy.rs");
        include!("functions/iap/get_web_type_compute_iam_policy.rs");
    }
    pub mod kms {
        include!("functions/kms/get_crypto_key_iam_policy.rs");
        include!("functions/kms/get_crypto_key_latest_version.rs");
        include!("functions/kms/get_crypto_key_versions.rs");
        include!("functions/kms/get_crypto_keys.rs");
        include!("functions/kms/get_ekm_connection_iam_policy.rs");
        include!("functions/kms/get_kms_crypto_key.rs");
        include!("functions/kms/get_kms_crypto_key_version.rs");
        include!("functions/kms/get_kms_key_ring.rs");
        include!("functions/kms/get_kms_secret.rs");
        include!("functions/kms/get_kms_secret_asymmetric.rs");
        include!("functions/kms/get_kms_secret_ciphertext.rs");
        include!("functions/kms/get_key_ring_iam_policy.rs");
        include!("functions/kms/get_key_rings.rs");
    }
    pub mod logging {
        include!("functions/logging/get_folder_settings.rs");
        include!("functions/logging/get_log_view_iam_policy.rs");
        include!("functions/logging/get_organization_settings.rs");
        include!("functions/logging/get_project_cmek_settings.rs");
        include!("functions/logging/get_project_settings.rs");
        include!("functions/logging/get_sink.rs");
    }
}
pub mod types {
    pub mod iam {
        include!("types/iam/access_boundary_policy_rule.rs");
        include!("types/iam/access_boundary_policy_rule_access_boundary_rule.rs");
        include!(
            "types/iam/access_boundary_policy_rule_access_boundary_rule_availability_condition.rs"
        );
        include!("types/iam/deny_policy_rule.rs");
        include!("types/iam/deny_policy_rule_deny_rule.rs");
        include!("types/iam/deny_policy_rule_deny_rule_denial_condition.rs");
        include!("types/iam/folders_policy_binding_condition.rs");
        include!("types/iam/folders_policy_binding_target.rs");
        include!("types/iam/organizations_policy_binding_condition.rs");
        include!("types/iam/organizations_policy_binding_target.rs");
        include!("types/iam/principal_access_boundary_policy_details.rs");
        include!("types/iam/principal_access_boundary_policy_details_rule.rs");
        include!("types/iam/projects_policy_binding_condition.rs");
        include!("types/iam/projects_policy_binding_target.rs");
        include!("types/iam/workforce_pool_access_restrictions.rs");
        include!("types/iam/workforce_pool_access_restrictions_allowed_service.rs");
        include!("types/iam/workforce_pool_provider_extra_attributes_oauth_2_client.rs");
        include!(
            "types/iam/workforce_pool_provider_extra_attributes_oauth_2_client_client_secret.rs"
        );
        include!(
            "types/iam/workforce_pool_provider_extra_attributes_oauth_2_client_client_secret_value.rs"
        );
        include!(
            "types/iam/workforce_pool_provider_extra_attributes_oauth_2_client_query_parameters.rs"
        );
        include!("types/iam/workforce_pool_provider_oidc.rs");
        include!("types/iam/workforce_pool_provider_oidc_client_secret.rs");
        include!("types/iam/workforce_pool_provider_oidc_client_secret_value.rs");
        include!("types/iam/workforce_pool_provider_oidc_web_sso_config.rs");
        include!("types/iam/workforce_pool_provider_saml.rs");
        include!("types/iam/workload_identity_pool_provider_aws.rs");
        include!("types/iam/workload_identity_pool_provider_oidc.rs");
        include!("types/iam/workload_identity_pool_provider_saml.rs");
        include!("types/iam/workload_identity_pool_provider_x_509.rs");
        include!("types/iam/workload_identity_pool_provider_x_509_trust_store.rs");
        include!(
            "types/iam/workload_identity_pool_provider_x_509_trust_store_intermediate_ca.rs"
        );
        include!(
            "types/iam/workload_identity_pool_provider_x_509_trust_store_trust_anchor.rs"
        );
        include!("types/iam/get_testable_permissions_permission.rs");
        include!("types/iam/get_workload_identity_pool_provider_aw.rs");
        include!("types/iam/get_workload_identity_pool_provider_oidc.rs");
        include!("types/iam/get_workload_identity_pool_provider_saml.rs");
        include!("types/iam/get_workload_identity_pool_provider_x_509.rs");
        include!("types/iam/get_workload_identity_pool_provider_x_509_trust_store.rs");
        include!(
            "types/iam/get_workload_identity_pool_provider_x_509_trust_store_intermediate_ca.rs"
        );
        include!(
            "types/iam/get_workload_identity_pool_provider_x_509_trust_store_trust_anchor.rs"
        );
    }
    pub mod iap {
        include!("types/iap/app_engine_service_iam_binding_condition.rs");
        include!("types/iap/app_engine_service_iam_member_condition.rs");
        include!("types/iap/app_engine_version_iam_binding_condition.rs");
        include!("types/iap/app_engine_version_iam_member_condition.rs");
        include!("types/iap/settings_access_settings.rs");
        include!("types/iap/settings_access_settings_allowed_domains_settings.rs");
        include!("types/iap/settings_access_settings_cors_settings.rs");
        include!("types/iap/settings_access_settings_gcip_settings.rs");
        include!("types/iap/settings_access_settings_oauth_settings.rs");
        include!("types/iap/settings_access_settings_reauth_settings.rs");
        include!("types/iap/settings_access_settings_workforce_identity_settings.rs");
        include!(
            "types/iap/settings_access_settings_workforce_identity_settings_oauth_2.rs"
        );
        include!("types/iap/settings_application_settings.rs");
        include!(
            "types/iap/settings_application_settings_access_denied_page_settings.rs"
        );
        include!(
            "types/iap/settings_application_settings_attribute_propagation_settings.rs"
        );
        include!("types/iap/settings_application_settings_csm_settings.rs");
        include!("types/iap/tunnel_dest_group_iam_binding_condition.rs");
        include!("types/iap/tunnel_dest_group_iam_member_condition.rs");
        include!("types/iap/tunnel_iam_binding_condition.rs");
        include!("types/iap/tunnel_iam_member_condition.rs");
        include!("types/iap/tunnel_instance_iam_binding_condition.rs");
        include!("types/iap/tunnel_instance_iam_member_condition.rs");
        include!("types/iap/web_backend_service_iam_binding_condition.rs");
        include!("types/iap/web_backend_service_iam_member_condition.rs");
        include!("types/iap/web_iam_binding_condition.rs");
        include!("types/iap/web_iam_member_condition.rs");
        include!("types/iap/web_region_backend_service_iam_binding_condition.rs");
        include!("types/iap/web_region_backend_service_iam_member_condition.rs");
        include!("types/iap/web_type_app_enging_iam_binding_condition.rs");
        include!("types/iap/web_type_app_enging_iam_member_condition.rs");
        include!("types/iap/web_type_compute_iam_binding_condition.rs");
        include!("types/iap/web_type_compute_iam_member_condition.rs");
    }
    pub mod identityplatform {
        include!("types/identityplatform/config_blocking_functions.rs");
        include!(
            "types/identityplatform/config_blocking_functions_forward_inbound_credentials.rs"
        );
        include!("types/identityplatform/config_blocking_functions_trigger.rs");
        include!("types/identityplatform/config_client.rs");
        include!("types/identityplatform/config_client_permissions.rs");
        include!("types/identityplatform/config_mfa.rs");
        include!("types/identityplatform/config_mfa_provider_config.rs");
        include!(
            "types/identityplatform/config_mfa_provider_config_totp_provider_config.rs"
        );
        include!("types/identityplatform/config_monitoring.rs");
        include!("types/identityplatform/config_monitoring_request_logging.rs");
        include!("types/identityplatform/config_multi_tenant.rs");
        include!("types/identityplatform/config_quota.rs");
        include!("types/identityplatform/config_quota_sign_up_quota_config.rs");
        include!("types/identityplatform/config_sign_in.rs");
        include!("types/identityplatform/config_sign_in_anonymous.rs");
        include!("types/identityplatform/config_sign_in_email.rs");
        include!("types/identityplatform/config_sign_in_hash_config.rs");
        include!("types/identityplatform/config_sign_in_phone_number.rs");
        include!("types/identityplatform/config_sms_region_config.rs");
        include!("types/identityplatform/config_sms_region_config_allow_by_default.rs");
        include!("types/identityplatform/config_sms_region_config_allowlist_only.rs");
        include!("types/identityplatform/inbound_saml_config_idp_config.rs");
        include!(
            "types/identityplatform/inbound_saml_config_idp_config_idp_certificate.rs"
        );
        include!("types/identityplatform/inbound_saml_config_sp_config.rs");
        include!(
            "types/identityplatform/inbound_saml_config_sp_config_sp_certificate.rs"
        );
        include!("types/identityplatform/tenant_inbound_saml_config_idp_config.rs");
        include!(
            "types/identityplatform/tenant_inbound_saml_config_idp_config_idp_certificate.rs"
        );
        include!("types/identityplatform/tenant_inbound_saml_config_sp_config.rs");
        include!(
            "types/identityplatform/tenant_inbound_saml_config_sp_config_sp_certificate.rs"
        );
    }
    pub mod integrationconnectors {
        include!("types/integrationconnectors/connection_auth_config.rs");
        include!(
            "types/integrationconnectors/connection_auth_config_additional_variable.rs"
        );
        include!(
            "types/integrationconnectors/connection_auth_config_additional_variable_encryption_key_value.rs"
        );
        include!(
            "types/integrationconnectors/connection_auth_config_additional_variable_secret_value.rs"
        );
        include!(
            "types/integrationconnectors/connection_auth_config_oauth_2_auth_code_flow.rs"
        );
        include!(
            "types/integrationconnectors/connection_auth_config_oauth_2_auth_code_flow_client_secret.rs"
        );
        include!(
            "types/integrationconnectors/connection_auth_config_oauth_2_client_credentials.rs"
        );
        include!(
            "types/integrationconnectors/connection_auth_config_oauth_2_client_credentials_client_secret.rs"
        );
        include!(
            "types/integrationconnectors/connection_auth_config_oauth_2_jwt_bearer.rs"
        );
        include!(
            "types/integrationconnectors/connection_auth_config_oauth_2_jwt_bearer_client_key.rs"
        );
        include!(
            "types/integrationconnectors/connection_auth_config_oauth_2_jwt_bearer_jwt_claims.rs"
        );
        include!("types/integrationconnectors/connection_auth_config_ssh_public_key.rs");
        include!(
            "types/integrationconnectors/connection_auth_config_ssh_public_key_ssh_client_cert.rs"
        );
        include!(
            "types/integrationconnectors/connection_auth_config_ssh_public_key_ssh_client_cert_pass.rs"
        );
        include!("types/integrationconnectors/connection_auth_config_user_password.rs");
        include!(
            "types/integrationconnectors/connection_auth_config_user_password_password.rs"
        );
        include!("types/integrationconnectors/connection_config_variable.rs");
        include!(
            "types/integrationconnectors/connection_config_variable_encryption_key_value.rs"
        );
        include!(
            "types/integrationconnectors/connection_config_variable_secret_value.rs"
        );
        include!(
            "types/integrationconnectors/connection_connector_version_infra_config.rs"
        );
        include!("types/integrationconnectors/connection_destination_config.rs");
        include!(
            "types/integrationconnectors/connection_destination_config_destination.rs"
        );
        include!("types/integrationconnectors/connection_eventing_config.rs");
        include!(
            "types/integrationconnectors/connection_eventing_config_additional_variable.rs"
        );
        include!(
            "types/integrationconnectors/connection_eventing_config_additional_variable_encryption_key_value.rs"
        );
        include!(
            "types/integrationconnectors/connection_eventing_config_additional_variable_secret_value.rs"
        );
        include!(
            "types/integrationconnectors/connection_eventing_config_auth_config.rs"
        );
        include!(
            "types/integrationconnectors/connection_eventing_config_auth_config_additional_variable.rs"
        );
        include!(
            "types/integrationconnectors/connection_eventing_config_auth_config_additional_variable_encryption_key_value.rs"
        );
        include!(
            "types/integrationconnectors/connection_eventing_config_auth_config_additional_variable_secret_value.rs"
        );
        include!(
            "types/integrationconnectors/connection_eventing_config_auth_config_user_password.rs"
        );
        include!(
            "types/integrationconnectors/connection_eventing_config_auth_config_user_password_password.rs"
        );
        include!(
            "types/integrationconnectors/connection_eventing_config_registration_destination_config.rs"
        );
        include!(
            "types/integrationconnectors/connection_eventing_config_registration_destination_config_destination.rs"
        );
        include!("types/integrationconnectors/connection_eventing_runtime_data.rs");
        include!(
            "types/integrationconnectors/connection_eventing_runtime_data_status.rs"
        );
        include!("types/integrationconnectors/connection_lock_config.rs");
        include!("types/integrationconnectors/connection_log_config.rs");
        include!("types/integrationconnectors/connection_node_config.rs");
        include!("types/integrationconnectors/connection_ssl_config.rs");
        include!(
            "types/integrationconnectors/connection_ssl_config_additional_variable.rs"
        );
        include!(
            "types/integrationconnectors/connection_ssl_config_additional_variable_encryption_key_value.rs"
        );
        include!(
            "types/integrationconnectors/connection_ssl_config_additional_variable_secret_value.rs"
        );
        include!(
            "types/integrationconnectors/connection_ssl_config_client_certificate.rs"
        );
        include!(
            "types/integrationconnectors/connection_ssl_config_client_private_key.rs"
        );
        include!(
            "types/integrationconnectors/connection_ssl_config_client_private_key_pass.rs"
        );
        include!(
            "types/integrationconnectors/connection_ssl_config_private_server_certificate.rs"
        );
        include!("types/integrationconnectors/connection_status.rs");
    }
    pub mod kms {
        include!("types/kms/crypto_key_iam_binding_condition.rs");
        include!("types/kms/crypto_key_iam_member_condition.rs");
        include!("types/kms/crypto_key_key_access_justifications_policy.rs");
        include!("types/kms/crypto_key_primary.rs");
        include!("types/kms/crypto_key_version_attestation.rs");
        include!("types/kms/crypto_key_version_attestation_cert_chains.rs");
        include!(
            "types/kms/crypto_key_version_attestation_external_protection_level_options.rs"
        );
        include!("types/kms/crypto_key_version_external_protection_level_options.rs");
        include!("types/kms/crypto_key_version_template.rs");
        include!("types/kms/ekm_connection_iam_binding_condition.rs");
        include!("types/kms/ekm_connection_iam_member_condition.rs");
        include!("types/kms/ekm_connection_service_resolver.rs");
        include!("types/kms/ekm_connection_service_resolver_server_certificate.rs");
        include!("types/kms/key_ring_iam_binding_condition.rs");
        include!("types/kms/key_ring_iam_member_condition.rs");
        include!("types/kms/key_ring_import_job_attestation.rs");
        include!("types/kms/key_ring_import_job_public_key.rs");
        include!("types/kms/get_crypto_key_latest_version_public_key.rs");
        include!("types/kms/get_crypto_key_versions_public_key.rs");
        include!("types/kms/get_crypto_key_versions_version.rs");
        include!("types/kms/get_crypto_key_versions_version_public_key.rs");
        include!("types/kms/get_crypto_keys_key.rs");
        include!("types/kms/get_crypto_keys_key_key_access_justifications_policy.rs");
        include!("types/kms/get_crypto_keys_key_primary.rs");
        include!("types/kms/get_crypto_keys_key_version_template.rs");
        include!("types/kms/get_kms_crypto_key_key_access_justifications_policy.rs");
        include!("types/kms/get_kms_crypto_key_primary.rs");
        include!("types/kms/get_kms_crypto_key_version_public_key.rs");
        include!("types/kms/get_kms_crypto_key_version_template.rs");
        include!("types/kms/get_key_rings_key_ring.rs");
    }
    pub mod logging {
        include!("types/logging/billing_account_bucket_config_cmek_settings.rs");
        include!("types/logging/billing_account_bucket_config_index_config.rs");
        include!("types/logging/billing_account_sink_bigquery_options.rs");
        include!("types/logging/billing_account_sink_exclusion.rs");
        include!("types/logging/folder_bucket_config_cmek_settings.rs");
        include!("types/logging/folder_bucket_config_index_config.rs");
        include!("types/logging/folder_sink_bigquery_options.rs");
        include!("types/logging/folder_sink_exclusion.rs");
        include!("types/logging/linked_dataset_bigquery_dataset.rs");
        include!("types/logging/log_view_iam_binding_condition.rs");
        include!("types/logging/log_view_iam_member_condition.rs");
        include!("types/logging/metric_bucket_options.rs");
        include!("types/logging/metric_bucket_options_explicit_buckets.rs");
        include!("types/logging/metric_bucket_options_exponential_buckets.rs");
        include!("types/logging/metric_bucket_options_linear_buckets.rs");
        include!("types/logging/metric_metric_descriptor.rs");
        include!("types/logging/metric_metric_descriptor_label.rs");
        include!("types/logging/organization_bucket_config_cmek_settings.rs");
        include!("types/logging/organization_bucket_config_index_config.rs");
        include!("types/logging/organization_sink_bigquery_options.rs");
        include!("types/logging/organization_sink_exclusion.rs");
        include!("types/logging/project_bucket_config_cmek_settings.rs");
        include!("types/logging/project_bucket_config_index_config.rs");
        include!("types/logging/project_sink_bigquery_options.rs");
        include!("types/logging/project_sink_exclusion.rs");
        include!("types/logging/get_sink_bigquery_option.rs");
        include!("types/logging/get_sink_exclusion.rs");
    }
    pub mod looker {
        include!("types/looker/instance_admin_settings.rs");
        include!("types/looker/instance_custom_domain.rs");
        include!("types/looker/instance_deny_maintenance_period.rs");
        include!("types/looker/instance_deny_maintenance_period_end_date.rs");
        include!("types/looker/instance_deny_maintenance_period_start_date.rs");
        include!("types/looker/instance_deny_maintenance_period_time.rs");
        include!("types/looker/instance_encryption_config.rs");
        include!("types/looker/instance_maintenance_window.rs");
        include!("types/looker/instance_maintenance_window_start_time.rs");
        include!("types/looker/instance_oauth_config.rs");
        include!("types/looker/instance_psc_config.rs");
        include!("types/looker/instance_psc_config_service_attachment.rs");
        include!("types/looker/instance_user_metadata.rs");
    }
    pub mod managedkafka {
        include!("types/managedkafka/cluster_capacity_config.rs");
        include!("types/managedkafka/cluster_gcp_config.rs");
        include!("types/managedkafka/cluster_gcp_config_access_config.rs");
        include!(
            "types/managedkafka/cluster_gcp_config_access_config_network_config.rs"
        );
        include!("types/managedkafka/cluster_rebalance_config.rs");
    }
    pub mod memcache {
        include!("types/memcache/instance_maintenance_policy.rs");
        include!(
            "types/memcache/instance_maintenance_policy_weekly_maintenance_window.rs"
        );
        include!(
            "types/memcache/instance_maintenance_policy_weekly_maintenance_window_start_time.rs"
        );
        include!("types/memcache/instance_maintenance_schedule.rs");
        include!("types/memcache/instance_memcache_node.rs");
        include!("types/memcache/instance_memcache_parameters.rs");
        include!("types/memcache/instance_node_config.rs");
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
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_GCP: [u8; 45] = *b"{\"version\":\"8.12.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "8.12.1".to_string()
}
