use std::collections::LinkedList;
use std::fs::File;

use colored::*;
use xml::EventReader;
use xml::reader::XmlEvent;
use crate::components::MAX_TEXT_LEN;
use crate::components::content::{ContentDependency, Displayable};
use super::content::{Content};
use super::content::list::{ContentList, ListType};
use super::section::Section;

pub struct Page {
    pub name: String,
    pub sections: Vec<Section>
}

impl Page {
    pub fn new(name: String) -> Self {
        Page { name, sections: Vec::new() }
    }

    pub fn from_xml(link: &str) -> Option<Self> {
        let file = File::open(link).unwrap();

        let mut page: Option<Page> = None;
        
        let parser = EventReader::new(file);

        let mut actual_section: Option<Section> = None;
        let mut actual_list: LinkedList<ContentList> = LinkedList::new();
        let mut actual_text: String = String::new();
        for e in parser {
            match e {
                #[allow(unused_variables)]
                Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                    match name.to_string().as_str() {
                        "Page" => {
                            if !page.is_none() {
                                panic!("Page are unique per file and cannot be inside another page tag");
                            }

                            let mut title: Option<String> = None;

                            for attr in attributes.iter() {
                                match attr.name.to_string().as_str() {
                                    "title" => {
                                        title = Some(attr.value.to_string());
                                    },
                                    _ => {/* Don't do nothing to unknown attributes */}
                                }
                            }

                            page = Some(Page::new(title.unwrap()));
                        },

                        "Section" => {
                            if !actual_section.is_none() {
                                panic!("Section tag cannot be inside another section tag");
                            }

                            let mut section_num: Option<usize> = None;
                            let mut name: Option<String> = None;
                            
                            for attr in attributes.iter() {
                                match attr.name.to_string().as_str() {
                                    "number" => {
                                        section_num = Some(attr.value.parse::<usize>().unwrap());
                                    },
                                    "name" => {
                                        name = Some(attr.value.to_string());
                                    },
                                    _ => {/* Don't do nothing to unknown attributes */}
                                }
                            }

                            actual_section = Some(
                                Section::new(section_num.unwrap(), name.unwrap())
                            );
                        }

                        "Text" => {
                            if !actual_text.is_empty() {
                                panic!("Text tag cannot be inside another text tag");
                            }
                        }

                        "List" => {
                            let mut list_type: Option<ListType> = None;

                            for attr in attributes.iter() {
                                match attr.name.to_string().as_str() {
                                    "type" => {
                                        match attr.value.to_string().as_str() {
                                            "Ordered" => {
                                                list_type = Some(ListType::Ordered);
                                            },
                                            "Unordered" => {
                                                list_type = Some(ListType::Unordered);
                                            },
                                            _ => {
                                                panic!("{} is not a valid list type", attr.value.to_string())
                                            }
                                        }
                                    },
                                    _ => {/* Don't do nothing to unknown attributes */}
                                }
                            }

                            actual_list.push_back(ContentList::new(list_type.unwrap()));
                        }

                        "br" => {}

                        _ => {
                            panic!("{} is not a valid tag", name)
                        }
                    }
                },
                Ok(XmlEvent::EndElement { name }) => {
                    let mut content_to_add: Option<Content> = None; 
                    
                    match name.to_string().as_str() {
                        "Section" => {
                            page.as_mut().unwrap().add(actual_section.take().unwrap());
                        },

                        "Text" => {
                            content_to_add = Some(Content::Text(actual_text.to_string()));

                            actual_text.clear();
                        },
                        
                        "List" => {
                            if actual_list.is_empty() {
                                panic!("List tag closed without opening");
                            }

                            let taken = actual_list.pop_back().unwrap();
    
                            content_to_add = Some(Content::List(taken));
                        }

                        "br" => {                    
                            content_to_add = Some(Content::BR);
                        }

                        _ => {}
                    }

                    match content_to_add {
                        Some(content) => {
                            add_in_context(&mut actual_list, &mut actual_section, content);
                        },
                        None => {}
                    }
                },
                Ok(XmlEvent::Characters(s)) => {
                    actual_text.push_str(parse_characters(s).as_str());
                },
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                },
                _ => {}
            }
        }

        page
    }

    pub fn to_string(&self, indentation: usize) -> String {
        let mut str = String::new();

        let title_len = self.name.len() + 2;
        str.push_str(format!("{}", " ".repeat(MAX_TEXT_LEN/2-title_len/2)).as_str());
        str.push_str((format!("# {}\n", self.name).bold()).to_string().as_str());

        self.sections
            .iter()
            .for_each(|elem| {str.push_str(elem.to_string(indentation).as_str());});
        
        str
    }
}

impl ContentDependency<Section> for Page {
    fn get_vector(&self) -> &Vec<Section> { &self.sections }
    fn get_vector_mut(&mut self) -> &mut Vec<Section> { &mut self.sections }
}

impl Displayable for Page {
    fn show(&self, indentation: usize) {
        println!("{}", self.to_string(indentation));
    }
}

fn parse_characters(str: String) -> String {
    let s = str.replace("\r", "").clone();
    let j = s
        .split("\n")
        .map(|s| s.trim())
        .map(|s| if s.is_empty() { "\n" } else { s })
        .collect::<Vec<&str>>();

    j.join("").trim().to_string()
}

fn add_in_context(actual_list: &mut LinkedList<ContentList>, actual_section: &mut Option<Section>, content: Content) {
    if !actual_list.is_empty() {
        actual_list.back_mut().unwrap().add(content);
    } else {
        actual_section.as_mut().unwrap().add(content);
    }
}