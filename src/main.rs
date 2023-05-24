mod components;

use std::collections::LinkedList;
use std::fs::File;

use components::page::*;
use components::section::*;
use components::content::*;
use components::content::list::*;
use xml::reader::{EventReader, XmlEvent};

macro_rules! content {
    ($name:ident, $value:expr) => {
        Content::$name($value)
    };
}

fn main() -> std::io::Result<()> {
    let file = File::open("./src/resources/index.xml")?;

    let mut page: Option<Page> = None;
    
    let parser = EventReader::new(file);

    let mut actual_section: Option<Section> = None;
    let mut actual_list: LinkedList<ContentList> = LinkedList::new();
    let mut actual_text: Option<String> = None;
    for e in parser {
        match e {
            #[allow(unused_variables)]
            Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                match name.to_string().as_str() {
                    "Page" => {
                        let name = attributes[0].value.to_string();

                        page = Some(Page::new(name));
                    },

                    "Section" => {
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
                        actual_text = Some(String::new());
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
                        let taken = actual_text.take().unwrap().to_string();

                        content_to_add = Some(content!(Text, taken));
                    },
                    
                    "List" => {
                        if actual_list.is_empty() {
                            panic!("List tag closed without opening");
                        }

                        let taken = actual_list.pop_back().unwrap();
   
                        content_to_add = Some(content!(List, taken));
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
                match actual_text.as_mut() {
                    Some(str) => {
                        str.push_str(s.to_string().trim());
                    }
                    None => {
                        // TODO: Add indentation
                        actual_text = Some(s.to_string().trim().to_string());
                    }
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            },
            _ => {}
        }
    }
    
    page.unwrap().show(0); 
    Ok(())
}

#[test]
fn test_page() {
    let mut page = Page::new("Teste".to_string());

    let mut list1 = ContentList {
        list_type: ListType::Unordered,
        content: vec! [
            content!(Text, "texto 1".to_string()),
            content!(Text, "texto 2".to_string()),
            content!(Text, "texto 3".to_string()),
        ],
    };

    let mut list2: ContentList = list1.clone();
    let list3: ContentList = list1.clone();

    list2.add(content!(List, list3));
    list1.add(content!(List, list2));

    let s = Section {
        section_num: 1,
        title: "Teste".to_string(),
        content: vec! [
            content!(Text, "lorem ipsum dolor sit amet consectetour lorem ipsum dolor sit amet consectetour lorem ipsum dolor sit amet consectetour".to_string()),
            Content::BR,
            content!(Text, "texto motivacional muito fodonator 2".to_string()),
            content!(List, list1)
        ],
    };

    page.add(s);
    page.show(0);
}

fn add_in_context(actual_list: &mut LinkedList<ContentList>, actual_section: &mut Option<Section>, content: Content) {
    if !actual_list.is_empty() {
        actual_list.back_mut().unwrap().add(content);
    } else {
        actual_section.as_mut().unwrap().add(content);
    }
}