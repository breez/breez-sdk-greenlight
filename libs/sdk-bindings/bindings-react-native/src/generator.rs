use anyhow::Result;
use askama::Template;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use serde::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use uniffi_bindgen::{BindingGenerator, BindingGeneratorConfig, ComponentInterface};

use crate::gen_kotlin;
use crate::gen_swift;
use crate::gen_typescript;

pub struct RNBindingGenerator {}

impl RNBindingGenerator {
    fn write_bindings(
        &self,
        bindings_output: &String,
        output_path: &Utf8Path,
        file_name: &Utf8Path,
    ) -> Result<Utf8PathBuf> {
        fs::create_dir_all(output_path)?;
        let bindings_path: camino::Utf8PathBuf = output_path.join(file_name);
        let mut f: File = File::create(&bindings_path)?;
        write!(f, "{}", bindings_output)?;
        Ok(bindings_path)
    }

    fn write_kotlin_mapper_bindings(
        &self,
        ci: &ComponentInterface,
        config: RNConfig,
        base_output_path: &Utf8Path,
    ) -> Result<()> {
        // Create the path
        let output_path =
            base_output_path.join(Utf8Path::new("android/src/main/java/com/breezsdk"));
        // Generate and write the binding to file
        let bindings_output = self::gen_kotlin::MapperGenerator::new(config.clone(), ci)
            .render()
            .map_err(anyhow::Error::new)?;
        let bindings_file = self
            .write_bindings(
                &bindings_output,
                &output_path,
                Utf8Path::new("BreezSDKMapper.kt"),
            )
            .unwrap();
        // Lint binding
        self.lint_kotlin_bindings(&bindings_file);
        Ok(())
    }

    fn write_kotlin_module_bindings(
        &self,
        ci: &ComponentInterface,
        config: RNConfig,
        base_output_path: &Utf8Path,
    ) -> Result<()> {
        // Create the path
        let output_path =
            base_output_path.join(Utf8Path::new("android/src/main/java/com/breezsdk"));
        // Generate and write the binding to file
        let bindings_output = self::gen_kotlin::ModuleGenerator::new(config.clone(), ci)
            .render()
            .map_err(anyhow::Error::new)?;
        let bindings_file = self
            .write_bindings(
                &bindings_output,
                &output_path,
                Utf8Path::new("BreezSDKModule.kt"),
            )
            .unwrap();
        // Lint binding
        self.lint_kotlin_bindings(&bindings_file);
        Ok(())
    }

    fn lint_kotlin_bindings(&self, bindings_file: &Utf8PathBuf) {
        if let Err(e) = Command::new("ktlint").arg("-F").arg(bindings_file).output() {
            println!(
                "Warning: Unable to auto-format {} using ktlint: {:?}",
                bindings_file.file_name().unwrap(),
                e
            )
        }
    }

    fn write_swift_mapper_bindings(
        &self,
        ci: &ComponentInterface,
        config: RNConfig,
        base_output_path: &Utf8Path,
    ) -> Result<()> {
        // Create the path
        let output_path = base_output_path.join(Utf8Path::new("ios"));
        // Generate and write the binding to file
        let bindings_output = self::gen_swift::MapperGenerator::new(config.clone(), ci)
            .render()
            .map_err(anyhow::Error::new)?;
        let bindings_file = self
            .write_bindings(
                &bindings_output,
                &output_path,
                Utf8Path::new("BreezSDKMapper.swift"),
            )
            .unwrap();
        // Lint binding
        self.lint_swift_bindings(&bindings_file);
        Ok(())
    }

    fn write_swift_extern_bindings(
        &self,
        ci: &ComponentInterface,
        config: RNConfig,
        base_output_path: &Utf8Path,
    ) -> Result<()> {
        // Create the path
        let output_path = base_output_path.join(Utf8Path::new("ios"));
        // Generate and write the binding to file
        let bindings_output = self::gen_swift::ExternGenerator::new(config.clone(), ci)
            .render()
            .map_err(anyhow::Error::new)?;
        let bindings_file = self
            .write_bindings(
                &bindings_output,
                &output_path,
                Utf8Path::new("RNBreezSDK.m"),
            )
            .unwrap();
        // Lint binding
        self.lint_swift_bindings(&bindings_file);
        Ok(())
    }

