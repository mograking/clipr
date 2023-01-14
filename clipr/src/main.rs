use std::path::Path;
use std::str;
use std::collections::HashMap;
use cli_clipboard;
use std::io::{self, BufRead};
use serde_json;
use std::result;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process::Command;
extern crate dirs;


fn create_dir<P: AsRef<Path>>(path: P) {
    fs::create_dir_all(path)
        .unwrap_or_else(|e| panic!("Error creating dir: {}", e));
}


fn create_file<P: AsRef<Path>>(path: P) {
    fs::File::create(&path).unwrap_or_else(|e| panic!("Error creating file: {}", e));
    fs::write( path, b"{ }" );
}

fn persistentWriteToClipboard( copypasta : String) {

    let mut file = File::create("temp.txt").unwrap();
    file.write_all(&copypasta.as_bytes().to_vec());

    Command::new("xclip" )
        .arg("-i")
        .arg("temp.txt")
        .arg("-selection")
        .arg("clipboard")
        .spawn()
        .expect("xclip failed");


}



fn main() {
    let mut rs:bool=true;

    let clipr_path = dirs::home_dir().expect("error").join(".clipr");
    let clipr_metafile_path =  clipr_path.join("clipr.json");
    rs = clipr_metafile_path.exists();


    if rs == true{
        //println!("Running Clipr");
    }
    else{
        create_dir(&clipr_path);
        create_file(&clipr_metafile_path);
        println!("New clipr user. \n ~/.clipr/clipr.json created \n ");
    }  

    let args: Vec<String> = env::args().collect();


    if args.len() <= 2 {

        let strdata = fs::read_to_string(&clipr_metafile_path).expect("Unable to read file");
        let jsondata : serde_json::Value = serde_json::from_str(&strdata).unwrap();


        let mut jsonkey : String = "".to_owned();

        if args.len() == 1 {

        for (key, value) in jsondata.as_object().unwrap() {
            println!("{:?} \t \t {:?}", key, value.as_str().expect("nothing"));
        }

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        handle.read_line(&mut buffer);
        buffer.pop();
        jsonkey = buffer;
        }
        else{

            jsonkey = args[1].clone();

        }


        let clipdata = jsondata[jsonkey].as_str().unwrap_or_else(|| "no such key");
        println!("{}", clipdata);

        persistentWriteToClipboard(clipdata.to_string());

    }
    else{
        println!("Invalid call");
    }
}
