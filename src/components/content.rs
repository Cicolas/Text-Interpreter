use colored::*;
use std::cmp;
use super::macros::indent_stdout;
use list::*;

pub trait ContentDependency<T: Displayable> {
    fn get_vector(&self) -> &Vec<T>;
    fn get_vector_mut(&mut self) -> &mut Vec<T>;
    
    fn add(&mut self, elem: T) {
        self.get_vector_mut().push(elem);
    }

    fn add_in_position(&mut self, index: usize, elem: T) {
        self.get_vector_mut().insert(index, elem);
    }
}

pub trait Displayable {
    fn show(&self, indentation: usize);
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Content {
    Text(&'static str),
    List(ContentList)
}

impl Content {
    pub const BR: Content = Content::Text("");
}

impl Displayable for Content {
    fn show(&self, indentation: usize) {
        match self {
            Content::Text(s) => {
                indent_stdout!(indentation);
                println!("{}", limit_display_size(s, super::MAX_TEXT_LEN-indentation*super::INDENTATION_SIZE));
            },
            Content::List(l) => {
                let mut counter: usize = 0;
                
                for elem in l.content.iter() {
                    counter += 1;
                    
                    let should_prefix: bool;

                    match elem {
                        Content::Text(_) => should_prefix = true,
                        Content::List(_) => should_prefix = false,
                        // other => assert!(false, "undefined should prefix")
                    }

                    if should_prefix {
                        indent_stdout!(indentation);
                        match l.list_type {
                            ListType::Ordered =>
                                print!("{} ", format!("{}.{}", indentation+1, counter).yellow()),
                            ListType::Unordered =>
                                print!("{} ", "-".yellow())
                        }
                    }

                    elem.show(if should_prefix { 0 } else { indentation + 1 });
                }
            }
        }
    }
}

pub mod list {
    use super::*;
    use super::super::macros::get_vectors;

    #[derive(Debug, Clone)]
    #[allow(unused)]
    pub enum ListType {
        Ordered,
        Unordered
    }
    
    #[derive(Debug, Clone)]
    pub struct ContentList {
        pub list_type: ListType,
        pub content: Vec<Content>
    }
    
    #[allow(unused)]
    impl ContentList {
        pub fn new(list_type: ListType) -> Self {
            ContentList {list_type, content: Vec::new()}
        }
    }
    
    impl ContentDependency<Content> for ContentList {
        get_vectors!(Content);
    }
}
    
fn limit_display_size(string: &str, max_length: usize) -> String {
    let mut str_list: Vec<&str> = Vec::new();

    for i in (0..string.len()).step_by(max_length) {
        let last_char = cmp::min(i + max_length, string.len());
        str_list.push(&string[i..last_char]);
    }

    str_list.join("\n")
}