    fn write_swift_module_bindings(
        &self,
        ci: &ComponentInterface,
        config: RNConfig,
        base_output_path: &Utf8Path,
    ) -> Result<()> {
        // Create the path
        let output_path = base_output_path.join(Utf8Path::new("ios"));
        // Generate and write the binding to file
        let bindings_output = self::gen_swift::ModuleGenerator::new(config.clone(), ci)
            .render()
            .map_err(anyhow::Error::new)?;
        let bindings_file = self
            .write_bindings(
                &bindings_output,
                &output_path,
                Utf8Path::new("RNBreezSDK.swift"),
            )
            .unwrap();
        // Lint binding
        self.lint_swift_bindings(&bindings_file);
        Ok(())
    }

    fn lint_swift_bindings(&self, bindings_file: &Utf8PathBuf) {
        if let Err(e) = Command::new("swiftformat")
            .arg(bindings_file.as_str())
            .output()
        {
            println!(
                "Warning: Unable to auto-format {} using swiftformat: {:?}",
                bindings_file.file_name().unwrap(),
                e
            )
        }
    }

    fn write_typescript_bindings(
        &self,
        ci: &ComponentInterface,
        config: RNConfig,
        base_output_path: &Utf8Path,
    ) -> Result<()> {
        // Create the path
        let output_path = base_output_path.join(Utf8Path::new("ts/src"));
        // Generate and write the binding to file
        let bindings_output = self::gen_typescript::ModuleGenerator::new(config.clone(), ci)
            .render()
            .map_err(anyhow::Error::new)?;
        let bindings_file = self
            .write_bindings(&bindings_output, &output_path, Utf8Path::new("index.ts"))
            .unwrap();
        // Lint binding
        self.lint_typescript_bindings(&bindings_file);
        Ok(())
    }

    fn lint_typescript_bindings(&self, bindings_file: &Utf8PathBuf) {
        if let Err(e) = Command::new("tslint")
            .arg("--fix")
            .arg(bindings_file)
            .output()
        {
            println!(
                "Warning: Unable to auto-format {} using tslint: {:?}",
                bindings_file.file_name().unwrap(),
                e
            )
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RNConfig {
    package_name: Option<String>,
}

impl RNConfig {}

impl BindingGeneratorConfig for RNConfig {
    fn get_entry_from_bindings_table(_bindings: &toml::value::Value) -> Option<toml::value::Value> {
        if let Some(table) = _bindings.as_table() {
            table.get("rn").cloned()
        } else {
            None
        }
    }

    fn get_config_defaults(ci: &ComponentInterface) -> Vec<(String, toml::value::Value)> {
        vec![
            (
                "package_name".to_string(),
                toml::value::Value::String(ci.namespace().to_string()),
            ),
            (
                "cdylib_name".to_string(),
                toml::value::Value::String(ci.namespace().to_string()),
            ),
        ]
    }
}

impl BindingGenerator for RNBindingGenerator {
    type Config = RNConfig;

    fn write_bindings(
        &self,
        ci: ComponentInterface,
        config: Self::Config,
        out_dir: &Utf8Path,
    ) -> Result<()> {
        fs::create_dir_all(out_dir)?;

        // generate kotlin
        self.write_kotlin_mapper_bindings(&ci, config.clone(), out_dir)?;
        self.write_kotlin_module_bindings(&ci, config.clone(), out_dir)?;

        // generate ios
        self.write_swift_mapper_bindings(&ci, config.clone(), out_dir)?;
        self.write_swift_extern_bindings(&ci, config.clone(), out_dir)?;
        self.write_swift_module_bindings(&ci, config.clone(), out_dir)?;

        // generate typescript
        self.write_typescript_bindings(&ci, config.clone(), out_dir)?;
        Ok(())
    }
}
