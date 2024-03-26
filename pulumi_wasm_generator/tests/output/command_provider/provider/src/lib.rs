use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest};
use bindings::exports::pulumi::command::local_command;
use bindings::exports::pulumi::command::remote_command;
use bindings::exports::pulumi::command::remote_copy_file;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl local_command::Guest for Component {
    fn invoke(name: String, args: local_command::Args) -> local_command::Res {

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

        let o = register(&request);

        local_command::Res {
            archive: o.get_field("archive"),
            archive_paths: o.get_field("archivePaths"),
            asset_paths: o.get_field("assetPaths"),
            assets: o.get_field("assets"),
            create: o.get_field("create"),
            delete: o.get_field("delete"),
            dir: o.get_field("dir"),
            environment: o.get_field("environment"),
            interpreter: o.get_field("interpreter"),
            stderr: o.get_field("stderr"),
            stdin: o.get_field("stdin"),
            stdout: o.get_field("stdout"),
            triggers: o.get_field("triggers"),
            update: o.get_field("update"),
        }

    }
}
impl remote_command::Guest for Component {
    fn invoke(name: String, args: remote_command::Args) -> remote_command::Res {

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

        let o = register(&request);

        remote_command::Res {
            connection: o.get_field("connection"),
            create: o.get_field("create"),
            delete: o.get_field("delete"),
            environment: o.get_field("environment"),
            stderr: o.get_field("stderr"),
            stdin: o.get_field("stdin"),
            stdout: o.get_field("stdout"),
            triggers: o.get_field("triggers"),
            update: o.get_field("update"),
        }

    }
}
impl remote_copy_file::Guest for Component {
    fn invoke(name: String, args: remote_copy_file::Args) -> remote_copy_file::Res {

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

        let o = register(&request);

        remote_copy_file::Res {
            connection: o.get_field("connection"),
            local_path: o.get_field("localPath"),
            remote_path: o.get_field("remotePath"),
            triggers: o.get_field("triggers"),
        }

    }
}
