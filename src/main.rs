mod components;

use components::page::*;
use components::section::*;
use components::content::*;
use components::content::list::*;
use xml::*;

macro_rules! content {
    ($name:ident, $value:expr) => {
        Content::$name($value)
    };
}

fn main() {
    let parser = xml::Parser::new();
}

#[test]
fn test_page() {
    let mut page = Page::new("Teste");

    let mut list1 = ContentList {
        list_type: ListType::Unordered,
        content: vec! [
            content!(Text, "texto 1"),
            content!(Text, "texto 2"),
            content!(Text, "texto 3"),
        ],
    };

    let mut list2: ContentList = list1.clone();
    let list3: ContentList = list1.clone();

    list2.add(content!(List, list3));
    list1.add(content!(List, list2));

    let s = Section {
        section_num: 1,
        title: "Teste",
        content: vec! [
            content!(Text, "lorem ipsum dolor sit amet consectetour lorem ipsum dolor sit amet consectetour lorem ipsum dolor sit amet consectetour"),
            Content::BR,
            content!(Text, "texto motivacional muito fodonator 2"),
            content!(List, list1)
        ],
    };

    page.add(s);
    page.show(0);
}