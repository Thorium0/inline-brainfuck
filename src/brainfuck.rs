#![macro_use]


macro_rules! brainfuck {
    ($($code:tt)*) => {
        run_brainfuck(stringify!($($code)*));
        
        fn run_brainfuck(code: &str) {
            use std::io;
            use std::io::Write;

            let mut vec = Vec::new();
            vec.push(0 as u8);
            let mut pointer = 0;
            let mut input = String::new();
            print!("Enter optional input: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("failed to read line");
            let input: Vec<char> = input.chars().collect();
            let mut input_index = 0;
            let mut bracket_count = 0;
            let code = code.replace(" ", "").replace("\n", "");
            let code: Vec<char> = code.chars().collect();
            let mut c = 0;
            loop {
                //println!("{} => {} : {} = {}", c, bracket_count, code[c], vec[pointer]);
                match code[c] {
                    '+' => vec[pointer]+=1,
                    '-' => vec[pointer]-=1,
                    '>' => { 
                        pointer+=1;
                        if pointer >= vec.len() {
                            vec.push(0)
                        }
                     },
                     '<' => {
                        pointer-=1;
                     }
                    '.' => {
                        print!("{}", vec[pointer] as char);
                        io::stdout().flush().unwrap();
                    }
                    ',' => {
                        if input_index >= input.len() {
                            break
                        }
                        vec[pointer] = input[input_index] as u8;
                        input_index+=1;
                    },
                    ']' => {
                        if vec[pointer] == 0 {
                            bracket_count -= 1;
                        } else {
                            for _ in 0..bracket_count {
                            
                                loop {
                                    if (code[c]) != '[' {
                                        //println!("{}", code[c]);
                                        c-=1;
                                    }
                                    else {
                                        break
                                    }
                                }
                            }
    
                        }

                        
                        
                    },
                    '[' => {
                        bracket_count += 1;
                        if vec[pointer] == 0 {
                            loop {
                                c+=1;
                                if (code[c]) == '[' {
                                    bracket_count += 1;
                                }
                                else if (code[c]) == ']' {
                                    bracket_count -= 1;
                                    if bracket_count == 0 {
                                        break
                                    }
                                }
                                
                            }
        
                        }

                    },
                    _ => break
                }
                c+=1;
                if c >= code.len() {
                    break
                }
            }


        }

    }
}


