/// Manages a SSH Public Key.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:compute:SshPublicKey
///     properties:
///       name: example
///       resourceGroupName: example
///       location: West Europe
///       publicKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: ~/.ssh/id_rsa.pub
///           return: result
/// ```
///
/// ## Import
///
/// SSH Public Keys can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/sshPublicKey:SshPublicKey example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/sshPublicKeys/mySshPublicKeyName1
/// ```
///
pub mod ssh_public_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SshPublicKeyArgs {
        /// The Azure Region where the SSH Public Key should exist. Changing this forces a new SSH Public Key to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this SSH Public Key. Changing this forces a new SSH Public Key to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// SSH public key used to authenticate to a virtual machine through ssh. the provided public key needs to be at least 2048-bit and in ssh-rsa format.
        #[builder(into)]
        pub public_key: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the SSH Public Key should exist. Changing this forces a new SSH Public Key to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the SSH Public Key.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SshPublicKeyResult {
        /// The Azure Region where the SSH Public Key should exist. Changing this forces a new SSH Public Key to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this SSH Public Key. Changing this forces a new SSH Public Key to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// SSH public key used to authenticate to a virtual machine through ssh. the provided public key needs to be at least 2048-bit and in ssh-rsa format.
        pub public_key: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the SSH Public Key should exist. Changing this forces a new SSH Public Key to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the SSH Public Key.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SshPublicKeyArgs) -> SshPublicKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_key_binding = args.public_key.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/sshPublicKey:SshPublicKey".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicKey".into(),
                    value: &public_key_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicKey".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SshPublicKeyResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKey").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
