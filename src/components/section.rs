use colored::*;
use super::macros::get_vectors;
use super::content::*;

#[derive(Debug)]
pub struct Section {
    pub section_num: usize,
    pub title: String,
    pub content: Vec<Content>
}

#[allow(unused)]
impl Section {
    pub fn new(section_num: usize, title: String) -> Self {
        Section {
            section_num,
            title,
            content: Vec::new(),
        }
    }

    pub fn to_string(&self, indentation: usize) -> String {
        let mut str = String::new();
        let t = format!("\n{}. ", self.section_num).to_string() + &self.title.clone();

        str.push_str(format!("{}\n", t.green().bold()).as_str());
        str.push_str(format!("{}\n", "-".repeat(t.len()).bright_black()).as_str());

        self.content
            .iter()
            .for_each(|elem| {str.push_str(elem.to_string(indentation).as_str());});
        
        str
    }
}

impl ContentDependency<Content> for Section {
    get_vectors!(Content);
}

impl Displayable for Section {
    fn show(&self, indentation: usize) {
        println!("{}", self.to_string(indentation));
    }
}