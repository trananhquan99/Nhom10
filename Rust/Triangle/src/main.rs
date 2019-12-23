use std::io;

fn main() {
    let mut buf = String::new();    
    println!("angle: ");    
    io::stdin().read_line(&mut buf).ok().expect("error!");   
    let angle: Result<f64, _> = buf.trim().parse();     
    let mut buf1 =String::new();    
    println!("side_length: ");      
    io::stdin().read_line(&mut buf1)
    .ok()
    .expect("error!");      
    let side_lenght: Result<f64, _> = buf1.trim().parse();  
    
    println!("{}",angle.ok().expect("Error"));
    println!("{}",side_lenght.ok().expect("Error"));
    let mut hyp :: f64= side_lenght / angle.sin();
    
    println!("Hypotennuse: {}",hyp::f64.ok().expect("Error"));
    
}
