pub mod machinelearning {
    include!("resources/machinelearning/compute_cluster.rs");
    include!("resources/machinelearning/compute_instance.rs");
    include!("resources/machinelearning/datastore_blobstorage.rs");
    include!("resources/machinelearning/datastore_datalake_gen_2.rs");
    include!("resources/machinelearning/datastore_fileshare.rs");
    include!("resources/machinelearning/inference_cluster.rs");
    include!("resources/machinelearning/synapse_spark.rs");
    include!("resources/machinelearning/workspace.rs");
    include!("resources/machinelearning/workspace_network_outbound_rule_fqdn.rs");
}
pub mod functions {
    pub mod machinelearning {
        include!("functions/machinelearning/get_workspace.rs");
    }
}
pub mod types {
    pub mod machinelearning {
        include!("types/machinelearning/compute_cluster_identity.rs");
        include!("types/machinelearning/compute_cluster_scale_settings.rs");
        include!("types/machinelearning/compute_cluster_ssh.rs");
        include!("types/machinelearning/compute_instance_assign_to_user.rs");
        include!("types/machinelearning/compute_instance_identity.rs");
        include!("types/machinelearning/compute_instance_ssh.rs");
        include!("types/machinelearning/inference_cluster_identity.rs");
        include!("types/machinelearning/inference_cluster_ssl.rs");
        include!("types/machinelearning/synapse_spark_identity.rs");
        include!("types/machinelearning/workspace_encryption.rs");
        include!("types/machinelearning/workspace_feature_store.rs");
        include!("types/machinelearning/workspace_identity.rs");
        include!("types/machinelearning/workspace_managed_network.rs");
        include!("types/machinelearning/workspace_serverless_compute.rs");
        include!("types/machinelearning/get_workspace_identity.rs");
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
