const MAX_TEXT_LEN: usize = 50;
const INDENTATION_SIZE: usize = 2; 

pub mod section;
pub mod content;
pub mod page;

mod macros {
    macro_rules! indent_stdout {
        ($indent:expr) => {
            print!("{}", " ".repeat($indent*super::INDENTATION_SIZE));
        };
    }
    
    macro_rules! get_vectors {
        ($type:ident) => {
            fn get_vector(&self) -> &Vec<$type> { &self.content }
            fn get_vector_mut(&mut self) -> &mut Vec<$type> { &mut self.content }    
        };
    }

    pub(super) use indent_stdout;
    pub(super) use get_vectors;
}
