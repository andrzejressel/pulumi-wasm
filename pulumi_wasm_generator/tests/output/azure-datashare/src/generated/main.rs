pub mod datashare {
    include!("resources/datashare/account.rs");
    include!("resources/datashare/dataset_blob_storage.rs");
    include!("resources/datashare/dataset_data_lake_gen_2.rs");
    include!("resources/datashare/dataset_kusto_cluster.rs");
    include!("resources/datashare/dataset_kusto_database.rs");
    include!("resources/datashare/share.rs");
}
pub mod functions {
    pub mod datashare {
        include!("functions/datashare/get_account.rs");
        include!("functions/datashare/get_dataset_blob_storage.rs");
        include!("functions/datashare/get_dataset_data_lake_gen_2.rs");
        include!("functions/datashare/get_dataset_kusto_cluster.rs");
        include!("functions/datashare/get_dataset_kusto_database.rs");
        include!("functions/datashare/get_share.rs");
    }
}
pub mod types {
    pub mod datashare {
        include!("types/datashare/account_identity.rs");
        include!("types/datashare/dataset_blob_storage_storage_account.rs");
        include!("types/datashare/share_snapshot_schedule.rs");
        include!("types/datashare/get_account_identity.rs");
        include!("types/datashare/get_dataset_blob_storage_storage_account.rs");
        include!("types/datashare/get_share_snapshot_schedule.rs");
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
