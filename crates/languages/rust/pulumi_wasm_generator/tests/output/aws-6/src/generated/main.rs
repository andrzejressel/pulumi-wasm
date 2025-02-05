pub mod controltower {
    include!("resources/controltower/control_tower_control.rs");
    include!("resources/controltower/landing_zone.rs");
}
pub mod costexplorer {
    include!("resources/costexplorer/anomaly_monitor.rs");
    include!("resources/costexplorer/anomaly_subscription.rs");
    include!("resources/costexplorer/cost_allocation_tag.rs");
    include!("resources/costexplorer/cost_category.rs");
}
pub mod costoptimizationhub {
    include!("resources/costoptimizationhub/enrollment_status.rs");
    include!("resources/costoptimizationhub/preferences.rs");
}
pub mod cur {
    include!("resources/cur/report_definition.rs");
}
pub mod customerprofiles {
    include!("resources/customerprofiles/domain.rs");
    include!("resources/customerprofiles/profile.rs");
}
pub mod dataexchange {
    include!("resources/dataexchange/data_set.rs");
    include!("resources/dataexchange/revision.rs");
}
pub mod datapipeline {
    include!("resources/datapipeline/pipeline.rs");
    include!("resources/datapipeline/pipeline_definition.rs");
}
pub mod datasync {
    include!("resources/datasync/agent.rs");
    include!("resources/datasync/efs_location.rs");
    include!("resources/datasync/fsx_open_zfs_file_system.rs");
    include!("resources/datasync/location_azure_blob.rs");
    include!("resources/datasync/location_fsx_lustre.rs");
    include!("resources/datasync/location_fsx_ontap_file_system.rs");
    include!("resources/datasync/location_fsx_windows.rs");
    include!("resources/datasync/location_hdfs.rs");
    include!("resources/datasync/location_object_storage.rs");
    include!("resources/datasync/location_smb.rs");
    include!("resources/datasync/nfs_location.rs");
    include!("resources/datasync/s_3_location.rs");
    include!("resources/datasync/task.rs");
}
pub mod datazone {
    include!("resources/datazone/asset_type.rs");
    include!("resources/datazone/domain.rs");
    include!("resources/datazone/environment.rs");
    include!("resources/datazone/environment_blueprint_configuration.rs");
    include!("resources/datazone/environment_profile.rs");
    include!("resources/datazone/form_type.rs");
    include!("resources/datazone/glossary.rs");
    include!("resources/datazone/glossary_term.rs");
    include!("resources/datazone/project.rs");
    include!("resources/datazone/user_profile.rs");
}
pub mod dax {
    include!("resources/dax/cluster.rs");
    include!("resources/dax/parameter_group.rs");
    include!("resources/dax/subnet_group.rs");
}
pub mod functions {
    pub mod controltower {
        include!("functions/controltower/get_controls.rs");
    }
    pub mod costexplorer {
        include!("functions/costexplorer/get_cost_category.rs");
        include!("functions/costexplorer/get_tags.rs");
    }
    pub mod cur {
        include!("functions/cur/get_report_definition.rs");
    }
    pub mod datapipeline {
        include!("functions/datapipeline/get_pipeline.rs");
        include!("functions/datapipeline/get_pipeline_definition.rs");
    }
    pub mod datazone {
        include!("functions/datazone/get_environment_blueprint.rs");
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
    pub mod controltower {
        include!("types/controltower/control_tower_control_parameter.rs");
        include!("types/controltower/landing_zone_drift_status.rs");
    }
    pub mod costexplorer {
        include!("types/costexplorer/anomaly_subscription_subscriber.rs");
        include!("types/costexplorer/anomaly_subscription_threshold_expression.rs");
        include!("types/costexplorer/anomaly_subscription_threshold_expression_and.rs");
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_and_cost_category.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_and_dimension.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_and_tags.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_cost_category.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_dimension.rs"
        );
        include!("types/costexplorer/anomaly_subscription_threshold_expression_not.rs");
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_not_cost_category.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_not_dimension.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_not_tags.rs"
        );
        include!("types/costexplorer/anomaly_subscription_threshold_expression_or.rs");
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_or_cost_category.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_or_dimension.rs"
        );
        include!(
            "types/costexplorer/anomaly_subscription_threshold_expression_or_tags.rs"
        );
        include!("types/costexplorer/anomaly_subscription_threshold_expression_tags.rs");
        include!("types/costexplorer/cost_category_rule.rs");
        include!("types/costexplorer/cost_category_rule_inherited_value.rs");
        include!("types/costexplorer/cost_category_rule_rule.rs");
        include!("types/costexplorer/cost_category_rule_rule_and.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_and.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_and_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_and_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_and_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_not.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_not_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_not_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_not_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_or.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_or_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_or_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_or_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_and_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_not.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_and.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_and_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_and_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_and_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_not.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_not_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_not_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_not_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_or.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_or_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_or_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_or_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_not_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_or.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_and.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_and_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_and_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_and_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_not.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_not_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_not_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_not_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_or.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_or_cost_category.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_or_dimension.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_or_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_or_tags.rs");
        include!("types/costexplorer/cost_category_rule_rule_tags.rs");
        include!("types/costexplorer/cost_category_split_charge_rule.rs");
        include!("types/costexplorer/cost_category_split_charge_rule_parameter.rs");
        include!("types/costexplorer/get_cost_category_rule.rs");
        include!("types/costexplorer/get_cost_category_rule_inherited_value.rs");
        include!("types/costexplorer/get_cost_category_rule_rule.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_and.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_and_and_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_and_and_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_and_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_cost_category.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_not.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_and_not_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_and_not_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_not_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_or.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_and_or_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_and_or_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_or_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_and_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_cost_category.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_and.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_not_and_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_not_and_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_and_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_cost_category.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_not.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_not_not_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_not_not_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_not_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_or.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_not_or_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_not_or_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_or_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_not_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_and.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_or_and_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_or_and_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_and_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_cost_category.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_not.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_or_not_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_or_not_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_not_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_or.rs");
        include!(
            "types/costexplorer/get_cost_category_rule_rule_or_or_cost_category.rs"
        );
        include!("types/costexplorer/get_cost_category_rule_rule_or_or_dimension.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_or_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_or_tag.rs");
        include!("types/costexplorer/get_cost_category_rule_rule_tag.rs");
        include!("types/costexplorer/get_cost_category_split_charge_rule.rs");
        include!("types/costexplorer/get_cost_category_split_charge_rule_parameter.rs");
        include!("types/costexplorer/get_tags_filter.rs");
        include!("types/costexplorer/get_tags_filter_and.rs");
        include!("types/costexplorer/get_tags_filter_and_cost_category.rs");
        include!("types/costexplorer/get_tags_filter_and_dimension.rs");
        include!("types/costexplorer/get_tags_filter_and_tags.rs");
        include!("types/costexplorer/get_tags_filter_cost_category.rs");
        include!("types/costexplorer/get_tags_filter_dimension.rs");
        include!("types/costexplorer/get_tags_filter_not.rs");
        include!("types/costexplorer/get_tags_filter_not_cost_category.rs");
        include!("types/costexplorer/get_tags_filter_not_dimension.rs");
        include!("types/costexplorer/get_tags_filter_not_tags.rs");
        include!("types/costexplorer/get_tags_filter_or.rs");
        include!("types/costexplorer/get_tags_filter_or_cost_category.rs");
        include!("types/costexplorer/get_tags_filter_or_dimension.rs");
        include!("types/costexplorer/get_tags_filter_or_tags.rs");
        include!("types/costexplorer/get_tags_filter_tags.rs");
        include!("types/costexplorer/get_tags_sort_by.rs");
        include!("types/costexplorer/get_tags_time_period.rs");
    }
    pub mod customerprofiles {
        include!("types/customerprofiles/domain_matching.rs");
        include!("types/customerprofiles/domain_matching_auto_merging.rs");
        include!(
            "types/customerprofiles/domain_matching_auto_merging_conflict_resolution.rs"
        );
        include!("types/customerprofiles/domain_matching_auto_merging_consolidation.rs");
        include!("types/customerprofiles/domain_matching_exporting_config.rs");
        include!(
            "types/customerprofiles/domain_matching_exporting_config_s_3_exporting.rs"
        );
        include!("types/customerprofiles/domain_matching_job_schedule.rs");
        include!("types/customerprofiles/domain_rule_based_matching.rs");
        include!(
            "types/customerprofiles/domain_rule_based_matching_attribute_types_selector.rs"
        );
        include!(
            "types/customerprofiles/domain_rule_based_matching_conflict_resolution.rs"
        );
        include!(
            "types/customerprofiles/domain_rule_based_matching_exporting_config.rs"
        );
        include!(
            "types/customerprofiles/domain_rule_based_matching_exporting_config_s_3_exporting.rs"
        );
        include!("types/customerprofiles/domain_rule_based_matching_matching_rule.rs");
        include!("types/customerprofiles/profile_address.rs");
        include!("types/customerprofiles/profile_billing_address.rs");
        include!("types/customerprofiles/profile_mailing_address.rs");
        include!("types/customerprofiles/profile_shipping_address.rs");
    }
    pub mod datapipeline {
        include!("types/datapipeline/pipeline_definition_parameter_object.rs");
        include!("types/datapipeline/pipeline_definition_parameter_object_attribute.rs");
        include!("types/datapipeline/pipeline_definition_parameter_value.rs");
        include!("types/datapipeline/pipeline_definition_pipeline_object.rs");
        include!("types/datapipeline/pipeline_definition_pipeline_object_field.rs");
        include!("types/datapipeline/get_pipeline_definition_parameter_object.rs");
        include!(
            "types/datapipeline/get_pipeline_definition_parameter_object_attribute.rs"
        );
        include!("types/datapipeline/get_pipeline_definition_parameter_value.rs");
        include!("types/datapipeline/get_pipeline_definition_pipeline_object.rs");
        include!("types/datapipeline/get_pipeline_definition_pipeline_object_field.rs");
    }
    pub mod datasync {
        include!("types/datasync/efs_location_ec_2_config.rs");
        include!("types/datasync/fsx_open_zfs_file_system_protocol.rs");
        include!("types/datasync/fsx_open_zfs_file_system_protocol_nfs.rs");
        include!(
            "types/datasync/fsx_open_zfs_file_system_protocol_nfs_mount_options.rs"
        );
        include!("types/datasync/location_azure_blob_sas_configuration.rs");
        include!("types/datasync/location_fsx_ontap_file_system_protocol.rs");
        include!("types/datasync/location_fsx_ontap_file_system_protocol_nfs.rs");
        include!(
            "types/datasync/location_fsx_ontap_file_system_protocol_nfs_mount_options.rs"
        );
        include!("types/datasync/location_fsx_ontap_file_system_protocol_smb.rs");
        include!(
            "types/datasync/location_fsx_ontap_file_system_protocol_smb_mount_options.rs"
        );
        include!("types/datasync/location_hdfs_name_node.rs");
        include!("types/datasync/location_hdfs_qop_configuration.rs");
        include!("types/datasync/location_smb_mount_options.rs");
        include!("types/datasync/nfs_location_mount_options.rs");
        include!("types/datasync/nfs_location_on_prem_config.rs");
        include!("types/datasync/s_3_location_s_3_config.rs");
        include!("types/datasync/task_excludes.rs");
        include!("types/datasync/task_includes.rs");
        include!("types/datasync/task_options.rs");
        include!("types/datasync/task_schedule.rs");
        include!("types/datasync/task_task_report_config.rs");
        include!("types/datasync/task_task_report_config_report_overrides.rs");
        include!("types/datasync/task_task_report_config_s_3_destination.rs");
    }
    pub mod datazone {
        include!("types/datazone/asset_type_forms_input.rs");
        include!("types/datazone/asset_type_timeouts.rs");
        include!("types/datazone/domain_single_sign_on.rs");
        include!("types/datazone/domain_timeouts.rs");
        include!("types/datazone/environment_last_deployment.rs");
        include!("types/datazone/environment_last_deployment_failure_reason.rs");
        include!("types/datazone/environment_profile_user_parameter.rs");
        include!("types/datazone/environment_provisioned_resource.rs");
        include!("types/datazone/environment_timeouts.rs");
        include!("types/datazone/environment_user_parameter.rs");
        include!("types/datazone/form_type_import.rs");
        include!("types/datazone/form_type_model.rs");
        include!("types/datazone/form_type_timeouts.rs");
        include!("types/datazone/glossary_term_term_relations.rs");
        include!("types/datazone/glossary_term_timeouts.rs");
        include!("types/datazone/project_failure_reason.rs");
        include!("types/datazone/project_timeouts.rs");
        include!("types/datazone/user_profile_detail.rs");
        include!("types/datazone/user_profile_detail_iam.rs");
        include!("types/datazone/user_profile_detail_sso.rs");
        include!("types/datazone/user_profile_timeouts.rs");
    }
    pub mod dax {
        include!("types/dax/cluster_node.rs");
        include!("types/dax/cluster_server_side_encryption.rs");
        include!("types/dax/parameter_group_parameter.rs");
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
        pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
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
