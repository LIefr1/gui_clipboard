use std::fs::File;
use std::io::Write;
use std::any::Any;


pub struct History { 
    history: Vec<String>
}

impl History { 
    pub fn new() -> Self { 

        Self { 
            history: Vec::new()
        }
    }

    pub fn add_entry(&mut self, entry: Box<dyn Any>) {
        self.history.push(entry);
    }
    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }
    
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
    pub fn save_history(&self) {
        let mut file = File::create("history.txt").unwrap();
        for entry in &self.history {
            file.write_all(entry.to_string().as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    }

    pub fn load_history(&mut self) {
        let mut file_result = File::open("history.txt").unwrap();
        let file = match file_result {
            Ok(file) => file,
            Err(error) => { 
                println!("{}.\n History file not found.", error);
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        for line in contents.lines() {
            self.history.push(line.to_string());
        }
    }

    

}