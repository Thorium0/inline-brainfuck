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
            let mut bracket_count = 0; // used for forward jumps only
            let code = code.replace(" ", "").replace("\n", "");
            let code: Vec<char> = code.chars().collect();
            let mut c = 0;
            loop {
                // Stop if we've reached the end of the program
                if c >= code.len() { break; }
                //println!("{} => {} : {} = {}", c, bracket_count, code[c], vec[pointer]);
                //println!("{:?}", vec);
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
                        if pointer == 0 {
                            // Clamp at 0 to avoid underflow. Brainfuck tapes are typically left-bounded in simple interpreters.
                            // Alternatively, we could grow to the left with vec.insert(0, 0), but clamping is simpler and safe.
                            pointer = 0;
                        } else {
                            pointer-=1;
                        }
                     }
                    '.' => {
                        print!("{}", vec[pointer] as char);
                        io::stdout().flush().unwrap();
                    }
                    ',' => {
                        if input_index >= input.len() {
                            vec[pointer] = 0;
                        } else {
                            vec[pointer] = input[input_index] as u8;
                            input_index+=1;
                        }
                    },
                    ']' => {
                        if vec[pointer] != 0 {
                            // Jump back to matching '[' taking nesting into account
                            let mut depth = 1; // we're at a ']'
                            while c > 0 {
                                c -= 1;
                                match code[c] {
                                    ']' => depth += 1,
                                    '[' => {
                                        depth -= 1;
                                        if depth == 0 { break; }
                                    }
                                    _ => {}
                                }
                            }
                            // After this, the main loop will c+=1, moving to the instruction after '['
                        } else {
                            // If current cell is zero, just continue execution past ']'
                            // (no action needed; do not modify c here)
                        }

                        
                        
                    },
                    '[' => {
                        bracket_count += 1;
                        if vec[pointer] == 0 {
                            loop {
                                c+=1;
                                if c >= code.len() {
                                    // unmatched '['; stop execution safely
                                    break;
                                }
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
                    _ => ()
                }
                c+=1;
                if c >= code.len() {
                    break
                }
            }

            println!("{:?}", vec);
        }

    }
}


