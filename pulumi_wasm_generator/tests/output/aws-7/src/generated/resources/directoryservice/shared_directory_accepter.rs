/// Accepts a shared directory in a consumer account.
///
/// > **NOTE:** Destroying this resource removes the shared directory from the consumer account only.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = shared_directory::create(
///         "example",
///         SharedDirectoryArgs::builder()
///             .directory_id("${exampleAwsDirectoryServiceDirectory.id}")
///             .notes("example")
///             .target(
///                 SharedDirectoryTarget::builder()
///                     .id("${receiver.accountId}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleSharedDirectoryAccepter = shared_directory_accepter::create(
///         "exampleSharedDirectoryAccepter",
///         SharedDirectoryAccepterArgs::builder()
///             .shared_directory_id("${example.sharedDirectoryId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Directory Service Shared Directories using the shared directory ID. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/sharedDirectoryAccepter:SharedDirectoryAccepter example d-9267633ece
/// ```
pub mod shared_directory_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedDirectoryAccepterArgs {
        /// Identifier of the directory that is stored in the directory consumer account that corresponds to the shared directory in the owner account.
        #[builder(into)]
        pub shared_directory_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SharedDirectoryAccepterResult {
        /// Method used when sharing a directory (i.e., `ORGANIZATIONS` or `HANDSHAKE`).
        pub method: pulumi_wasm_rust::Output<String>,
        /// Message sent by the directory owner to the directory consumer to help the directory consumer administrator determine whether to approve or reject the share invitation.
        pub notes: pulumi_wasm_rust::Output<String>,
        /// Account identifier of the directory owner.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the Managed Microsoft AD directory from the perspective of the directory owner.
        pub owner_directory_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the directory that is stored in the directory consumer account that corresponds to the shared directory in the owner account.
        pub shared_directory_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SharedDirectoryAccepterArgs,
    ) -> SharedDirectoryAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let shared_directory_id_binding = args
            .shared_directory_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directoryservice/sharedDirectoryAccepter:SharedDirectoryAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "sharedDirectoryId".into(),
                    value: &shared_directory_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "method".into(),
                },
                register_interface::ResultField {
                    name: "notes".into(),
                },
                register_interface::ResultField {
                    name: "ownerAccountId".into(),
                },
                register_interface::ResultField {
                    name: "ownerDirectoryId".into(),
                },
                register_interface::ResultField {
                    name: "sharedDirectoryId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SharedDirectoryAccepterResult {
            method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("method").unwrap(),
            ),
            notes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notes").unwrap(),
            ),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerAccountId").unwrap(),
            ),
            owner_directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerDirectoryId").unwrap(),
            ),
            shared_directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedDirectoryId").unwrap(),
            ),
        }
    }
}
