
pub mod local_command {

    pub struct CommandArgs {
        pub archive_paths: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub asset_paths: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub create: pulumi_wasm_rust::Output<Option<String>>,
        pub delete: pulumi_wasm_rust::Output<Option<String>>,
        pub dir: pulumi_wasm_rust::Output<Option<String>>,
        pub environment: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub interpreter: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub stdin: pulumi_wasm_rust::Output<Option<String>>,
        pub triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub update: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct CommandResult {
        pub archive: pulumi_wasm_rust::Output<Option<String>>,
        pub archive_paths: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub asset_paths: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub assets: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub create: pulumi_wasm_rust::Output<Option<String>>,
        pub delete: pulumi_wasm_rust::Output<Option<String>>,
        pub dir: pulumi_wasm_rust::Output<Option<String>>,
        pub environment: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub interpreter: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub stderr: pulumi_wasm_rust::Output<String>,
        pub stdin: pulumi_wasm_rust::Output<Option<String>>,
        pub stdout: pulumi_wasm_rust::Output<String>,
        pub triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub update: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn command(name: &str, args: CommandArgs) -> CommandResult {

        let result = crate::bindings::pulumi::command::local_command::invoke(name, &crate::bindings::pulumi::command::local_command::Args {
            archive_paths: &crate::clone::<Option<Vec<String>>>(args.archive_paths),
            asset_paths: &crate::clone::<Option<Vec<String>>>(args.asset_paths),
            create: &crate::clone::<Option<String>>(args.create),
            delete: &crate::clone::<Option<String>>(args.delete),
            dir: &crate::clone::<Option<String>>(args.dir),
            environment: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.environment),
            interpreter: &crate::clone::<Option<Vec<String>>>(args.interpreter),
            stdin: &crate::clone::<Option<String>>(args.stdin),
            triggers: &crate::clone::<Option<Vec<String>>>(args.triggers),
            update: &crate::clone::<Option<String>>(args.update),
        });

        CommandResult {
            archive: crate::random_to_domain_mapper::<Option<String>>(result.archive),
            archive_paths: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.archive_paths),
            asset_paths: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.asset_paths),
            assets: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.assets),
            create: crate::random_to_domain_mapper::<Option<String>>(result.create),
            delete: crate::random_to_domain_mapper::<Option<String>>(result.delete),
            dir: crate::random_to_domain_mapper::<Option<String>>(result.dir),
            environment: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.environment),
            interpreter: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.interpreter),
            stderr: crate::random_to_domain_mapper::<String>(result.stderr),
            stdin: crate::random_to_domain_mapper::<Option<String>>(result.stdin),
            stdout: crate::random_to_domain_mapper::<String>(result.stdout),
            triggers: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.triggers),
            update: crate::random_to_domain_mapper::<Option<String>>(result.update),
        }
    }

}


pub mod remote_command {

    pub struct CommandArgs {
        pub connection: pulumi_wasm_rust::Output<crate::types::Connection>,
        pub create: pulumi_wasm_rust::Output<Option<String>>,
        pub delete: pulumi_wasm_rust::Output<Option<String>>,
        pub environment: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub stdin: pulumi_wasm_rust::Output<Option<String>>,
        pub triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub update: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct CommandResult {
        pub connection: pulumi_wasm_rust::Output<crate::types::Connection>,
        pub create: pulumi_wasm_rust::Output<Option<String>>,
        pub delete: pulumi_wasm_rust::Output<Option<String>>,
        pub environment: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub stderr: pulumi_wasm_rust::Output<String>,
        pub stdin: pulumi_wasm_rust::Output<Option<String>>,
        pub stdout: pulumi_wasm_rust::Output<String>,
        pub triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub update: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn command(name: &str, args: CommandArgs) -> CommandResult {

        let result = crate::bindings::pulumi::command::remote_command::invoke(name, &crate::bindings::pulumi::command::remote_command::Args {
            connection: &crate::clone::<crate::types::Connection>(args.connection),
            create: &crate::clone::<Option<String>>(args.create),
            delete: &crate::clone::<Option<String>>(args.delete),
            environment: &crate::clone::<Option<std::collections::HashMap<String, String>>>(args.environment),
            stdin: &crate::clone::<Option<String>>(args.stdin),
            triggers: &crate::clone::<Option<Vec<String>>>(args.triggers),
            update: &crate::clone::<Option<String>>(args.update),
        });

        CommandResult {
            connection: crate::random_to_domain_mapper::<crate::types::Connection>(result.connection),
            create: crate::random_to_domain_mapper::<Option<String>>(result.create),
            delete: crate::random_to_domain_mapper::<Option<String>>(result.delete),
            environment: crate::random_to_domain_mapper::<Option<std::collections::HashMap<String, String>>>(result.environment),
            stderr: crate::random_to_domain_mapper::<String>(result.stderr),
            stdin: crate::random_to_domain_mapper::<Option<String>>(result.stdin),
            stdout: crate::random_to_domain_mapper::<String>(result.stdout),
            triggers: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.triggers),
            update: crate::random_to_domain_mapper::<Option<String>>(result.update),
        }
    }

}


pub mod remote_copy_file {

    pub struct CopyFileArgs {
        pub connection: pulumi_wasm_rust::Output<crate::types::Connection>,
        pub local_path: pulumi_wasm_rust::Output<String>,
        pub remote_path: pulumi_wasm_rust::Output<String>,
        pub triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }

    pub struct CopyFileResult {
        pub connection: pulumi_wasm_rust::Output<crate::types::Connection>,
        pub local_path: pulumi_wasm_rust::Output<String>,
        pub remote_path: pulumi_wasm_rust::Output<String>,
        pub triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }

    pub fn copy_file(name: &str, args: CopyFileArgs) -> CopyFileResult {

        let result = crate::bindings::pulumi::command::remote_copy_file::invoke(name, &crate::bindings::pulumi::command::remote_copy_file::Args {
            connection: &crate::clone::<crate::types::Connection>(args.connection),
            local_path: &crate::clone::<String>(args.local_path),
            remote_path: &crate::clone::<String>(args.remote_path),
            triggers: &crate::clone::<Option<Vec<String>>>(args.triggers),
        });

        CopyFileResult {
            connection: crate::random_to_domain_mapper::<crate::types::Connection>(result.connection),
            local_path: crate::random_to_domain_mapper::<String>(result.local_path),
            remote_path: crate::random_to_domain_mapper::<String>(result.remote_path),
            triggers: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.triggers),
        }
    }

}

