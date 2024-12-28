mod history;
use clipboard::Clipboard;



pub fn clipboard() {
    let mut history = History::new().expect("Failed to initialize history");
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");


    let mut history_arr = match history.load_history() { 
        Ok(history) => history.get_history(),
        Err(error) => { 
            println!("{}.\n .", error);
            return;
        }
    }; 

    let mut prev = String::new();

    loop { 
        if let Ok(entry) = clipboard.get() {
            if entry != prev {
                history.add_entry(entry.clone());
                prev = entry;
            }
             else { 
                continue;
            }
        } else {
            println!("Failed to get clipboard contents");
            continue;
        }
        thread::sleep(Duration::from_secs(1));
    } 


}

