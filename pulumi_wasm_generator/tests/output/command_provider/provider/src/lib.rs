use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest, ResultField};
use bindings::exports::pulumi::command::local_command;
use bindings::exports::pulumi::command::remote_command;
use bindings::exports::pulumi::command::remote_copy_file;

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
                ResultField { name: "archive".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "archivePaths".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "assetPaths".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "assets".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "create".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "delete".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "dir".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "environment".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "interpreter".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "stderr".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "stdin".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "stdout".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "triggers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "update".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        local_command::Res {
            archive: o.get_field("archive", false),
            archive_paths: o.get_field("archivePaths", false),
            asset_paths: o.get_field("assetPaths", false),
            assets: o.get_field("assets", false),
            create: o.get_field("create", false),
            delete: o.get_field("delete", false),
            dir: o.get_field("dir", false),
            environment: o.get_field("environment", false),
            interpreter: o.get_field("interpreter", false),
            stderr: o.get_field("stderr", true),
            stdin: o.get_field("stdin", false),
            stdout: o.get_field("stdout", true),
            triggers: o.get_field("triggers", false),
            update: o.get_field("update", false),
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
                ResultField { name: "connection".into(), schema: vec![129, 166, 79, 98, 106, 101, 99, 116, 138, 175, 97, 103, 101, 110, 116, 83, 111, 99, 107, 101, 116, 80, 97, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 100, 105, 97, 108, 69, 114, 114, 111, 114, 76, 105, 109, 105, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 104, 111, 115, 116, 166, 83, 116, 114, 105, 110, 103, 168, 112, 97, 115, 115, 119, 111, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 112, 101, 114, 68, 105, 97, 108, 84, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 112, 111, 114, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 68, 111, 117, 98, 108, 101, 170, 112, 114, 105, 118, 97, 116, 101, 75, 101, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 112, 114, 105, 118, 97, 116, 101, 75, 101, 121, 80, 97, 115, 115, 119, 111, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 112, 114, 111, 120, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 137, 175, 97, 103, 101, 110, 116, 83, 111, 99, 107, 101, 116, 80, 97, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 100, 105, 97, 108, 69, 114, 114, 111, 114, 76, 105, 109, 105, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 104, 111, 115, 116, 166, 83, 116, 114, 105, 110, 103, 168, 112, 97, 115, 115, 119, 111, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 112, 101, 114, 68, 105, 97, 108, 84, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 112, 111, 114, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 68, 111, 117, 98, 108, 101, 170, 112, 114, 105, 118, 97, 116, 101, 75, 101, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 112, 114, 105, 118, 97, 116, 101, 75, 101, 121, 80, 97, 115, 115, 119, 111, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 117, 115, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 117, 115, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "create".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "delete".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "environment".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "stderr".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "stdin".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "stdout".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "triggers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "update".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        remote_command::Res {
            connection: o.get_field("connection", true),
            create: o.get_field("create", false),
            delete: o.get_field("delete", false),
            environment: o.get_field("environment", false),
            stderr: o.get_field("stderr", true),
            stdin: o.get_field("stdin", false),
            stdout: o.get_field("stdout", true),
            triggers: o.get_field("triggers", false),
            update: o.get_field("update", false),
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
                ResultField { name: "connection".into(), schema: vec![129, 166, 79, 98, 106, 101, 99, 116, 138, 175, 97, 103, 101, 110, 116, 83, 111, 99, 107, 101, 116, 80, 97, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 100, 105, 97, 108, 69, 114, 114, 111, 114, 76, 105, 109, 105, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 104, 111, 115, 116, 166, 83, 116, 114, 105, 110, 103, 168, 112, 97, 115, 115, 119, 111, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 112, 101, 114, 68, 105, 97, 108, 84, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 112, 111, 114, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 68, 111, 117, 98, 108, 101, 170, 112, 114, 105, 118, 97, 116, 101, 75, 101, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 112, 114, 105, 118, 97, 116, 101, 75, 101, 121, 80, 97, 115, 115, 119, 111, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 112, 114, 111, 120, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 137, 175, 97, 103, 101, 110, 116, 83, 111, 99, 107, 101, 116, 80, 97, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 100, 105, 97, 108, 69, 114, 114, 111, 114, 76, 105, 109, 105, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 104, 111, 115, 116, 166, 83, 116, 114, 105, 110, 103, 168, 112, 97, 115, 115, 119, 111, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 112, 101, 114, 68, 105, 97, 108, 84, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 112, 111, 114, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 68, 111, 117, 98, 108, 101, 170, 112, 114, 105, 118, 97, 116, 101, 75, 101, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 112, 114, 105, 118, 97, 116, 101, 75, 101, 121, 80, 97, 115, 115, 119, 111, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 117, 115, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 117, 115, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "localPath".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "remotePath".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "triggers".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        remote_copy_file::Res {
            connection: o.get_field("connection", true),
            local_path: o.get_field("localPath", true),
            remote_path: o.get_field("remotePath", true),
            triggers: o.get_field("triggers", false),
        }

    }
}
