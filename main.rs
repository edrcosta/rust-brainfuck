fn computer_ui(text : i32){
    if text == 1 {
        print!("rust brainfuck interpreter");
        print!("\n\nPlease input branfuck sequence:");
    }
}


fn main(){
   
    computer_ui(1);
    
    let memory : [i32; 32] = [0; 32]; // 32 bits of computer memory 

    let mut line = String::new();    
    std::io::stdin().read_line(&mut line).unwrap();

    let mut command_array = line.split("");
    for s in command_array {
        println!("{}", s);
    }
    print!("{}", &mut line);

}