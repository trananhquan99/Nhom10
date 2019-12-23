$ git clone https://github.com/trananhquan99/Tr-n-Anh-Qu-n.git
use std::io;

fn main() {  
    let mut buf = String::new();    
    println!("Your name: ");    
    io::stdin().read_line(&mut buf).ok().expect("error!");      
    let mut buf1 =String::new();    
    println!("Your age: ");      
    io::stdin().read_line(&mut buf1)
    .ok()
    .expect("error!");      
    let buf1_i32: Result<u32, _> = buf1.trim().parse();  
    println!("Hello, {}", buf);  
    println!("Your age is {}", buf1_i32.ok().expect("Error"));
}