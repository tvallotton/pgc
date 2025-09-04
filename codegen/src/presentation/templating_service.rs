use std::{mem::take, sync::Arc};

use minijinja::{Environment, context};

use crate::{
    error::Error,
    ir::{Ir, ModelModule, QueryNamespace},
    presentation::{
        FileGeneratorService, environment::env, file_generation_config::TemplateGenConfig,
    },
    response::File,
};

pub struct TemplatingService {
    pub ir: Ir,
    pub config: TemplateGenConfig,
    pub environment: Environment<'static>,
}

impl FileGeneratorService for TemplatingService {
    fn generate(&mut self) -> Result<Vec<File>, Error> {
        let mut files = self.model_module_files()?;
        self.add_query_files(&mut files)?;
        self.include_other_templates(&mut files)?;
        files.push(self.add_model_entrypoint()?);
        return Ok(files);
    }
}

impl TemplatingService {
    pub fn new(ir: Ir, config: TemplateGenConfig) -> Result<Self, Error> {
        let environment = env(ir.clone(), &config)?;

        Ok(TemplatingService {
            ir,
            config,
            environment,
        })
    }

    fn include_other_templates(&self, files: &mut Vec<File>) -> Result<(), Error> {
        for file in &self.config.other_templates {
            let content = self.environment.render_named_str(
                &file.path,
                &file.content,
                context! {
                    ir => self.ir
                },
            )?;

            files.push(File {
                path: file.path.clone(),
                content: content,
            });
        }
        Ok(())
    }

    fn model_module_files(&self) -> Result<Vec<File>, Error> {
        let mut files = vec![];
        for module in self.ir.model_modules.model_modules.values() {
            self.add_model_module_file(&mut files, module)?;
        }
        Ok(files)
    }

    pub fn add_model_module_file(
        &self,
        files: &mut Vec<File>,
        module: &ModelModule,
    ) -> Result<(), Error> {
        let filename = format!("models/{}.{}", module.name, &self.config.file_extension);

        let content = self.environment.get_template("model")?.render(context! {
            this_module => ["models", &module.name],
            used_types => module.used_types(),
            model_module => module,
            ir => self.ir,
        })?;

        files.push(File {
            path: filename,
            content,
        });
        Ok(())
    }

    fn add_model_entrypoint(&self) -> Result<File, Error> {
        let content = self
            .environment
            .get_template("model_init")?
            .render(context!(
                ir => self.ir,
                this_module => ["models", self.config.model_directory_entrypoint]
            ))?;
        let path = format!("models/{}", self.config.model_directory_entrypoint);
        Ok(File { path, content })
    }

    pub fn add_query_files(&self, files: &mut Vec<File>) -> Result<(), Error> {
        let namespace = &self.ir.query_namespace;
        let mut path = vec![];
        self.add_query_namespaces_recursively(files, &mut path, &namespace)?;
        Ok(())
    }

    fn add_query_namespaces_recursively(
        &self,
        files: &mut Vec<File>,
        path: &mut Vec<Arc<str>>,
        namespace: &QueryNamespace,
    ) -> Result<(), Error> {
        self.add_query_namespace(files, path, namespace)?;

        for (name, subnamespace) in namespace.subnamespaces.iter() {
            path.push(name.clone());
            self.add_query_namespaces_recursively(files, path, subnamespace)?;
            path.pop();
        }
        Ok(())
    }

    pub fn add_query_namespace(
        &self,
        files: &mut Vec<File>,
        module_segments: &Vec<Arc<str>>,
        namespace: &QueryNamespace,
    ) -> Result<(), Error> {
        let content = self
            .environment
            .get_template("query")
            .unwrap()
            .render(context! {
                query_namespace => namespace,
                this_module => module_segments,
                ir => self.ir,
                used_types => namespace.used_types(),
            })?;

        let path;

        if namespace.subnamespaces.len() == 0 {
            path = format!(
                "{}.{}",
                module_segments.join("/"),
                self.config.file_extension
            );
        } else {
            path = format!(
                "{}/{}",
                module_segments.join("/"),
                self.config.query_directory_entrypoint
            );
        }

        files.push(File { path, content });
        Ok(())
    }
}
