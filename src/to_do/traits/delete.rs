use crate::state::write_to_file;

use serde_json::{Map, value::Value};

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file("./state.json", state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}