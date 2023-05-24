use colored::*;
use super::macros::get_vectors;
use super::content::*;

#[derive(Debug)]
pub struct Section {
    pub section_num: usize,
    pub title: &'static str,
    pub content: Vec<Content>
}

#[allow(unused)]
impl Section {
    pub fn new(section_num: usize, title: &'static str) -> Self {
        Section {
            section_num,
            title,
            content: Vec::new(),
        }
    }
}

impl ContentDependency<Content> for Section {
    get_vectors!(Content);
}

impl Displayable for Section {
    fn show(&self, indentation: usize) {
        let t = format!("{}. ", self.section_num).to_string() + &self.title.clone();

        println!("{}", t.green().bold());
        println!("{}", "-".repeat(t.len()).bright_black());

        self.content
            .iter()
            .for_each(|elem| {elem.show(indentation)});
    }
}