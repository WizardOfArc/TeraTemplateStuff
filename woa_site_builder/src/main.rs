use serde::Deserialize;
use serde_json::Value;
use std::env;
use std::fmt;
use std::fs;
use std::path;
use tera::{Context, Tera};

mod tera_filters;

#[derive(Debug)]
enum PageMappingError {
    UnableToRead(String),
    UnableToParse(String),
}

impl fmt::Display for PageMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PageMappingError::UnableToRead(message) => {
                write!(f, "Could not read mapping file: {}", message)
            }
            PageMappingError::UnableToParse(message) => {
                write!(f, "Could not parse JSON into Page Mapping: {}", message)
            }
        }
    }
}

#[derive(Deserialize)]
struct PageMapping {
    template: String,
    context_json_file: String,
}

#[derive(Debug)]
enum TemplateRenderError {
    UnableToReadDataFile(String),
    UnableToParseDataJson(String),
    UnableToRenderTemplate(String),
    UnableToBeWriteRenderedFile(String),
    UnableToCreateContext(String),
}

impl fmt::Display for TemplateRenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplateRenderError::UnableToBeWriteRenderedFile(message) => {
                write!(f, "Could not write rendered template: {}", message)
            }
            TemplateRenderError::UnableToParseDataJson(message) => {
                write!(f, "JSON could not be parsed: {}", message)
            }
            TemplateRenderError::UnableToReadDataFile(message) => {
                write!(f, "JSON file could not be read: {}", message)
            }
            TemplateRenderError::UnableToRenderTemplate(message) => {
                write!(f, "Template could not be rendered: {}", message)
            }
            TemplateRenderError::UnableToCreateContext(message) => {
                write!(
                    f,
                    "Tera Context unable to be created from the JSON: {}",
                    message
                )
            }
        }
    }
}

fn render_template(
    mapping: &PageMapping,
    tera: &Tera,
    target_dir: &str,
    data_dir: &str,
) -> Result<String, TemplateRenderError> {
    let file_path = path::Path::new(data_dir).join(&mapping.context_json_file);
    let json_string = fs::read_to_string(&file_path)
        .map_err(|e| TemplateRenderError::UnableToReadDataFile(e.to_string()))?;
    let json_value: Value = serde_json::from_str(&json_string)
        .map_err(|e| TemplateRenderError::UnableToParseDataJson(e.to_string()))?;
    let ctx = Context::from_value(json_value)
        .map_err(|e| TemplateRenderError::UnableToCreateContext(e.to_string()))?;
    let rendered_output = tera
        .render(&mapping.template, &ctx)
        .map_err(|e| TemplateRenderError::UnableToRenderTemplate(format!("{:?}", e.kind)))?;
    let target_path = path::Path::new(target_dir).join(&mapping.template);
    match fs::write(&target_path, rendered_output) {
        Ok(()) => Ok(target_path.display().to_string()),
        Err(e) => Err(TemplateRenderError::UnableToBeWriteRenderedFile(
            e.to_string(),
        )),
    }
}

fn load_page_mapping(data_dir: &str) -> Result<Vec<PageMapping>, PageMappingError> {
    let mapping_file_path = path::Path::new(data_dir).join("page_mappings.json");
    let mapping_json_string = fs::read_to_string(&mapping_file_path)
        .map_err(|e| PageMappingError::UnableToRead(e.to_string()))?;
    let mappings: Vec<PageMapping> = serde_json::from_str(&mapping_json_string)
        .map_err(|e| PageMappingError::UnableToParse(e.to_string()))?;
    Ok(mappings)
}

fn main() {
    let templates_dir = env::var("TERA_TEMPLATES").expect("TERA_TEMPLATES must be set");
    let template_blob = format!("{}/**/*.html", templates_dir);
    let data_dir = env::var("WOA_DATA_DIR").expect("WOA_DATA_DIR must be set");
    let target_dir = env::var("WOA_TARGET_DIR").expect("WOA_TARGET_DIR must be set");
    let mut tera = match Tera::new(&template_blob) {
        Ok(t) => t,
        Err(e) => {
            println!("Tera Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    tera.register_filter("ogham", tera_filters::to_ogham);
    println!("Registered Templates:");
    for name in tera.get_template_names() {
        println!("{}", name);
    }
    println!("--------\n");
    let mappings: Vec<PageMapping> = match load_page_mapping(&data_dir) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Error loading mapping: {}", e);
            std::process::exit(1);
        }
    };

    println!("Rendering...");
    for mapping in mappings.iter() {
        match render_template(mapping, &tera, &target_dir, &data_dir) {
            Ok(outputted) => println!("{} ok", outputted),
            Err(e) => {
                println!(
                    "!!!! <<< {} FAILED to render: {} >>> !!!!!",
                    mapping.template, e
                );
                break;
            }
        }
    }
}
