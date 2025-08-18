use minijinja::{context, Environment};
use serde_json::json;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::{
    error::Error,
    file_gen_config::FileGenConfig,
    jinja_environment_builder::{
        JinjaEnvironmentBuilder, MODELS_DIR_ENTRYPOINT, MODEL_SCHEMA_FILE, QUERY,
    },
    model_modules::{ModelModule, ModelModules},
    query_namespace::{QueryNamespace, QueryNamespaceBuilder},
    request::Request,
    response::File,
};

pub struct FileGenerator {
    pub environment: Environment<'static>,
    pub config: FileGenConfig,
    pub model_modules: ModelModules,
    pub namespace: QueryNamespace,
    pub request: Request,
}

impl FileGenerator {
    pub fn new(request: &Request) -> Result<FileGenerator, Error> {
        let environment = JinjaEnvironmentBuilder::new(request).build()?;
        let config = FileGenConfig::new(request)?;
        let model_modules = ModelModules::new(request)?;
        let namespace = QueryNamespace::from_request(request)?;

        Ok(FileGenerator {
            environment,
            config,
            namespace,
            model_modules,
            request: request.clone(),
        })
    }

    pub fn render_files(&self) -> Result<Vec<File>, Error> {
        let mut files = self.model_module_files()?;
        files.extend(self.model_dir_entrypoint()?);
        self.query_files(&mut files)?;

        Ok(files)
    }

    fn model_module_files(&self) -> Result<Vec<File>, Error> {
        let mut files = vec![];
        for (name, module) in self.model_modules.model_modules.iter() {
            let filename = format!("models/{}.{}", name, &self.config.extension);

            let content = self
                .environment
                .get_template(MODEL_SCHEMA_FILE)?
                .render(context! {
                    imports => module.imports(),
                    schema => name,
                    models => &module.classes,
                    enums => &module.enums,
                    request => &self.request,
                })?;

            let file = File {
                path: filename,
                content,
            };
            files.push(file);
        }
        Ok(files)
    }

    fn model_dir_entrypoint(&self) -> Result<Option<File>, Error> {
        let Some(filename) = self.config.directory_entrypoint.clone() else {
            return Ok(None);
        };

        let content = self
            .environment
            .get_template(MODELS_DIR_ENTRYPOINT)?
            .render(context!(
                model_modules=> &self.model_modules.model_modules,
                request => &self.request,
            ))?;
        let path = format!("models/{filename}.{}", self.config.extension);
        Ok(Some(File { path, content }))
    }

    fn query_files(&self, files: &mut Vec<File>) -> Result<(), Error> {
        let pathbuf = PathBuf::from("./");
        self._query_files(pathbuf.as_path(), &self.namespace, files)?;
        Ok(())
    }

    fn _query_files(
        &self,
        dir_path: &Path,
        namespace: &QueryNamespace,
        files: &mut Vec<File>,
    ) -> Result<(), Error> {
        let entrypoint = self.directory_entrypoint();
        if namespace.subnamespaces.is_empty() {
            let name = if namespace.name.is_empty() {
                &entrypoint
            } else {
                &namespace.name
            };
            let path = dir_path.join(&name);
            let file = self.render_query_file(&path, namespace)?;
            files.push(file);
        } else {
            let path = dir_path.join(&namespace.name).join(entrypoint);
            let file = self.render_query_file(&path, namespace)?;
            files.push(file);
        }

        for subnamespace in namespace.subnamespaces.values() {
            self._query_files(&dir_path.join(&namespace.name), subnamespace, files)?;
        }

        Ok(())
    }

    fn render_query_file(&self, path: &Path, namespace: &QueryNamespace) -> Result<File, Error> {
        let content = self.environment.get_template(QUERY)?.render(&context! (
            query_namespace => namespace,
            imports => namespace.imports(),
            request => &self.request,
            model_modules => self.model_modules.model_modules,
        ))?;

        Ok(File {
            path: format!("{}.{}", path.to_str().unwrap(), self.config.extension),
            content,
        })
    }

    fn directory_entrypoint(&self) -> String {
        if let Some(entrypoint) = self.config.directory_entrypoint.as_ref() {
            return entrypoint.clone();
        }
        return format!("query.{}", self.config.extension);
    }
}
