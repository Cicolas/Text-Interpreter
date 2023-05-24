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
    Text(String),
    List(ContentList)
}

impl Content {
    pub const BR: Content = Content::Text(String::new());
    
    pub fn to_string(&self, indentation: usize) -> String {
        let mut str = String::new();

        match self {
            Content::Text(s) => {
                str.push_str(indent_stdout!(indentation));
                let max_length = super::MAX_TEXT_LEN - indentation*super::INDENTATION_SIZE;

                s   .split("\n")
                    .map(|s| limit_display_size(s, max_length))
                    .for_each(|s| str.push_str((s + "\n").as_str()));
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
                        str.push_str(indent_stdout!(indentation));
                        match l.list_type {
                            ListType::Ordered =>
                                str.push_str((format!("{}.{} ", indentation+1, counter).yellow()).to_string().as_str()),
                            ListType::Unordered =>
                                str.push_str((format!("{} ", "-").yellow()).to_string().as_str()),
                        }
                    }

                    str.push_str(elem.to_string(if should_prefix { 0 } else { indentation + 1 }).as_str());
                }
            }
        }

        str
    }	
}

impl Displayable for Content {
    fn show(&self, indentation: usize) {
        println!("{}", self.to_string(indentation));
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