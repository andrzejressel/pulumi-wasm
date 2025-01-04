/// Manages a directory in your account (directory owner) shared with another account (directory consumer).
///
/// ## Example Usage
///
///
/// ## Import
///
/// Using `pulumi import`, import Directory Service Shared Directories using the owner directory ID/shared directory ID. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/sharedDirectory:SharedDirectory example d-1234567890/d-9267633ece
/// ```
pub mod shared_directory {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedDirectoryArgs {
        /// Identifier of the Managed Microsoft AD directory that you want to share with other accounts.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// Method used when sharing a directory. Valid values are `ORGANIZATIONS` and `HANDSHAKE`. Default is `HANDSHAKE`.
        #[builder(into, default)]
        pub method: pulumi_wasm_rust::Output<Option<String>>,
        /// Message sent by the directory owner to the directory consumer to help the directory consumer administrator determine whether to approve or reject the share invitation.
        #[builder(into, default)]
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier for the directory consumer account with whom the directory is to be shared. See below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target: pulumi_wasm_rust::Output<
            super::super::types::directoryservice::SharedDirectoryTarget,
        >,
    }
    #[allow(dead_code)]
    pub struct SharedDirectoryResult {
        /// Identifier of the Managed Microsoft AD directory that you want to share with other accounts.
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// Method used when sharing a directory. Valid values are `ORGANIZATIONS` and `HANDSHAKE`. Default is `HANDSHAKE`.
        pub method: pulumi_wasm_rust::Output<Option<String>>,
        /// Message sent by the directory owner to the directory consumer to help the directory consumer administrator determine whether to approve or reject the share invitation.
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the directory that is stored in the directory consumer account that corresponds to the shared directory in the owner account.
        pub shared_directory_id: pulumi_wasm_rust::Output<String>,
        /// Identifier for the directory consumer account with whom the directory is to be shared. See below.
        ///
        /// The following arguments are optional:
        pub target: pulumi_wasm_rust::Output<
            super::super::types::directoryservice::SharedDirectoryTarget,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SharedDirectoryArgs) -> SharedDirectoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let directory_id_binding = args.directory_id.get_inner();
        let method_binding = args.method.get_inner();
        let notes_binding = args.notes.get_inner();
        let target_binding = args.target.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directoryservice/sharedDirectory:SharedDirectory".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "method".into(),
                    value: &method_binding,
                },
                register_interface::ObjectField {
                    name: "notes".into(),
                    value: &notes_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "directoryId".into(),
                },
                register_interface::ResultField {
                    name: "method".into(),
                },
                register_interface::ResultField {
                    name: "notes".into(),
                },
                register_interface::ResultField {
                    name: "sharedDirectoryId".into(),
                },
                register_interface::ResultField {
                    name: "target".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SharedDirectoryResult {
            directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryId").unwrap(),
            ),
            method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("method").unwrap(),
            ),
            notes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notes").unwrap(),
            ),
            shared_directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedDirectoryId").unwrap(),
            ),
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
        }
    }
}
