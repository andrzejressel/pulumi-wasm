pub mod hdinsight {
    include!("resources/hdinsight/h_base_cluster.rs");
    include!("resources/hdinsight/hadoop_cluster.rs");
    include!("resources/hdinsight/interactive_query_cluster.rs");
    include!("resources/hdinsight/kafka_cluster.rs");
    include!("resources/hdinsight/spark_cluster.rs");
}
pub mod functions {
    pub mod hdinsight {
        include!("functions/hdinsight/get_cluster.rs");
    }
}
pub mod types {
    pub mod hdinsight {
        include!("types/hdinsight/h_base_cluster_component_version.rs");
        include!("types/hdinsight/h_base_cluster_compute_isolation.rs");
        include!("types/hdinsight/h_base_cluster_disk_encryption.rs");
        include!("types/hdinsight/h_base_cluster_extension.rs");
        include!("types/hdinsight/h_base_cluster_gateway.rs");
        include!("types/hdinsight/h_base_cluster_metastores.rs");
        include!("types/hdinsight/h_base_cluster_metastores_ambari.rs");
        include!("types/hdinsight/h_base_cluster_metastores_hive.rs");
        include!("types/hdinsight/h_base_cluster_metastores_oozie.rs");
        include!("types/hdinsight/h_base_cluster_monitor.rs");
        include!("types/hdinsight/h_base_cluster_network.rs");
        include!("types/hdinsight/h_base_cluster_private_link_configuration.rs");
        include!(
            "types/hdinsight/h_base_cluster_private_link_configuration_ip_configuration.rs"
        );
        include!("types/hdinsight/h_base_cluster_roles.rs");
        include!("types/hdinsight/h_base_cluster_roles_head_node.rs");
        include!("types/hdinsight/h_base_cluster_roles_head_node_script_action.rs");
        include!("types/hdinsight/h_base_cluster_roles_worker_node.rs");
        include!("types/hdinsight/h_base_cluster_roles_worker_node_autoscale.rs");
        include!(
            "types/hdinsight/h_base_cluster_roles_worker_node_autoscale_recurrence.rs"
        );
        include!(
            "types/hdinsight/h_base_cluster_roles_worker_node_autoscale_recurrence_schedule.rs"
        );
        include!("types/hdinsight/h_base_cluster_roles_worker_node_script_action.rs");
        include!("types/hdinsight/h_base_cluster_roles_zookeeper_node.rs");
        include!("types/hdinsight/h_base_cluster_roles_zookeeper_node_script_action.rs");
        include!("types/hdinsight/h_base_cluster_security_profile.rs");
        include!("types/hdinsight/h_base_cluster_storage_account.rs");
        include!("types/hdinsight/h_base_cluster_storage_account_gen_2.rs");
        include!("types/hdinsight/hadoop_cluster_component_version.rs");
        include!("types/hdinsight/hadoop_cluster_compute_isolation.rs");
        include!("types/hdinsight/hadoop_cluster_disk_encryption.rs");
        include!("types/hdinsight/hadoop_cluster_extension.rs");
        include!("types/hdinsight/hadoop_cluster_gateway.rs");
        include!("types/hdinsight/hadoop_cluster_metastores.rs");
        include!("types/hdinsight/hadoop_cluster_metastores_ambari.rs");
        include!("types/hdinsight/hadoop_cluster_metastores_hive.rs");
        include!("types/hdinsight/hadoop_cluster_metastores_oozie.rs");
        include!("types/hdinsight/hadoop_cluster_monitor.rs");
        include!("types/hdinsight/hadoop_cluster_network.rs");
        include!("types/hdinsight/hadoop_cluster_private_link_configuration.rs");
        include!(
            "types/hdinsight/hadoop_cluster_private_link_configuration_ip_configuration.rs"
        );
        include!("types/hdinsight/hadoop_cluster_roles.rs");
        include!("types/hdinsight/hadoop_cluster_roles_edge_node.rs");
        include!("types/hdinsight/hadoop_cluster_roles_edge_node_https_endpoint.rs");
        include!(
            "types/hdinsight/hadoop_cluster_roles_edge_node_install_script_action.rs"
        );
        include!(
            "types/hdinsight/hadoop_cluster_roles_edge_node_uninstall_script_action.rs"
        );
        include!("types/hdinsight/hadoop_cluster_roles_head_node.rs");
        include!("types/hdinsight/hadoop_cluster_roles_head_node_script_action.rs");
        include!("types/hdinsight/hadoop_cluster_roles_worker_node.rs");
        include!("types/hdinsight/hadoop_cluster_roles_worker_node_autoscale.rs");
        include!(
            "types/hdinsight/hadoop_cluster_roles_worker_node_autoscale_capacity.rs"
        );
        include!(
            "types/hdinsight/hadoop_cluster_roles_worker_node_autoscale_recurrence.rs"
        );
        include!(
            "types/hdinsight/hadoop_cluster_roles_worker_node_autoscale_recurrence_schedule.rs"
        );
        include!("types/hdinsight/hadoop_cluster_roles_worker_node_script_action.rs");
        include!("types/hdinsight/hadoop_cluster_roles_zookeeper_node.rs");
        include!("types/hdinsight/hadoop_cluster_roles_zookeeper_node_script_action.rs");
        include!("types/hdinsight/hadoop_cluster_security_profile.rs");
        include!("types/hdinsight/hadoop_cluster_storage_account.rs");
        include!("types/hdinsight/hadoop_cluster_storage_account_gen_2.rs");
        include!("types/hdinsight/interactive_query_cluster_component_version.rs");
        include!("types/hdinsight/interactive_query_cluster_compute_isolation.rs");
        include!("types/hdinsight/interactive_query_cluster_disk_encryption.rs");
        include!("types/hdinsight/interactive_query_cluster_extension.rs");
        include!("types/hdinsight/interactive_query_cluster_gateway.rs");
        include!("types/hdinsight/interactive_query_cluster_metastores.rs");
        include!("types/hdinsight/interactive_query_cluster_metastores_ambari.rs");
        include!("types/hdinsight/interactive_query_cluster_metastores_hive.rs");
        include!("types/hdinsight/interactive_query_cluster_metastores_oozie.rs");
        include!("types/hdinsight/interactive_query_cluster_monitor.rs");
        include!("types/hdinsight/interactive_query_cluster_network.rs");
        include!(
            "types/hdinsight/interactive_query_cluster_private_link_configuration.rs"
        );
        include!(
            "types/hdinsight/interactive_query_cluster_private_link_configuration_ip_configuration.rs"
        );
        include!("types/hdinsight/interactive_query_cluster_roles.rs");
        include!("types/hdinsight/interactive_query_cluster_roles_head_node.rs");
        include!(
            "types/hdinsight/interactive_query_cluster_roles_head_node_script_action.rs"
        );
        include!("types/hdinsight/interactive_query_cluster_roles_worker_node.rs");
        include!(
            "types/hdinsight/interactive_query_cluster_roles_worker_node_autoscale.rs"
        );
        include!(
            "types/hdinsight/interactive_query_cluster_roles_worker_node_autoscale_recurrence.rs"
        );
        include!(
            "types/hdinsight/interactive_query_cluster_roles_worker_node_autoscale_recurrence_schedule.rs"
        );
        include!(
            "types/hdinsight/interactive_query_cluster_roles_worker_node_script_action.rs"
        );
        include!("types/hdinsight/interactive_query_cluster_roles_zookeeper_node.rs");
        include!(
            "types/hdinsight/interactive_query_cluster_roles_zookeeper_node_script_action.rs"
        );
        include!("types/hdinsight/interactive_query_cluster_security_profile.rs");
        include!("types/hdinsight/interactive_query_cluster_storage_account.rs");
        include!("types/hdinsight/interactive_query_cluster_storage_account_gen_2.rs");
        include!("types/hdinsight/kafka_cluster_component_version.rs");
        include!("types/hdinsight/kafka_cluster_compute_isolation.rs");
        include!("types/hdinsight/kafka_cluster_disk_encryption.rs");
        include!("types/hdinsight/kafka_cluster_extension.rs");
        include!("types/hdinsight/kafka_cluster_gateway.rs");
        include!("types/hdinsight/kafka_cluster_metastores.rs");
        include!("types/hdinsight/kafka_cluster_metastores_ambari.rs");
        include!("types/hdinsight/kafka_cluster_metastores_hive.rs");
        include!("types/hdinsight/kafka_cluster_metastores_oozie.rs");
        include!("types/hdinsight/kafka_cluster_monitor.rs");
        include!("types/hdinsight/kafka_cluster_network.rs");
        include!("types/hdinsight/kafka_cluster_private_link_configuration.rs");
        include!(
            "types/hdinsight/kafka_cluster_private_link_configuration_ip_configuration.rs"
        );
        include!("types/hdinsight/kafka_cluster_rest_proxy.rs");
        include!("types/hdinsight/kafka_cluster_roles.rs");
        include!("types/hdinsight/kafka_cluster_roles_head_node.rs");
        include!("types/hdinsight/kafka_cluster_roles_head_node_script_action.rs");
        include!("types/hdinsight/kafka_cluster_roles_kafka_management_node.rs");
        include!(
            "types/hdinsight/kafka_cluster_roles_kafka_management_node_script_action.rs"
        );
        include!("types/hdinsight/kafka_cluster_roles_worker_node.rs");
        include!("types/hdinsight/kafka_cluster_roles_worker_node_script_action.rs");
        include!("types/hdinsight/kafka_cluster_roles_zookeeper_node.rs");
        include!("types/hdinsight/kafka_cluster_roles_zookeeper_node_script_action.rs");
        include!("types/hdinsight/kafka_cluster_security_profile.rs");
        include!("types/hdinsight/kafka_cluster_storage_account.rs");
        include!("types/hdinsight/kafka_cluster_storage_account_gen_2.rs");
        include!("types/hdinsight/spark_cluster_component_version.rs");
        include!("types/hdinsight/spark_cluster_compute_isolation.rs");
        include!("types/hdinsight/spark_cluster_disk_encryption.rs");
        include!("types/hdinsight/spark_cluster_extension.rs");
        include!("types/hdinsight/spark_cluster_gateway.rs");
        include!("types/hdinsight/spark_cluster_metastores.rs");
        include!("types/hdinsight/spark_cluster_metastores_ambari.rs");
        include!("types/hdinsight/spark_cluster_metastores_hive.rs");
        include!("types/hdinsight/spark_cluster_metastores_oozie.rs");
        include!("types/hdinsight/spark_cluster_monitor.rs");
        include!("types/hdinsight/spark_cluster_network.rs");
        include!("types/hdinsight/spark_cluster_private_link_configuration.rs");
        include!(
            "types/hdinsight/spark_cluster_private_link_configuration_ip_configuration.rs"
        );
        include!("types/hdinsight/spark_cluster_roles.rs");
        include!("types/hdinsight/spark_cluster_roles_head_node.rs");
        include!("types/hdinsight/spark_cluster_roles_head_node_script_action.rs");
        include!("types/hdinsight/spark_cluster_roles_worker_node.rs");
        include!("types/hdinsight/spark_cluster_roles_worker_node_autoscale.rs");
        include!(
            "types/hdinsight/spark_cluster_roles_worker_node_autoscale_capacity.rs"
        );
        include!(
            "types/hdinsight/spark_cluster_roles_worker_node_autoscale_recurrence.rs"
        );
        include!(
            "types/hdinsight/spark_cluster_roles_worker_node_autoscale_recurrence_schedule.rs"
        );
        include!("types/hdinsight/spark_cluster_roles_worker_node_script_action.rs");
        include!("types/hdinsight/spark_cluster_roles_zookeeper_node.rs");
        include!("types/hdinsight/spark_cluster_roles_zookeeper_node_script_action.rs");
        include!("types/hdinsight/spark_cluster_security_profile.rs");
        include!("types/hdinsight/spark_cluster_storage_account.rs");
        include!("types/hdinsight/spark_cluster_storage_account_gen_2.rs");
        include!("types/hdinsight/get_cluster_gateway.rs");
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
