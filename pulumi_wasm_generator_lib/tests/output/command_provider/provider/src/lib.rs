use bindings::exports::pulumi::command::local_command;
use bindings::exports::pulumi::command::remote_command;
use bindings::exports::pulumi::command::remote_copy_file;

use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl local_command::Guest for Component {
    fn invoke(name: String, args: local_command::Args) -> local_command::Res {
        wasm_common::setup_logger();
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
            results: vec![
                ResultField { name: "archive".into() },
                ResultField { name: "archivePaths".into() },
                ResultField { name: "assetPaths".into() },
                ResultField { name: "assets".into() },
                ResultField { name: "create".into() },
                ResultField { name: "delete".into() },
                ResultField { name: "dir".into() },
                ResultField { name: "environment".into() },
                ResultField { name: "interpreter".into() },
                ResultField { name: "stderr".into() },
                ResultField { name: "stdin".into() },
                ResultField { name: "stdout".into() },
                ResultField { name: "triggers".into() },
                ResultField { name: "update".into() },
            ],
        };

        let o = register(&request);

        local_command::Res {
            archive: o.fields.iter().find(|o| o.name == "archive").unwrap().output.duplicate(),
            archive_paths: o.fields.iter().find(|o| o.name == "archivePaths").unwrap().output.duplicate(),
            asset_paths: o.fields.iter().find(|o| o.name == "assetPaths").unwrap().output.duplicate(),
            assets: o.fields.iter().find(|o| o.name == "assets").unwrap().output.duplicate(),
            create: o.fields.iter().find(|o| o.name == "create").unwrap().output.duplicate(),
            delete: o.fields.iter().find(|o| o.name == "delete").unwrap().output.duplicate(),
            dir: o.fields.iter().find(|o| o.name == "dir").unwrap().output.duplicate(),
            environment: o.fields.iter().find(|o| o.name == "environment").unwrap().output.duplicate(),
            interpreter: o.fields.iter().find(|o| o.name == "interpreter").unwrap().output.duplicate(),
            stderr: o.fields.iter().find(|o| o.name == "stderr").unwrap().output.duplicate(),
            stdin: o.fields.iter().find(|o| o.name == "stdin").unwrap().output.duplicate(),
            stdout: o.fields.iter().find(|o| o.name == "stdout").unwrap().output.duplicate(),
            triggers: o.fields.iter().find(|o| o.name == "triggers").unwrap().output.duplicate(),
            update: o.fields.iter().find(|o| o.name == "update").unwrap().output.duplicate(),
        }

    }
}
impl remote_command::Guest for Component {
    fn invoke(name: String, args: remote_command::Args) -> remote_command::Res {
        wasm_common::setup_logger();
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
            results: vec![
                ResultField { name: "connection".into() },
                ResultField { name: "create".into() },
                ResultField { name: "delete".into() },
                ResultField { name: "environment".into() },
                ResultField { name: "stderr".into() },
                ResultField { name: "stdin".into() },
                ResultField { name: "stdout".into() },
                ResultField { name: "triggers".into() },
                ResultField { name: "update".into() },
            ],
        };

        let o = register(&request);

        remote_command::Res {
            connection: o.fields.iter().find(|o| o.name == "connection").unwrap().output.duplicate(),
            create: o.fields.iter().find(|o| o.name == "create").unwrap().output.duplicate(),
            delete: o.fields.iter().find(|o| o.name == "delete").unwrap().output.duplicate(),
            environment: o.fields.iter().find(|o| o.name == "environment").unwrap().output.duplicate(),
            stderr: o.fields.iter().find(|o| o.name == "stderr").unwrap().output.duplicate(),
            stdin: o.fields.iter().find(|o| o.name == "stdin").unwrap().output.duplicate(),
            stdout: o.fields.iter().find(|o| o.name == "stdout").unwrap().output.duplicate(),
            triggers: o.fields.iter().find(|o| o.name == "triggers").unwrap().output.duplicate(),
            update: o.fields.iter().find(|o| o.name == "update").unwrap().output.duplicate(),
        }

    }
}
impl remote_copy_file::Guest for Component {
    fn invoke(name: String, args: remote_copy_file::Args) -> remote_copy_file::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "command:remote:CopyFile".into(),
            name,
            object: vec![
                ObjectField { name: "connection".into(), value: args.connection },
                ObjectField { name: "localPath".into(), value: args.local_path },
                ObjectField { name: "remotePath".into(), value: args.remote_path },
                ObjectField { name: "triggers".into(), value: args.triggers },
            ],
            results: vec![
                ResultField { name: "connection".into() },
                ResultField { name: "localPath".into() },
                ResultField { name: "remotePath".into() },
                ResultField { name: "triggers".into() },
            ],
        };

        let o = register(&request);

        remote_copy_file::Res {
            connection: o.fields.iter().find(|o| o.name == "connection").unwrap().output.duplicate(),
            local_path: o.fields.iter().find(|o| o.name == "localPath").unwrap().output.duplicate(),
            remote_path: o.fields.iter().find(|o| o.name == "remotePath").unwrap().output.duplicate(),
            triggers: o.fields.iter().find(|o| o.name == "triggers").unwrap().output.duplicate(),
        }

    }
}
