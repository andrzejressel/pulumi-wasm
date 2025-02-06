pub mod assuredworkloads {
    include!("resources/assuredworkloads/workload.rs");
}
pub mod backupdisasterrecovery {
    include!("resources/backupdisasterrecovery/backup_plan.rs");
    include!("resources/backupdisasterrecovery/backup_plan_association.rs");
    include!("resources/backupdisasterrecovery/backup_vault.rs");
    include!("resources/backupdisasterrecovery/management_server.rs");
}
pub mod beyondcorp {
    include!("resources/beyondcorp/app_connection.rs");
    include!("resources/beyondcorp/app_connector.rs");
    include!("resources/beyondcorp/app_gateway.rs");
}
pub mod biglake {
    include!("resources/biglake/catalog.rs");
    include!("resources/biglake/database.rs");
    include!("resources/biglake/table.rs");
}
pub mod bigquery {
    include!("resources/bigquery/app_profile.rs");
    include!("resources/bigquery/bi_reservation.rs");
    include!("resources/bigquery/capacity_commitment.rs");
    include!("resources/bigquery/connection.rs");
    include!("resources/bigquery/connection_iam_binding.rs");
    include!("resources/bigquery/connection_iam_member.rs");
    include!("resources/bigquery/connection_iam_policy.rs");
    include!("resources/bigquery/data_transfer_config.rs");
    include!("resources/bigquery/dataset.rs");
    include!("resources/bigquery/dataset_access.rs");
    include!("resources/bigquery/dataset_iam_binding.rs");
    include!("resources/bigquery/dataset_iam_member.rs");
    include!("resources/bigquery/dataset_iam_policy.rs");
    include!("resources/bigquery/iam_binding.rs");
    include!("resources/bigquery/iam_member.rs");
    include!("resources/bigquery/iam_policy.rs");
    include!("resources/bigquery/job.rs");
    include!("resources/bigquery/reservation.rs");
    include!("resources/bigquery/reservation_assignment.rs");
    include!("resources/bigquery/routine.rs");
    include!("resources/bigquery/table.rs");
}
pub mod bigqueryanalyticshub {
    include!("resources/bigqueryanalyticshub/data_exchange.rs");
    include!("resources/bigqueryanalyticshub/data_exchange_iam_binding.rs");
    include!("resources/bigqueryanalyticshub/data_exchange_iam_member.rs");
    include!("resources/bigqueryanalyticshub/data_exchange_iam_policy.rs");
    include!("resources/bigqueryanalyticshub/listing.rs");
    include!("resources/bigqueryanalyticshub/listing_iam_binding.rs");
    include!("resources/bigqueryanalyticshub/listing_iam_member.rs");
    include!("resources/bigqueryanalyticshub/listing_iam_policy.rs");
}
pub mod bigquerydatapolicy {
    include!("resources/bigquerydatapolicy/data_policy.rs");
    include!("resources/bigquerydatapolicy/data_policy_iam_binding.rs");
    include!("resources/bigquerydatapolicy/data_policy_iam_member.rs");
    include!("resources/bigquerydatapolicy/data_policy_iam_policy.rs");
}
pub mod bigtable {
    include!("resources/bigtable/authorized_view.rs");
    include!("resources/bigtable/gc_policy.rs");
    include!("resources/bigtable/instance.rs");
    include!("resources/bigtable/instance_iam_binding.rs");
    include!("resources/bigtable/instance_iam_member.rs");
    include!("resources/bigtable/instance_iam_policy.rs");
    include!("resources/bigtable/table.rs");
    include!("resources/bigtable/table_iam_binding.rs");
    include!("resources/bigtable/table_iam_member.rs");
    include!("resources/bigtable/table_iam_policy.rs");
}
pub mod billing {
    include!("resources/billing/account_iam_binding.rs");
    include!("resources/billing/account_iam_member.rs");
    include!("resources/billing/account_iam_policy.rs");
    include!("resources/billing/budget.rs");
    include!("resources/billing/project_info.rs");
    include!("resources/billing/sub_account.rs");
}
pub mod binaryauthorization {
    include!("resources/binaryauthorization/attestor.rs");
    include!("resources/binaryauthorization/attestor_iam_binding.rs");
    include!("resources/binaryauthorization/attestor_iam_member.rs");
    include!("resources/binaryauthorization/attestor_iam_policy.rs");
    include!("resources/binaryauthorization/policy.rs");
}
pub mod functions {
    pub mod backupdisasterrecovery {
        include!("functions/backupdisasterrecovery/get_backup.rs");
        include!("functions/backupdisasterrecovery/get_backup_plan.rs");
        include!("functions/backupdisasterrecovery/get_backup_plan_association.rs");
        include!("functions/backupdisasterrecovery/get_backup_vault.rs");
        include!("functions/backupdisasterrecovery/get_data_source.rs");
        include!("functions/backupdisasterrecovery/get_management_server.rs");
    }
    pub mod beyondcorp {
        include!("functions/beyondcorp/get_app_connection.rs");
        include!("functions/beyondcorp/get_app_connector.rs");
        include!("functions/beyondcorp/get_app_gateway.rs");
    }
    pub mod bigquery {
        include!("functions/bigquery/get_connection_iam_policy.rs");
        include!("functions/bigquery/get_dataset.rs");
        include!("functions/bigquery/get_dataset_iam_policy.rs");
        include!("functions/bigquery/get_default_service_account.rs");
        include!("functions/bigquery/get_table_iam_policy.rs");
        include!("functions/bigquery/get_tables.rs");
    }
    pub mod bigqueryanalyticshub {
        include!("functions/bigqueryanalyticshub/get_data_exchange_iam_policy.rs");
        include!("functions/bigqueryanalyticshub/get_listing_iam_policy.rs");
    }
    pub mod bigquerydatapolicy {
        include!("functions/bigquerydatapolicy/get_iam_policy.rs");
    }
    pub mod bigtable {
        include!("functions/bigtable/get_instance_iam_policy.rs");
        include!("functions/bigtable/get_table_iam_policy.rs");
    }
    pub mod billing {
        include!("functions/billing/get_account_iam_policy.rs");
    }
    pub mod binaryauthorization {
        include!("functions/binaryauthorization/get_attestor_iam_policy.rs");
    }
}
pub mod types {
    pub mod assuredworkloads {
        include!("types/assuredworkloads/workload_compliance_status.rs");
        include!("types/assuredworkloads/workload_ekm_provisioning_response.rs");
        include!("types/assuredworkloads/workload_kms_settings.rs");
        include!("types/assuredworkloads/workload_partner_permissions.rs");
        include!("types/assuredworkloads/workload_resource.rs");
        include!("types/assuredworkloads/workload_resource_setting.rs");
        include!("types/assuredworkloads/workload_saa_enrollment_response.rs");
        include!("types/assuredworkloads/workload_workload_options.rs");
    }
    pub mod backupdisasterrecovery {
        include!(
            "types/backupdisasterrecovery/backup_plan_association_rules_config_info.rs"
        );
        include!(
            "types/backupdisasterrecovery/backup_plan_association_rules_config_info_last_backup_error.rs"
        );
        include!("types/backupdisasterrecovery/backup_plan_backup_rule.rs");
        include!(
            "types/backupdisasterrecovery/backup_plan_backup_rule_standard_schedule.rs"
        );
        include!(
            "types/backupdisasterrecovery/backup_plan_backup_rule_standard_schedule_backup_window.rs"
        );
        include!(
            "types/backupdisasterrecovery/backup_plan_backup_rule_standard_schedule_week_day_of_month.rs"
        );
        include!("types/backupdisasterrecovery/management_server_management_uri.rs");
        include!("types/backupdisasterrecovery/management_server_network.rs");
        include!("types/backupdisasterrecovery/get_backup_backup.rs");
        include!(
            "types/backupdisasterrecovery/get_backup_plan_association_rules_config_info.rs"
        );
        include!(
            "types/backupdisasterrecovery/get_backup_plan_association_rules_config_info_last_backup_error.rs"
        );
        include!("types/backupdisasterrecovery/get_backup_plan_backup_rule.rs");
        include!(
            "types/backupdisasterrecovery/get_backup_plan_backup_rule_standard_schedule.rs"
        );
        include!(
            "types/backupdisasterrecovery/get_backup_plan_backup_rule_standard_schedule_backup_window.rs"
        );
        include!(
            "types/backupdisasterrecovery/get_backup_plan_backup_rule_standard_schedule_week_day_of_month.rs"
        );
        include!("types/backupdisasterrecovery/get_data_source_backup_config_info.rs");
        include!(
            "types/backupdisasterrecovery/get_data_source_backup_config_info_backup_appliance_backup_config.rs"
        );
        include!(
            "types/backupdisasterrecovery/get_data_source_backup_config_info_gcp_backup_config.rs"
        );
        include!(
            "types/backupdisasterrecovery/get_data_source_data_source_backup_appliance_application.rs"
        );
        include!(
            "types/backupdisasterrecovery/get_data_source_data_source_gcp_resource.rs"
        );
        include!(
            "types/backupdisasterrecovery/get_data_source_data_source_gcp_resource_compute_instance_data_source_property.rs"
        );
        include!("types/backupdisasterrecovery/get_management_server_management_uri.rs");
        include!("types/backupdisasterrecovery/get_management_server_network.rs");
    }
    pub mod beyondcorp {
        include!("types/beyondcorp/app_connection_application_endpoint.rs");
        include!("types/beyondcorp/app_connection_gateway.rs");
        include!("types/beyondcorp/app_connector_principal_info.rs");
        include!("types/beyondcorp/app_connector_principal_info_service_account.rs");
        include!("types/beyondcorp/app_gateway_allocated_connection.rs");
        include!("types/beyondcorp/get_app_connection_application_endpoint.rs");
        include!("types/beyondcorp/get_app_connection_gateway.rs");
        include!("types/beyondcorp/get_app_connector_principal_info.rs");
        include!("types/beyondcorp/get_app_connector_principal_info_service_account.rs");
        include!("types/beyondcorp/get_app_gateway_allocated_connection.rs");
    }
    pub mod biglake {
        include!("types/biglake/database_hive_options.rs");
        include!("types/biglake/table_hive_options.rs");
        include!("types/biglake/table_hive_options_storage_descriptor.rs");
    }
    pub mod bigquery {
        include!("types/bigquery/app_profile_data_boost_isolation_read_only.rs");
        include!("types/bigquery/app_profile_single_cluster_routing.rs");
        include!("types/bigquery/app_profile_standard_isolation.rs");
        include!("types/bigquery/bi_reservation_preferred_table.rs");
        include!("types/bigquery/connection_aws.rs");
        include!("types/bigquery/connection_aws_access_role.rs");
        include!("types/bigquery/connection_azure.rs");
        include!("types/bigquery/connection_cloud_resource.rs");
        include!("types/bigquery/connection_cloud_spanner.rs");
        include!("types/bigquery/connection_cloud_sql.rs");
        include!("types/bigquery/connection_cloud_sql_credential.rs");
        include!("types/bigquery/connection_iam_binding_condition.rs");
        include!("types/bigquery/connection_iam_member_condition.rs");
        include!("types/bigquery/connection_spark.rs");
        include!("types/bigquery/connection_spark_metastore_service_config.rs");
        include!("types/bigquery/connection_spark_spark_history_server_config.rs");
        include!("types/bigquery/data_transfer_config_email_preferences.rs");
        include!("types/bigquery/data_transfer_config_encryption_configuration.rs");
        include!("types/bigquery/data_transfer_config_schedule_options.rs");
        include!("types/bigquery/data_transfer_config_sensitive_params.rs");
        include!("types/bigquery/dataset_access.rs");
        include!("types/bigquery/dataset_access_authorized_dataset.rs");
        include!("types/bigquery/dataset_access_authorized_dataset_dataset.rs");
        include!("types/bigquery/dataset_access_dataset.rs");
        include!("types/bigquery/dataset_access_dataset_dataset.rs");
        include!("types/bigquery/dataset_access_routine.rs");
        include!("types/bigquery/dataset_access_view.rs");
        include!("types/bigquery/dataset_default_encryption_configuration.rs");
        include!("types/bigquery/dataset_external_catalog_dataset_options.rs");
        include!("types/bigquery/dataset_external_dataset_reference.rs");
        include!("types/bigquery/dataset_iam_binding_condition.rs");
        include!("types/bigquery/dataset_iam_member_condition.rs");
        include!("types/bigquery/iam_binding_condition.rs");
        include!("types/bigquery/iam_member_condition.rs");
        include!("types/bigquery/job_copy.rs");
        include!("types/bigquery/job_copy_destination_encryption_configuration.rs");
        include!("types/bigquery/job_copy_destination_table.rs");
        include!("types/bigquery/job_copy_source_table.rs");
        include!("types/bigquery/job_extract.rs");
        include!("types/bigquery/job_extract_source_model.rs");
        include!("types/bigquery/job_extract_source_table.rs");
        include!("types/bigquery/job_load.rs");
        include!("types/bigquery/job_load_destination_encryption_configuration.rs");
        include!("types/bigquery/job_load_destination_table.rs");
        include!("types/bigquery/job_load_parquet_options.rs");
        include!("types/bigquery/job_load_time_partitioning.rs");
        include!("types/bigquery/job_query.rs");
        include!("types/bigquery/job_query_default_dataset.rs");
        include!("types/bigquery/job_query_destination_encryption_configuration.rs");
        include!("types/bigquery/job_query_destination_table.rs");
        include!("types/bigquery/job_query_script_options.rs");
        include!("types/bigquery/job_query_user_defined_function_resource.rs");
        include!("types/bigquery/job_status.rs");
        include!("types/bigquery/job_status_error.rs");
        include!("types/bigquery/job_status_error_result.rs");
        include!("types/bigquery/reservation_autoscale.rs");
        include!("types/bigquery/routine_argument.rs");
        include!("types/bigquery/routine_remote_function_options.rs");
        include!("types/bigquery/routine_spark_options.rs");
        include!("types/bigquery/table_biglake_configuration.rs");
        include!("types/bigquery/table_encryption_configuration.rs");
        include!("types/bigquery/table_external_data_configuration.rs");
        include!("types/bigquery/table_external_data_configuration_avro_options.rs");
        include!("types/bigquery/table_external_data_configuration_bigtable_options.rs");
        include!(
            "types/bigquery/table_external_data_configuration_bigtable_options_column_family.rs"
        );
        include!(
            "types/bigquery/table_external_data_configuration_bigtable_options_column_family_column.rs"
        );
        include!("types/bigquery/table_external_data_configuration_csv_options.rs");
        include!(
            "types/bigquery/table_external_data_configuration_google_sheets_options.rs"
        );
        include!(
            "types/bigquery/table_external_data_configuration_hive_partitioning_options.rs"
        );
        include!("types/bigquery/table_external_data_configuration_json_options.rs");
        include!("types/bigquery/table_external_data_configuration_parquet_options.rs");
        include!("types/bigquery/table_materialized_view.rs");
        include!("types/bigquery/table_range_partitioning.rs");
        include!("types/bigquery/table_range_partitioning_range.rs");
        include!("types/bigquery/table_table_constraints.rs");
        include!("types/bigquery/table_table_constraints_foreign_key.rs");
        include!(
            "types/bigquery/table_table_constraints_foreign_key_column_references.rs"
        );
        include!(
            "types/bigquery/table_table_constraints_foreign_key_referenced_table.rs"
        );
        include!("types/bigquery/table_table_constraints_primary_key.rs");
        include!("types/bigquery/table_table_replication_info.rs");
        include!("types/bigquery/table_time_partitioning.rs");
        include!("types/bigquery/table_view.rs");
        include!("types/bigquery/get_dataset_access.rs");
        include!("types/bigquery/get_dataset_access_dataset.rs");
        include!("types/bigquery/get_dataset_access_dataset_dataset.rs");
        include!("types/bigquery/get_dataset_access_routine.rs");
        include!("types/bigquery/get_dataset_access_view.rs");
        include!("types/bigquery/get_dataset_default_encryption_configuration.rs");
        include!("types/bigquery/get_dataset_external_catalog_dataset_option.rs");
        include!("types/bigquery/get_dataset_external_dataset_reference.rs");
        include!("types/bigquery/get_tables_table.rs");
    }
    pub mod bigqueryanalyticshub {
        include!("types/bigqueryanalyticshub/data_exchange_iam_binding_condition.rs");
        include!("types/bigqueryanalyticshub/data_exchange_iam_member_condition.rs");
        include!(
            "types/bigqueryanalyticshub/data_exchange_sharing_environment_config.rs"
        );
        include!(
            "types/bigqueryanalyticshub/data_exchange_sharing_environment_config_dcr_exchange_config.rs"
        );
        include!(
            "types/bigqueryanalyticshub/data_exchange_sharing_environment_config_default_exchange_config.rs"
        );
        include!("types/bigqueryanalyticshub/listing_bigquery_dataset.rs");
        include!(
            "types/bigqueryanalyticshub/listing_bigquery_dataset_selected_resource.rs"
        );
        include!("types/bigqueryanalyticshub/listing_data_provider.rs");
        include!("types/bigqueryanalyticshub/listing_iam_binding_condition.rs");
        include!("types/bigqueryanalyticshub/listing_iam_member_condition.rs");
        include!("types/bigqueryanalyticshub/listing_publisher.rs");
        include!("types/bigqueryanalyticshub/listing_restricted_export_config.rs");
    }
    pub mod bigquerydatapolicy {
        include!("types/bigquerydatapolicy/data_policy_data_masking_policy.rs");
        include!("types/bigquerydatapolicy/data_policy_iam_binding_condition.rs");
        include!("types/bigquerydatapolicy/data_policy_iam_member_condition.rs");
    }
    pub mod bigtable {
        include!("types/bigtable/authorized_view_subset_view.rs");
        include!("types/bigtable/authorized_view_subset_view_family_subset.rs");
        include!("types/bigtable/gc_policy_max_age.rs");
        include!("types/bigtable/gc_policy_max_version.rs");
        include!("types/bigtable/instance_cluster.rs");
        include!("types/bigtable/instance_cluster_autoscaling_config.rs");
        include!("types/bigtable/instance_iam_binding_condition.rs");
        include!("types/bigtable/instance_iam_member_condition.rs");
        include!("types/bigtable/table_automated_backup_policy.rs");
        include!("types/bigtable/table_column_family.rs");
        include!("types/bigtable/table_iam_binding_condition.rs");
        include!("types/bigtable/table_iam_member_condition.rs");
    }
    pub mod billing {
        include!("types/billing/account_iam_binding_condition.rs");
        include!("types/billing/account_iam_member_condition.rs");
        include!("types/billing/budget_all_updates_rule.rs");
        include!("types/billing/budget_amount.rs");
        include!("types/billing/budget_amount_specified_amount.rs");
        include!("types/billing/budget_budget_filter.rs");
        include!("types/billing/budget_budget_filter_custom_period.rs");
        include!("types/billing/budget_budget_filter_custom_period_end_date.rs");
        include!("types/billing/budget_budget_filter_custom_period_start_date.rs");
        include!("types/billing/budget_threshold_rule.rs");
    }
    pub mod binaryauthorization {
        include!("types/binaryauthorization/attestor_attestation_authority_note.rs");
        include!(
            "types/binaryauthorization/attestor_attestation_authority_note_public_key.rs"
        );
        include!(
            "types/binaryauthorization/attestor_attestation_authority_note_public_key_pkix_public_key.rs"
        );
        include!("types/binaryauthorization/attestor_iam_binding_condition.rs");
        include!("types/binaryauthorization/attestor_iam_member_condition.rs");
        include!("types/binaryauthorization/policy_admission_whitelist_pattern.rs");
        include!("types/binaryauthorization/policy_cluster_admission_rule.rs");
        include!("types/binaryauthorization/policy_default_admission_rule.rs");
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
#[link_section = "pulumi_wasm_provider::gcp"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_GCP: [u8; 45] = *b"{\"version\":\"8.12.1\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "8.12.1".to_string()
}
