mod components;

use components::page::*;
use components::section::*;
use components::content::*;
use components::content::list::*;

fn main() {
    let file_path = std::env::args().nth(1).unwrap();

    let page: Option<Page> = Page::from_xml(file_path.trim());
    
    page.unwrap().show(0);
}

#[test]
fn test_page() {
    let mut page = Page::new("Teste".to_string());

    let mut list1 = ContentList {
        list_type: ListType::Unordered,
        content: vec! [
            Content::Text("texto 1".to_string()),
            Content::Text("texto 2".to_string()),
            Content::Text("texto 3".to_string()),
        ],
    };

    let mut list2: ContentList = list1.clone();
    let list3: ContentList = list1.clone();

    list2.add(Content::List(list3));
    list1.add(Content::List(list2));

    let s = Section {
        section_num: 1,
        title: "Teste".to_string(),
        content: vec! [
            Content::Text("lorem ipsum dolor sit amet consectetour lorem ipsum dolor sit amet consectetour lorem ipsum dolor sit amet consectetour".to_string()),
            Content::BR,
            Content::Text("texto motivacional muito fodonator 2".to_string()),
            Content::List(list1)
        ],
    };

    page.add(s);
    page.show(0);
}