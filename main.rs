// just print one of the ui texts 
fn computer_ui(text : i32){
    if text == 1 {
        print!("rust brainfuck interpreter");
        print!("\n\nPlease input branfuck sequence:");
    }

    if text == 2 {
        println!("UNKNOWN OPERAND!!!");
    }
}

fn main(){
    computer_ui(1);
        
    let mut memory: [i32; 32] = [0; 32]; // 32 bits of computer memory 
    let mut pointer: i32 = 0;

    // Creating a mutable string to store a command
    let mut line = String::new();    
    
    // reading std in 
    std::io::stdin().read_line(&mut line).unwrap();

    // Running by each instruction operand
    let mut operand_array = line.split("");
    
    // run by all chars in the code
    for s in operand_array {
        
        if s == ">" {
            pointer+=1;
        } else if s == "<" {
            pointer-=1;
        } else if s == "+" {
            memory[pointer as usize]+=1;
        } else if s == "-" {
            memory[pointer as usize]-=1;
        } else if s == "[" {
            println!("start loop");
        } else if s == "]" {
            println!("check finish loop");
        }
        
        println!("{}", s);
    }
    
    // print!("{}", &mut line);

}