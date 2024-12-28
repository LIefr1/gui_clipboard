#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 

mod backend;
use backend::clipboard;

fn main()  {
    clipboard::clipboard();

}




