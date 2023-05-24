use colored::*;
use crate::components::MAX_TEXT_LEN;
use crate::components::content::{ContentDependency, Displayable};
use super::section::Section;

pub struct Page {
    pub name: &'static str,
    pub sections: Vec<Section>
}

impl Page {
    pub fn new(name: &'static str) -> Self {
        Page { name, sections: Vec::new() }
    }
}

impl ContentDependency<Section> for Page {
    fn get_vector(&self) -> &Vec<Section> { &self.sections }
    fn get_vector_mut(&mut self) -> &mut Vec<Section> { &mut self.sections }
}

impl Displayable for Page {
    fn show(&self, indentation: usize) {
        let title_len = self.name.len() + 2;
        print!("{}", " ".repeat(MAX_TEXT_LEN/2-title_len/2));        
        print!("# {}\n\n", self.name.bold());

        self.sections
            .iter()
            .for_each(|elem| {elem.show(indentation)});
    }
}