pub mod documentdb {
    /// An Azure Cosmos DB container.
    /// API Version: 2021-03-15.
    ///
    /// ## Example Usage
    /// ### CosmosDBSqlContainerCreateUpdate
    ///
    ///
    ///
    ///
    ///
    /// ## Import
    ///
    /// An existing resource can be imported using its type token, name, and identifier, e.g.
    ///
    /// ```sh
    /// $ pulumi import azure-native:documentdb:SqlResourceSqlContainer containerName /subscriptions/subid/resourceGroups/rg1/providers/Microsoft.DocumentDB/databaseAccounts/ddb1/sqlDatabases/databaseName/sqlContainers/containerName
    /// ```
    pub mod sql_resource_sql_container {
        include!("resources/documentdb/sql_resource_sql_container.rs");
    }
}
pub mod functions {}
pub mod types {
    pub mod documentdb {
        include!("types/documentdb/composite_path_response.rs");
        include!("types/documentdb/indexing_policy_response.rs");
        include!("types/documentdb/sql_container_get_properties_response_resource.rs");
    }
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world my-world {
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
        pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface } }
    );
}
