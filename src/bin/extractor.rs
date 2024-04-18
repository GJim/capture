use std::path::Path;
use clap::{Command, Arg};
use tree_sitter::{Parser, Tree, TreeCursor};

fn find_function_range(cursor: &mut TreeCursor, src: &str, target_function: &str) -> Option<(usize, usize)> {
    let mut stack: Vec<TreeCursor> = Vec::new();
    stack.push(cursor.clone());

    while let Some(mut cur_cursor) = stack.pop() {
        let node = cur_cursor.node();

        if node.kind() == "identifier" && node.utf8_text(src.as_bytes()).unwrap() == target_function {
            if let Some(parent) = node.parent() {
                if parent.kind() == "method_declaration" {
                    return Some((parent.start_byte(), parent.end_byte()));
                }
            }
        }

        // Explore children, then siblings, then move back to parent
        if cur_cursor.goto_first_child() {
            stack.push(cur_cursor.clone());
            while cur_cursor.goto_next_sibling() {
                stack.push(cur_cursor.clone());
            }
        }
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Rust Code Snippet Extractor")
        .version("1.0")
        .about("Extracts code snippets from source files")
        .arg(Arg::new("filepath")
            .short('f')
            .long("filepath")
            .help("Sets the input file to use")
            .required(true))
        .arg(Arg::new("function")
            .short('t')
            .long("target")
            .help("The target function name to extract")
            .required(true))
        .get_matches();

    let file_path = matches.get_one::<String>("filepath").unwrap();
    let target_function = matches.get_one::<String>("function").unwrap();

    // Proceed with reading the file, parsing, and other operations
    // println!("Filepath: {}", file_path);
    // println!("Target function: {}", target_function);

    let file_lang = match Path::new(file_path).extension() {
        Some(ext) => {
            match ext.to_str() {
                Some(ext_str) => ext_str,
                None => Err("Invalid file extension")?
            }
        },
        None => Err("No file extension found")?
    };

    // read source code
    let src = std::fs::read_to_string(file_path)?;
    
    // parse source code
    let parser_lang = match file_lang {
        "java" => tree_sitter_java::language(),
        "cs" => tree_sitter_c_sharp::language(),
        _ => Err("Unsupported language")?
    };
    let mut parser = Parser::new();
    parser.set_language(parser_lang)?;

    let parse_tree: Tree = parser.parse(&src, None).unwrap();
    let mut cursor = parse_tree.walk();
    
    match find_function_range(&mut cursor, &src, &target_function) {
        Some((start, end)) => {
            // println!("start: {}, end: {}", start, end);
            println!("{}", &src[start..end]);
        },
        None => {
            println!("target not found");
        }
    }
    Ok(())
}
