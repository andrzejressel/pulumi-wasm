pub mod serviceusage {
    include!("resources/serviceusage/consumer_quota_override.rs");
}
pub mod siteverification {
    include!("resources/siteverification/owner.rs");
    include!("resources/siteverification/web_resource.rs");
}
pub mod sourcerepo {
    include!("resources/sourcerepo/repository.rs");
    include!("resources/sourcerepo/repository_iam_binding.rs");
    include!("resources/sourcerepo/repository_iam_member.rs");
    include!("resources/sourcerepo/repository_iam_policy.rs");
}
pub mod spanner {
    include!("resources/spanner/backup_schedule.rs");
    include!("resources/spanner/database.rs");
    include!("resources/spanner/database_iam_binding.rs");
    include!("resources/spanner/database_iam_member.rs");
    include!("resources/spanner/database_iam_policy.rs");
    include!("resources/spanner/instance.rs");
    include!("resources/spanner/instance_config.rs");
    include!("resources/spanner/instance_iam_binding.rs");
    include!("resources/spanner/instance_iam_member.rs");
    include!("resources/spanner/instance_iam_policy.rs");
}
pub mod sql {
    include!("resources/sql/database.rs");
    include!("resources/sql/database_instance.rs");
    include!("resources/sql/source_representation_instance.rs");
    include!("resources/sql/ssl_cert.rs");
    include!("resources/sql/user.rs");
}
pub mod storage {
    include!("resources/storage/bucket.rs");
    include!("resources/storage/bucket_acl.rs");
    include!("resources/storage/bucket_access_control.rs");
    include!("resources/storage/bucket_iam_binding.rs");
    include!("resources/storage/bucket_iam_member.rs");
    include!("resources/storage/bucket_iam_policy.rs");
    include!("resources/storage/bucket_object.rs");
    include!("resources/storage/default_object_acl.rs");
    include!("resources/storage/default_object_access_control.rs");
    include!("resources/storage/hmac_key.rs");
    include!("resources/storage/insights_report_config.rs");
    include!("resources/storage/managed_folder.rs");
    include!("resources/storage/managed_folder_iam_binding.rs");
    include!("resources/storage/managed_folder_iam_member.rs");
    include!("resources/storage/managed_folder_iam_policy.rs");
    include!("resources/storage/notification.rs");
    include!("resources/storage/object_acl.rs");
    include!("resources/storage/object_access_control.rs");
    include!("resources/storage/transfer_agent_pool.rs");
    include!("resources/storage/transfer_job.rs");
}
pub mod tags {
    include!("resources/tags/location_tag_binding.rs");
    include!("resources/tags/tag_binding.rs");
    include!("resources/tags/tag_key.rs");
    include!("resources/tags/tag_key_iam_binding.rs");
    include!("resources/tags/tag_key_iam_member.rs");
    include!("resources/tags/tag_key_iam_policy.rs");
    include!("resources/tags/tag_value.rs");
    include!("resources/tags/tag_value_iam_binding.rs");
    include!("resources/tags/tag_value_iam_member.rs");
    include!("resources/tags/tag_value_iam_policy.rs");
}
pub mod tpu {
    include!("resources/tpu/node.rs");
    include!("resources/tpu/v_2_queued_resource.rs");
    include!("resources/tpu/v_2_vm.rs");
}
pub mod transcoder {
    include!("resources/transcoder/job.rs");
    include!("resources/transcoder/job_template.rs");
}
pub mod vertex {
    include!("resources/vertex/ai_dataset.rs");
    include!("resources/vertex/ai_deployment_resource_pool.rs");
    include!("resources/vertex/ai_endpoint.rs");
    include!("resources/vertex/ai_endpoint_iam_binding.rs");
    include!("resources/vertex/ai_endpoint_iam_member.rs");
    include!("resources/vertex/ai_endpoint_iam_policy.rs");
    include!("resources/vertex/ai_feature_group.rs");
    include!("resources/vertex/ai_feature_group_feature.rs");
    include!("resources/vertex/ai_feature_online_store.rs");
    include!("resources/vertex/ai_feature_online_store_featureview.rs");
    include!("resources/vertex/ai_feature_store.rs");
    include!("resources/vertex/ai_feature_store_entity_type.rs");
    include!("resources/vertex/ai_feature_store_entity_type_feature.rs");
    include!("resources/vertex/ai_feature_store_entity_type_iam_binding.rs");
    include!("resources/vertex/ai_feature_store_entity_type_iam_member.rs");
    include!("resources/vertex/ai_feature_store_entity_type_iam_policy.rs");
    include!("resources/vertex/ai_feature_store_iam_binding.rs");
    include!("resources/vertex/ai_feature_store_iam_member.rs");
    include!("resources/vertex/ai_feature_store_iam_policy.rs");
    include!("resources/vertex/ai_index.rs");
    include!("resources/vertex/ai_index_endpoint.rs");
    include!("resources/vertex/ai_index_endpoint_deployed_index.rs");
    include!("resources/vertex/ai_metadata_store.rs");
    include!("resources/vertex/ai_tensorboard.rs");
}
pub mod functions {
    pub mod siteverification {
        include!("functions/siteverification/get_token.rs");
    }
    pub mod sourcerepo {
        include!("functions/sourcerepo/get_repository.rs");
        include!("functions/sourcerepo/get_repository_iam_policy.rs");
    }
    pub mod spanner {
        include!("functions/spanner/get_database.rs");
        include!("functions/spanner/get_database_iam_policy.rs");
        include!("functions/spanner/get_instance.rs");
        include!("functions/spanner/get_instance_iam_policy.rs");
    }
    pub mod sql {
        include!("functions/sql/get_backup_run.rs");
        include!("functions/sql/get_ca_certs.rs");
        include!("functions/sql/get_database.rs");
        include!("functions/sql/get_database_instance.rs");
        include!("functions/sql/get_database_instance_latest_recovery_time.rs");
        include!("functions/sql/get_database_instances.rs");
        include!("functions/sql/get_databases.rs");
        include!("functions/sql/get_tiers.rs");
    }
    pub mod storage {
        include!("functions/storage/get_bucket.rs");
        include!("functions/storage/get_bucket_iam_policy.rs");
        include!("functions/storage/get_bucket_object.rs");
        include!("functions/storage/get_bucket_object_content.rs");
        include!("functions/storage/get_bucket_objects.rs");
        include!("functions/storage/get_buckets.rs");
        include!("functions/storage/get_managed_folder_iam_policy.rs");
        include!("functions/storage/get_object_signed_url.rs");
        include!("functions/storage/get_project_service_account.rs");
        include!("functions/storage/get_transfer_project_service_account.rs");
        include!("functions/storage/get_transfer_project_servie_account.rs");
    }
    pub mod tags {
        include!("functions/tags/get_tag_key.rs");
        include!("functions/tags/get_tag_key_iam_policy.rs");
        include!("functions/tags/get_tag_keys.rs");
        include!("functions/tags/get_tag_value.rs");
        include!("functions/tags/get_tag_value_iam_policy.rs");
        include!("functions/tags/get_tag_values.rs");
    }
    pub mod tpu {
        include!("functions/tpu/get_tensorflow_versions.rs");
        include!("functions/tpu/get_v_2_accelerator_types.rs");
        include!("functions/tpu/get_v_2_runtime_versions.rs");
    }
    pub mod vertex {
        include!("functions/vertex/get_ai_endpoint_iam_policy.rs");
        include!("functions/vertex/get_ai_featurestore_entitytype_iam_policy.rs");
        include!("functions/vertex/get_ai_featurestore_iam_policy.rs");
        include!("functions/vertex/get_ai_index.rs");
    }
}
pub mod types {
    pub mod siteverification {
        include!("types/siteverification/web_resource_site.rs");
    }
    pub mod sourcerepo {
        include!("types/sourcerepo/repository_iam_binding_condition.rs");
        include!("types/sourcerepo/repository_iam_member_condition.rs");
        include!("types/sourcerepo/repository_pubsub_config.rs");
        include!("types/sourcerepo/get_repository_pubsub_config.rs");
    }
    pub mod spanner {
        include!("types/spanner/backup_schedule_full_backup_spec.rs");
        include!("types/spanner/backup_schedule_incremental_backup_spec.rs");
        include!("types/spanner/backup_schedule_spec.rs");
        include!("types/spanner/backup_schedule_spec_cron_spec.rs");
        include!("types/spanner/database_encryption_config.rs");
        include!("types/spanner/database_iam_binding_condition.rs");
        include!("types/spanner/database_iam_member_condition.rs");
        include!("types/spanner/instance_autoscaling_config.rs");
        include!(
            "types/spanner/instance_autoscaling_config_asymmetric_autoscaling_option.rs"
        );
        include!(
            "types/spanner/instance_autoscaling_config_asymmetric_autoscaling_option_overrides.rs"
        );
        include!(
            "types/spanner/instance_autoscaling_config_asymmetric_autoscaling_option_overrides_autoscaling_limits.rs"
        );
        include!(
            "types/spanner/instance_autoscaling_config_asymmetric_autoscaling_option_replica_selection.rs"
        );
        include!("types/spanner/instance_autoscaling_config_autoscaling_limits.rs");
        include!("types/spanner/instance_autoscaling_config_autoscaling_targets.rs");
        include!("types/spanner/instance_config_replica.rs");
        include!("types/spanner/instance_iam_binding_condition.rs");
        include!("types/spanner/instance_iam_member_condition.rs");
        include!("types/spanner/get_database_encryption_config.rs");
        include!("types/spanner/get_instance_autoscaling_config.rs");
        include!(
            "types/spanner/get_instance_autoscaling_config_asymmetric_autoscaling_option.rs"
        );
        include!(
            "types/spanner/get_instance_autoscaling_config_asymmetric_autoscaling_option_override.rs"
        );
        include!(
            "types/spanner/get_instance_autoscaling_config_asymmetric_autoscaling_option_override_autoscaling_limit.rs"
        );
        include!(
            "types/spanner/get_instance_autoscaling_config_asymmetric_autoscaling_option_replica_selection.rs"
        );
        include!("types/spanner/get_instance_autoscaling_config_autoscaling_limit.rs");
        include!("types/spanner/get_instance_autoscaling_config_autoscaling_target.rs");
    }
    pub mod sql {
        include!("types/sql/database_instance_clone.rs");
        include!("types/sql/database_instance_ip_address.rs");
        include!("types/sql/database_instance_replica_configuration.rs");
        include!("types/sql/database_instance_restore_backup_context.rs");
        include!("types/sql/database_instance_server_ca_cert.rs");
        include!("types/sql/database_instance_settings.rs");
        include!("types/sql/database_instance_settings_active_directory_config.rs");
        include!("types/sql/database_instance_settings_advanced_machine_features.rs");
        include!("types/sql/database_instance_settings_backup_configuration.rs");
        include!(
            "types/sql/database_instance_settings_backup_configuration_backup_retention_settings.rs"
        );
        include!("types/sql/database_instance_settings_data_cache_config.rs");
        include!("types/sql/database_instance_settings_database_flag.rs");
        include!("types/sql/database_instance_settings_deny_maintenance_period.rs");
        include!("types/sql/database_instance_settings_insights_config.rs");
        include!("types/sql/database_instance_settings_ip_configuration.rs");
        include!(
            "types/sql/database_instance_settings_ip_configuration_authorized_network.rs"
        );
        include!("types/sql/database_instance_settings_ip_configuration_psc_config.rs");
        include!(
            "types/sql/database_instance_settings_ip_configuration_psc_config_psc_auto_connection.rs"
        );
        include!("types/sql/database_instance_settings_location_preference.rs");
        include!("types/sql/database_instance_settings_maintenance_window.rs");
        include!("types/sql/database_instance_settings_password_validation_policy.rs");
        include!("types/sql/database_instance_settings_sql_server_audit_config.rs");
        include!("types/sql/user_password_policy.rs");
        include!("types/sql/user_password_policy_status.rs");
        include!("types/sql/user_sql_server_user_detail.rs");
        include!("types/sql/get_ca_certs_cert.rs");
        include!("types/sql/get_database_instance_clone.rs");
        include!("types/sql/get_database_instance_ip_address.rs");
        include!("types/sql/get_database_instance_replica_configuration.rs");
        include!("types/sql/get_database_instance_restore_backup_context.rs");
        include!("types/sql/get_database_instance_server_ca_cert.rs");
        include!("types/sql/get_database_instance_setting.rs");
        include!("types/sql/get_database_instance_setting_active_directory_config.rs");
        include!("types/sql/get_database_instance_setting_advanced_machine_feature.rs");
        include!("types/sql/get_database_instance_setting_backup_configuration.rs");
        include!(
            "types/sql/get_database_instance_setting_backup_configuration_backup_retention_setting.rs"
        );
        include!("types/sql/get_database_instance_setting_data_cache_config.rs");
        include!("types/sql/get_database_instance_setting_database_flag.rs");
        include!("types/sql/get_database_instance_setting_deny_maintenance_period.rs");
        include!("types/sql/get_database_instance_setting_insights_config.rs");
        include!("types/sql/get_database_instance_setting_ip_configuration.rs");
        include!(
            "types/sql/get_database_instance_setting_ip_configuration_authorized_network.rs"
        );
        include!(
            "types/sql/get_database_instance_setting_ip_configuration_psc_config.rs"
        );
        include!(
            "types/sql/get_database_instance_setting_ip_configuration_psc_config_psc_auto_connection.rs"
        );
        include!("types/sql/get_database_instance_setting_location_preference.rs");
        include!("types/sql/get_database_instance_setting_maintenance_window.rs");
        include!(
            "types/sql/get_database_instance_setting_password_validation_policy.rs"
        );
        include!("types/sql/get_database_instance_setting_sql_server_audit_config.rs");
        include!("types/sql/get_database_instances_instance.rs");
        include!("types/sql/get_database_instances_instance_clone.rs");
        include!("types/sql/get_database_instances_instance_ip_address.rs");
        include!("types/sql/get_database_instances_instance_replica_configuration.rs");
        include!("types/sql/get_database_instances_instance_restore_backup_context.rs");
        include!("types/sql/get_database_instances_instance_server_ca_cert.rs");
        include!("types/sql/get_database_instances_instance_setting.rs");
        include!(
            "types/sql/get_database_instances_instance_setting_active_directory_config.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_advanced_machine_feature.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_backup_configuration.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_backup_configuration_backup_retention_setting.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_data_cache_config.rs"
        );
        include!("types/sql/get_database_instances_instance_setting_database_flag.rs");
        include!(
            "types/sql/get_database_instances_instance_setting_deny_maintenance_period.rs"
        );
        include!("types/sql/get_database_instances_instance_setting_insights_config.rs");
        include!(
            "types/sql/get_database_instances_instance_setting_ip_configuration.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_ip_configuration_authorized_network.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_ip_configuration_psc_config.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_ip_configuration_psc_config_psc_auto_connection.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_location_preference.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_maintenance_window.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_password_validation_policy.rs"
        );
        include!(
            "types/sql/get_database_instances_instance_setting_sql_server_audit_config.rs"
        );
        include!("types/sql/get_databases_database.rs");
        include!("types/sql/get_tiers_tier.rs");
    }
    pub mod storage {
        include!("types/storage/bucket_autoclass.rs");
        include!("types/storage/bucket_cor.rs");
        include!("types/storage/bucket_custom_placement_config.rs");
        include!("types/storage/bucket_encryption.rs");
        include!("types/storage/bucket_hierarchical_namespace.rs");
        include!("types/storage/bucket_iam_binding_condition.rs");
        include!("types/storage/bucket_iam_member_condition.rs");
        include!("types/storage/bucket_lifecycle_rule.rs");
        include!("types/storage/bucket_lifecycle_rule_action.rs");
        include!("types/storage/bucket_lifecycle_rule_condition.rs");
        include!("types/storage/bucket_logging.rs");
        include!("types/storage/bucket_object_customer_encryption.rs");
        include!("types/storage/bucket_object_retention.rs");
        include!("types/storage/bucket_retention_policy.rs");
        include!("types/storage/bucket_soft_delete_policy.rs");
        include!("types/storage/bucket_versioning.rs");
        include!("types/storage/bucket_website.rs");
        include!("types/storage/default_object_access_control_project_team.rs");
        include!("types/storage/insights_report_config_csv_options.rs");
        include!("types/storage/insights_report_config_frequency_options.rs");
        include!("types/storage/insights_report_config_frequency_options_end_date.rs");
        include!("types/storage/insights_report_config_frequency_options_start_date.rs");
        include!(
            "types/storage/insights_report_config_object_metadata_report_options.rs"
        );
        include!(
            "types/storage/insights_report_config_object_metadata_report_options_storage_destination_options.rs"
        );
        include!(
            "types/storage/insights_report_config_object_metadata_report_options_storage_filters.rs"
        );
        include!("types/storage/managed_folder_iam_binding_condition.rs");
        include!("types/storage/managed_folder_iam_member_condition.rs");
        include!("types/storage/object_access_control_project_team.rs");
        include!("types/storage/transfer_agent_pool_bandwidth_limit.rs");
        include!("types/storage/transfer_job_event_stream.rs");
        include!("types/storage/transfer_job_notification_config.rs");
        include!("types/storage/transfer_job_schedule.rs");
        include!("types/storage/transfer_job_schedule_schedule_end_date.rs");
        include!("types/storage/transfer_job_schedule_schedule_start_date.rs");
        include!("types/storage/transfer_job_schedule_start_time_of_day.rs");
        include!("types/storage/transfer_job_transfer_spec.rs");
        include!("types/storage/transfer_job_transfer_spec_aws_s_3_data_source.rs");
        include!(
            "types/storage/transfer_job_transfer_spec_aws_s_3_data_source_aws_access_key.rs"
        );
        include!(
            "types/storage/transfer_job_transfer_spec_azure_blob_storage_data_source.rs"
        );
        include!(
            "types/storage/transfer_job_transfer_spec_azure_blob_storage_data_source_azure_credentials.rs"
        );
        include!("types/storage/transfer_job_transfer_spec_gcs_data_sink.rs");
        include!("types/storage/transfer_job_transfer_spec_gcs_data_source.rs");
        include!("types/storage/transfer_job_transfer_spec_hdfs_data_source.rs");
        include!("types/storage/transfer_job_transfer_spec_http_data_source.rs");
        include!("types/storage/transfer_job_transfer_spec_object_conditions.rs");
        include!("types/storage/transfer_job_transfer_spec_posix_data_sink.rs");
        include!("types/storage/transfer_job_transfer_spec_posix_data_source.rs");
        include!("types/storage/transfer_job_transfer_spec_transfer_options.rs");
        include!("types/storage/get_bucket_autoclass.rs");
        include!("types/storage/get_bucket_cor.rs");
        include!("types/storage/get_bucket_custom_placement_config.rs");
        include!("types/storage/get_bucket_encryption.rs");
        include!("types/storage/get_bucket_hierarchical_namespace.rs");
        include!("types/storage/get_bucket_lifecycle_rule.rs");
        include!("types/storage/get_bucket_lifecycle_rule_action.rs");
        include!("types/storage/get_bucket_lifecycle_rule_condition.rs");
        include!("types/storage/get_bucket_logging.rs");
        include!("types/storage/get_bucket_object_content_customer_encryption.rs");
        include!("types/storage/get_bucket_object_content_retention.rs");
        include!("types/storage/get_bucket_object_customer_encryption.rs");
        include!("types/storage/get_bucket_object_retention.rs");
        include!("types/storage/get_bucket_objects_bucket_object.rs");
        include!("types/storage/get_bucket_retention_policy.rs");
        include!("types/storage/get_bucket_soft_delete_policy.rs");
        include!("types/storage/get_bucket_versioning.rs");
        include!("types/storage/get_bucket_website.rs");
        include!("types/storage/get_buckets_bucket.rs");
    }
    pub mod tags {
        include!("types/tags/tag_key_iam_binding_condition.rs");
        include!("types/tags/tag_key_iam_member_condition.rs");
        include!("types/tags/tag_value_iam_binding_condition.rs");
        include!("types/tags/tag_value_iam_member_condition.rs");
        include!("types/tags/get_tag_keys_key.rs");
        include!("types/tags/get_tag_values_value.rs");
    }
    pub mod tpu {
        include!("types/tpu/node_network_endpoint.rs");
        include!("types/tpu/node_scheduling_config.rs");
        include!("types/tpu/v_2_queued_resource_tpu.rs");
        include!("types/tpu/v_2_queued_resource_tpu_node_spec.rs");
        include!("types/tpu/v_2_queued_resource_tpu_node_spec_node.rs");
        include!("types/tpu/v_2_vm_accelerator_config.rs");
        include!("types/tpu/v_2_vm_data_disk.rs");
        include!("types/tpu/v_2_vm_network_config.rs");
        include!("types/tpu/v_2_vm_network_endpoint.rs");
        include!("types/tpu/v_2_vm_network_endpoint_access_config.rs");
        include!("types/tpu/v_2_vm_scheduling_config.rs");
        include!("types/tpu/v_2_vm_service_account.rs");
        include!("types/tpu/v_2_vm_shielded_instance_config.rs");
        include!("types/tpu/v_2_vm_symptom.rs");
    }
    pub mod transcoder {
        include!("types/transcoder/job_config.rs");
        include!("types/transcoder/job_config_ad_break.rs");
        include!("types/transcoder/job_config_edit_list.rs");
        include!("types/transcoder/job_config_elementary_stream.rs");
        include!("types/transcoder/job_config_elementary_stream_audio_stream.rs");
        include!("types/transcoder/job_config_elementary_stream_video_stream.rs");
        include!("types/transcoder/job_config_elementary_stream_video_stream_h_264.rs");
        include!(
            "types/transcoder/job_config_elementary_stream_video_stream_h_264_hlg.rs"
        );
        include!(
            "types/transcoder/job_config_elementary_stream_video_stream_h_264_sdr.rs"
        );
        include!("types/transcoder/job_config_encryption.rs");
        include!("types/transcoder/job_config_encryption_aes_128.rs");
        include!("types/transcoder/job_config_encryption_drm_systems.rs");
        include!("types/transcoder/job_config_encryption_drm_systems_clearkey.rs");
        include!("types/transcoder/job_config_encryption_drm_systems_fairplay.rs");
        include!("types/transcoder/job_config_encryption_drm_systems_playready.rs");
        include!("types/transcoder/job_config_encryption_drm_systems_widevine.rs");
        include!("types/transcoder/job_config_encryption_mpeg_cenc.rs");
        include!("types/transcoder/job_config_encryption_sample_aes.rs");
        include!("types/transcoder/job_config_encryption_secret_manager_key_source.rs");
        include!("types/transcoder/job_config_input.rs");
        include!("types/transcoder/job_config_manifest.rs");
        include!("types/transcoder/job_config_mux_stream.rs");
        include!("types/transcoder/job_config_mux_stream_segment_settings.rs");
        include!("types/transcoder/job_config_output.rs");
        include!("types/transcoder/job_config_overlay.rs");
        include!("types/transcoder/job_config_overlay_animation.rs");
        include!("types/transcoder/job_config_overlay_animation_animation_fade.rs");
        include!("types/transcoder/job_config_overlay_animation_animation_fade_xy.rs");
        include!("types/transcoder/job_config_overlay_image.rs");
        include!("types/transcoder/job_config_pubsub_destination.rs");
        include!("types/transcoder/job_template_config.rs");
        include!("types/transcoder/job_template_config_ad_break.rs");
        include!("types/transcoder/job_template_config_edit_list.rs");
        include!("types/transcoder/job_template_config_elementary_stream.rs");
        include!(
            "types/transcoder/job_template_config_elementary_stream_audio_stream.rs"
        );
        include!(
            "types/transcoder/job_template_config_elementary_stream_video_stream.rs"
        );
        include!(
            "types/transcoder/job_template_config_elementary_stream_video_stream_h_264.rs"
        );
        include!(
            "types/transcoder/job_template_config_elementary_stream_video_stream_h_264_hlg.rs"
        );
        include!(
            "types/transcoder/job_template_config_elementary_stream_video_stream_h_264_sdr.rs"
        );
        include!("types/transcoder/job_template_config_encryption.rs");
        include!("types/transcoder/job_template_config_encryption_aes_128.rs");
        include!("types/transcoder/job_template_config_encryption_drm_systems.rs");
        include!(
            "types/transcoder/job_template_config_encryption_drm_systems_clearkey.rs"
        );
        include!(
            "types/transcoder/job_template_config_encryption_drm_systems_fairplay.rs"
        );
        include!(
            "types/transcoder/job_template_config_encryption_drm_systems_playready.rs"
        );
        include!(
            "types/transcoder/job_template_config_encryption_drm_systems_widevine.rs"
        );
        include!("types/transcoder/job_template_config_encryption_mpeg_cenc.rs");
        include!("types/transcoder/job_template_config_encryption_sample_aes.rs");
        include!(
            "types/transcoder/job_template_config_encryption_secret_manager_key_source.rs"
        );
        include!("types/transcoder/job_template_config_input.rs");
        include!("types/transcoder/job_template_config_manifest.rs");
        include!("types/transcoder/job_template_config_mux_stream.rs");
        include!("types/transcoder/job_template_config_mux_stream_segment_settings.rs");
        include!("types/transcoder/job_template_config_output.rs");
        include!("types/transcoder/job_template_config_overlay.rs");
        include!("types/transcoder/job_template_config_overlay_animation.rs");
        include!(
            "types/transcoder/job_template_config_overlay_animation_animation_fade.rs"
        );
        include!(
            "types/transcoder/job_template_config_overlay_animation_animation_fade_xy.rs"
        );
        include!("types/transcoder/job_template_config_overlay_image.rs");
        include!("types/transcoder/job_template_config_pubsub_destination.rs");
    }
    pub mod vertex {
        include!("types/vertex/ai_dataset_encryption_spec.rs");
        include!("types/vertex/ai_deployment_resource_pool_dedicated_resources.rs");
        include!(
            "types/vertex/ai_deployment_resource_pool_dedicated_resources_autoscaling_metric_spec.rs"
        );
        include!(
            "types/vertex/ai_deployment_resource_pool_dedicated_resources_machine_spec.rs"
        );
        include!("types/vertex/ai_endpoint_deployed_model.rs");
        include!("types/vertex/ai_endpoint_deployed_model_automatic_resource.rs");
        include!("types/vertex/ai_endpoint_deployed_model_dedicated_resource.rs");
        include!(
            "types/vertex/ai_endpoint_deployed_model_dedicated_resource_autoscaling_metric_spec.rs"
        );
        include!(
            "types/vertex/ai_endpoint_deployed_model_dedicated_resource_machine_spec.rs"
        );
        include!("types/vertex/ai_endpoint_deployed_model_private_endpoint.rs");
        include!("types/vertex/ai_endpoint_encryption_spec.rs");
        include!("types/vertex/ai_endpoint_iam_binding_condition.rs");
        include!("types/vertex/ai_endpoint_iam_member_condition.rs");
        include!("types/vertex/ai_endpoint_predict_request_response_logging_config.rs");
        include!(
            "types/vertex/ai_endpoint_predict_request_response_logging_config_bigquery_destination.rs"
        );
        include!("types/vertex/ai_endpoint_private_service_connect_config.rs");
        include!("types/vertex/ai_feature_group_big_query.rs");
        include!("types/vertex/ai_feature_group_big_query_big_query_source.rs");
        include!("types/vertex/ai_feature_online_store_bigtable.rs");
        include!("types/vertex/ai_feature_online_store_bigtable_auto_scaling.rs");
        include!("types/vertex/ai_feature_online_store_dedicated_serving_endpoint.rs");
        include!(
            "types/vertex/ai_feature_online_store_dedicated_serving_endpoint_private_service_connect_config.rs"
        );
        include!("types/vertex/ai_feature_online_store_embedding_management.rs");
        include!("types/vertex/ai_feature_online_store_featureview_big_query_source.rs");
        include!(
            "types/vertex/ai_feature_online_store_featureview_feature_registry_source.rs"
        );
        include!(
            "types/vertex/ai_feature_online_store_featureview_feature_registry_source_feature_group.rs"
        );
        include!("types/vertex/ai_feature_online_store_featureview_sync_config.rs");
        include!(
            "types/vertex/ai_feature_online_store_featureview_vector_search_config.rs"
        );
        include!(
            "types/vertex/ai_feature_online_store_featureview_vector_search_config_brute_force_config.rs"
        );
        include!(
            "types/vertex/ai_feature_online_store_featureview_vector_search_config_tree_ah_config.rs"
        );
        include!("types/vertex/ai_feature_online_store_optimized.rs");
        include!("types/vertex/ai_feature_store_encryption_spec.rs");
        include!("types/vertex/ai_feature_store_entity_type_iam_binding_condition.rs");
        include!("types/vertex/ai_feature_store_entity_type_iam_member_condition.rs");
        include!("types/vertex/ai_feature_store_entity_type_monitoring_config.rs");
        include!(
            "types/vertex/ai_feature_store_entity_type_monitoring_config_categorical_threshold_config.rs"
        );
        include!(
            "types/vertex/ai_feature_store_entity_type_monitoring_config_import_features_analysis.rs"
        );
        include!(
            "types/vertex/ai_feature_store_entity_type_monitoring_config_numerical_threshold_config.rs"
        );
        include!(
            "types/vertex/ai_feature_store_entity_type_monitoring_config_snapshot_analysis.rs"
        );
        include!("types/vertex/ai_feature_store_iam_binding_condition.rs");
        include!("types/vertex/ai_feature_store_iam_member_condition.rs");
        include!("types/vertex/ai_feature_store_online_serving_config.rs");
        include!("types/vertex/ai_feature_store_online_serving_config_scaling.rs");
        include!("types/vertex/ai_index_deployed_index.rs");
        include!("types/vertex/ai_index_endpoint_deployed_index_automatic_resources.rs");
        include!("types/vertex/ai_index_endpoint_deployed_index_dedicated_resources.rs");
        include!(
            "types/vertex/ai_index_endpoint_deployed_index_dedicated_resources_machine_spec.rs"
        );
        include!(
            "types/vertex/ai_index_endpoint_deployed_index_deployed_index_auth_config.rs"
        );
        include!(
            "types/vertex/ai_index_endpoint_deployed_index_deployed_index_auth_config_auth_provider.rs"
        );
        include!("types/vertex/ai_index_endpoint_deployed_index_private_endpoint.rs");
        include!(
            "types/vertex/ai_index_endpoint_deployed_index_private_endpoint_psc_automated_endpoint.rs"
        );
        include!("types/vertex/ai_index_endpoint_private_service_connect_config.rs");
        include!("types/vertex/ai_index_index_stat.rs");
        include!("types/vertex/ai_index_metadata.rs");
        include!("types/vertex/ai_index_metadata_config.rs");
        include!("types/vertex/ai_index_metadata_config_algorithm_config.rs");
        include!(
            "types/vertex/ai_index_metadata_config_algorithm_config_brute_force_config.rs"
        );
        include!(
            "types/vertex/ai_index_metadata_config_algorithm_config_tree_ah_config.rs"
        );
        include!("types/vertex/ai_metadata_store_encryption_spec.rs");
        include!("types/vertex/ai_metadata_store_state.rs");
        include!("types/vertex/ai_tensorboard_encryption_spec.rs");
        include!("types/vertex/get_ai_index_deployed_index.rs");
        include!("types/vertex/get_ai_index_index_stat.rs");
        include!("types/vertex/get_ai_index_metadata.rs");
        include!("types/vertex/get_ai_index_metadata_config.rs");
        include!("types/vertex/get_ai_index_metadata_config_algorithm_config.rs");
        include!(
            "types/vertex/get_ai_index_metadata_config_algorithm_config_brute_force_config.rs"
        );
        include!(
            "types/vertex/get_ai_index_metadata_config_algorithm_config_tree_ah_config.rs"
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
#[cfg(target_arch = "wasm32")]
pub static PULUMI_WASM_PROVIDER_GCP: [u8; 45] = *b"{\"version\":\"8.12.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> &'static str {
    "8.12.1"
}
