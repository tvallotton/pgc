use std::sync::Arc;

use minijinja::{context, Environment};

use crate::{
    error::Error,
    ir::{Ir, ModelModule, QueryNamespace},
    presentation::python::file_generation_config::FileGenerationConfig,
    response::File,
};

pub struct FileGeneratorService {
    pub ir: Ir,
    pub config: FileGenerationConfig,
    pub environment: Environment<'static>,
}

impl FileGeneratorService {
    fn files(&self) -> Result<Vec<File>, Error> {
        let mut files = self.model_module_files()?;
        self.add_query_files(&mut files);
        files.push(self.add_model_entrypoint()?);
        return Ok(files);
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
            path => ["models", &module.name],
            used_types => module.used_types(),
            module => module,
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
            .get_template("model_dir")?
            .render(context!(
                ir => self.ir,
            ))?;
        let path = format!("models/{}", self.config.model_directory_entrypoint);
        Ok(File { path, content })
    }

    pub fn add_query_files(&self, files: &mut Vec<File>) {
        let namespace = &self.ir.query_namespace;
        self.add_query_namespaces_recursively(files, &vec![], &namespace);
    }

    fn add_query_namespaces_recursively(
        &self,
        files: &mut Vec<File>,
        path: &Vec<Arc<str>>,
        namespace: &QueryNamespace,
    ) -> Result<(), Error> {
        self.add_query_namespace(files, path, namespace)?;

        for (name, subnamespace) in namespace.subnamespaces.iter() {
            let mut path = path.clone();
            path.push(name.clone());
            self.add_query_namespaces_recursively(files, &path, subnamespace);
        }
        Ok(())
    }

    pub fn add_query_namespace(
        &self,
        files: &mut Vec<File>,
        path: &Vec<Arc<str>>,
        namespace: &QueryNamespace,
    ) -> Result<(), Error> {
        let content = self
            .environment
            .get_template("query")
            .unwrap()
            .render(context! {
                query_namespace => namespace,
                path => path,
                ir => self.ir,
            })?;

        let path = format!(
            "{}.{}",
            path.join("/"),
            self.config.query_directory_entrypoint
        );

        files.push(File { path, content });
        Ok(())
    }
}
