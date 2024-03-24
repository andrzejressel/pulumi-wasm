use crate::bindings::component::pulumi_wasm::register_interface::RegisterResourceRequest;
use crate::bindings::component::pulumi_wasm::register_interface::ObjectField;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl bindings::exports::pulumi::command::command_local_command::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::command::command_local_command::Args) -> bindings::exports::pulumi::command::command_local_command::Res {
/*        let mut res = bindings::exports::pulumi::random::command_local_command::res {
        };
        res
*/

//        let r#type = "command:local:Command".to_string();

        let request = RegisterResourceRequest {
            type_: "command:local:Command".into(),
            name,
            object: vec![
                ObjectField { name: "archivePaths".into(), value: args.archive_paths },
                ObjectField { name: "assetPaths".into(), value: args.asset_paths },
                ObjectField { name: "create".into(), value: args.create },
                ObjectField { name: "delete".into(), value: args.delete },
                ObjectField { name: "dir".into(), value: args.dir },
                ObjectField { name: "environment".into(), value: args.environment },
                ObjectField { name: "interpreter".into(), value: args.interpreter },
                ObjectField { name: "stdin".into(), value: args.stdin },
                ObjectField { name: "triggers".into(), value: args.triggers },
                ObjectField { name: "update".into(), value: args.update },
            ],
        };

        todo!()

    }
}
impl bindings::exports::pulumi::command::command_remote_command::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::command::command_remote_command::Args) -> bindings::exports::pulumi::command::command_remote_command::Res {
/*        let mut res = bindings::exports::pulumi::random::command_remote_command::res {
        };
        res
*/

//        let r#type = "command:remote:Command".to_string();

        let request = RegisterResourceRequest {
            type_: "command:remote:Command".into(),
            name,
            object: vec![
                ObjectField { name: "connection".into(), value: args.connection },
                ObjectField { name: "create".into(), value: args.create },
                ObjectField { name: "delete".into(), value: args.delete },
                ObjectField { name: "environment".into(), value: args.environment },
                ObjectField { name: "stdin".into(), value: args.stdin },
                ObjectField { name: "triggers".into(), value: args.triggers },
                ObjectField { name: "update".into(), value: args.update },
            ],
        };

        todo!()

    }
}
impl bindings::exports::pulumi::command::command_remote_copy_file::Guest for Component {
    fn invoke(name: String, args: bindings::exports::pulumi::command::command_remote_copy_file::Args) -> bindings::exports::pulumi::command::command_remote_copy_file::Res {
/*        let mut res = bindings::exports::pulumi::random::command_remote_copy_file::res {
        };
        res
*/

//        let r#type = "command:remote:CopyFile".to_string();

        let request = RegisterResourceRequest {
            type_: "command:remote:CopyFile".into(),
            name,
            object: vec![
                ObjectField { name: "connection".into(), value: args.connection },
                ObjectField { name: "localPath".into(), value: args.local_path },
                ObjectField { name: "remotePath".into(), value: args.remote_path },
                ObjectField { name: "triggers".into(), value: args.triggers },
            ],
        };

        todo!()

    }
}
