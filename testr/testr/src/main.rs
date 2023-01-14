use std::fs::File;
use std::io::prelude::*;
use std::process::Command;


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

    // Command::new("rm" )
    //   .arg("temp.txt")
    //   .spawn()
    //   .expect("rm failed");

}



fn main() {

  persistentWriteToClipboard("test overwrite".to_string());
}
