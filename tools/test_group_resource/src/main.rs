// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

mod args;

use args::{Args, SubCommand};
use clap::Parser;
use dsc_lib::dscresources::resource_manifest::{ResourceManifest, GetMethod};
use dsc_lib::dscresources::dscresource::{DscResource, ImplementedAs};

fn main() {
    let args = Args::parse();
    match args.subcommand {
        SubCommand::List => {
            let resource1 = DscResource {
                type_name: "Test/TestResource1".to_string(),
                version: "1.0.0".to_string(),
                description: Some("This is a test resource.".to_string()),
                implemented_as: ImplementedAs::Custom("TestResource".to_string()),
                path: "test_resource1".to_string(),
                directory: "test_directory".to_string(),
                author: Some("Microsoft".to_string()),
                properties: vec!["Property1".to_string(), "Property2".to_string()],
                requires: Some("Test/TestGroup".to_string()),
                manifest: Some(serde_json::to_value(ResourceManifest {
                    description: Some("This is a test resource.".to_string()),
                    schema_version: "https://raw.githubusercontent.com/PowerShell/DSC/main/schemas/2023/08/bundled/resource/manifest.json".to_string(),
                    resource_type: "Test/TestResource1".to_string(),
                    version: "1.0.0".to_string(),
                    tags: None,
                    get: GetMethod {
                        executable: String::new(),
                        args: None,
                        input: None,
                    },
                    set: None,
                    test: None,
                    export: None,
                    validate: None,
                    provider: None,
                    exit_codes: None,
                    schema: None,
                }).unwrap()),
            };
            let resource2 = DscResource {
                type_name: "Test/TestResource2".to_string(),
                version: "1.0.1".to_string(),
                description: Some("This is a test resource.".to_string()),
                implemented_as: ImplementedAs::Custom("TestResource".to_string()),
                path: "test_resource2".to_string(),
                directory: "test_directory".to_string(),
                author: Some("Microsoft".to_string()),
                properties: vec!["Property1".to_string(), "Property2".to_string()],
                requires: Some("Test/TestGroup".to_string()),
                manifest: Some(serde_json::to_value(ResourceManifest {
                    description: Some("This is a test resource.".to_string()),
                    schema_version: "https://raw.githubusercontent.com/PowerShell/DSC/main/schemas/2023/08/bundled/resource/manifest.json".to_string(),
                    resource_type: "Test/TestResource2".to_string(),
                    version: "1.0.1".to_string(),
                    tags: None,
                    get: GetMethod {
                        executable: String::new(),
                        args: None,
                        input: None,
                    },
                    set: None,
                    test: None,
                    export: None,
                    validate: None,
                    provider: None,
                    exit_codes: None,
                    schema: None,
                }).unwrap()),
            };
            println!("{}", serde_json::to_string(&resource1).unwrap());
            println!("{}", serde_json::to_string(&resource2).unwrap());
        },
        SubCommand::ListMissingRequires => {
            let resource1 = DscResource {
                type_name: "InvalidResource".to_string(),
                version: "1.0.0".to_string(),
                description: Some("This is a test resource.".to_string()),
                implemented_as: ImplementedAs::Custom("TestResource".to_string()),
                path: "test_resource1".to_string(),
                directory: "test_directory".to_string(),
                author: Some("Microsoft".to_string()),
                properties: vec!["Property1".to_string(), "Property2".to_string()],
                requires: None,
                manifest: None,
            };
            println!("{}", serde_json::to_string(&resource1).unwrap());
        }
    }
}
