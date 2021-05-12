mod mod_under; // now private
pub mod nested_mod;

pub mod other {
    // indirect access 
    use super::mod_under::{some_function as other_function};

    pub fn some_function() {
        other_function();
    }

    
}