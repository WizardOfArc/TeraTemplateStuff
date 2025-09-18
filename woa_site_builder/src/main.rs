use serde_json::Value;
use std::fs;
use tera::{Context, Tera};

struct PageMapping {
    template: String,
    render_target: String,
    context_json_file: String,
}

fn main() {
    println!("Hello, world!");
    use std::env;
    println!("Current working directory: {:?}", env::current_dir());
    let templates_dir = std::env::var("TERA_TEMPLATES").expect("TERA_TEMPLATES must be set");
    let template_blob = format!("{}/**/*.html", templates_dir);
    let data_dir = std::env::var("WOA_DATA_DIR").expect("WOA_DATA_DIR must be set");
    let tera = match Tera::new(&template_blob) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    println!("Parsed Templates:");
    for name in tera.get_template_names() {
        println!("{}", name);
    }
    let mappings: Vec<PageMapping> = vec![
        PageMapping {
            template: "index.html".to_string(),
            context_json_file: "index.json".to_string(),
            render_target: "output/index.html".to_string(),
        },
        PageMapping {
            template: "blog.html".to_string(),
            render_target: "output/blog.html".to_string(),
            context_json_file: "blog_posts.json".to_string(),
        },
    ];
    for mapping in mappings.iter() {
        println!(
            "Read from {} and {} and write to {}",
            mapping.template, mapping.context_json_file, mapping.render_target
        );
        let file_path = format!("{}/{}", data_dir, mapping.context_json_file);
        let json_string = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {}: {}", file_path, e);
                std::process::exit(1);
            }
        };
        let json_value: Value = serde_json::from_str(&json_string).unwrap();
        let ctx = Context::from_value(json_value).unwrap();
        let rendered_output = tera.render(&mapping.template, &ctx).unwrap();
        println!(
            "write {} to the file {}",
            rendered_output, mapping.render_target
        );
    }
    // TODO: write to file
}
