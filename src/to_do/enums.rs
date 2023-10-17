use std::fmt;
use serde::ser::{Serialize, Serializer};

#[allow(dead_code)]
pub enum TaskStatus {
    DONE,
    PENDING
}

#[allow(dead_code)]
// One way to stringfy the results of the enum.
impl TaskStatus {
    
    pub fn stringfy(&self) -> String {
        match self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("Input {} not supported", input_string)
        }
    }
}

// Other, better way to do the same as above
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Self::DONE => write!(f, "DONE"),
            &Self::PENDING => write!(f, "PENDING"),
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_str(&self.stringfy().as_str())?)
    }
}