use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

struct TemplatePlaceholders {
    template: String,
    file_to_write: String,
    placeholders: Vec<String>,
}

fn main() -> std::io::Result<()> {

    let mut plugin_metadata : HashMap<String, String> = HashMap::new();

    // A 2d array defining the message to show to the user in the first position of every element
    // and the name of the attribute to set in the HashMap
    let data_to_request: [[&str; 2]; 6] = [
        ["Plugin name (skewer-case): ", "plugin_name"],
        ["Composer package name (snake_case): ", "composer_package_name"],
        ["Meta plugin name (wp): ", "meta_plugin_name"],
        ["Meta plugin description (wp): ", "meta_plugin_description"],
        ["Plugin class name (PascalCase): ", "plugin_class_name"],
        ["Plugin shortcode tag (snake_case): ", "plugin_shortcode_tag"]
    ];

    for request in data_to_request {
        let mut user_input : String = String::new();
        // Print the request
        print!("{}", request[0]);
        // flush stdout
        std::io::stdout().flush().ok().expect("Could not flush stdout");
        std::io::stdin().read_line(&mut user_input).expect("Error: unable to read user input");
        trim_newline(&mut user_input);
        plugin_metadata.insert(request[1].to_string(), user_input);
    }

    create_dir_structure(&plugin_metadata["plugin_name"]);

    // load the templates to the binary
    let composer_template = std::include_str!("templates/composer.json");
    let loader_template = std::include_str!("templates/loader.php");
    let source_class_template = std::include_str!("templates/source_class.php");


    let templates_and_placeholders : [TemplatePlaceholders; 3] = [
        TemplatePlaceholders {
            template: composer_template.to_string(),
            file_to_write: format!("{}/composer.json", plugin_metadata["plugin_name"]),
            placeholders: [
                "composer_package_name".to_string(),
            ].to_vec(),
        },
        TemplatePlaceholders {
            template: loader_template.to_string(),
            file_to_write: format!("{}/{}.php", plugin_metadata["plugin_name"], plugin_metadata["plugin_name"]),
            placeholders: [
                "meta_plugin_name".to_string(),
                "meta_plugin_description".to_string(),
                "plugin_class_name".to_string(),
            ].to_vec(),
        },
        TemplatePlaceholders {
            template: source_class_template.to_string(),
            file_to_write: format!("{}/src/{}.php", plugin_metadata["plugin_name"], plugin_metadata["plugin_class_name"]),
            placeholders: [
                "plugin_class_name".to_string(),
                "plugin_shortcode_tag".to_string(),
            ].to_vec(),
        }
    ];

    for template in &templates_and_placeholders {
        // initialize the string to contain the parsed template
        let mut parsed_template : String = template.template.clone();
        // create a file for 
        let mut plugin_file = File::create(&template.file_to_write)?;

        // iterate over the template placeholders to replace
        for placeholder in &template.placeholders {
            // replace for every placeholder
            parsed_template = replace_template_placeholders(&parsed_template, &placeholder, &plugin_metadata[placeholder]);
        }
        // write parsed contents to file
        plugin_file.write_all(parsed_template.as_bytes())?;
    }

    println!("Completed plugin setup!");

    return Ok(());
}

fn replace_template_placeholders(template: &String, placeholder: &String, replace_with: &String) -> String{
    let template_regex = Regex::new(format!(r"\[\[{}\]\]", placeholder).as_str()).unwrap();

    return template_regex.replace_all(template.as_str(), replace_with.as_str());
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn create_dir_structure(plugin_name: &String){
    // create the base path "plugin-name"
    std::fs::create_dir(plugin_name).expect("Failed to create base directory. Are you missing permissions or does the directory already exist?");
    // create the src dir inside the base dir "plugin-name/src"
    std::fs::create_dir(format!("{}/src", plugin_name)).expect("Failed to create base directory. Are you missing permissions or does the directory already exist?");
}