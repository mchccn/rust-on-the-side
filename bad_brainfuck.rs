#![allow(dead_code)]

const MEM_SIZE: usize = 65535;

fn interpret(code: String) -> () {
    let mut memory: [u8; 65535] = [0; 65535];

    let mut ptr: usize = 0;

    let mut c = 0;

    let mut i = 0;

    while i < code.len() {
        let inst = code.chars().nth(i).unwrap();

        if inst == '>' {
            if ptr == MEM_SIZE - 1 {
                ptr = 0;
            } else {
                ptr += 1;
            }
        }

        if inst == '<' {
            if ptr == 0 {
                ptr = MEM_SIZE - 1;
            } else {
                ptr -= 1;
            }
        }

        if inst == '+' {
            if memory[ptr] == 255 {
                memory[ptr] = 0;
            } else {
                memory[ptr] += 1;
            }
        }

        if inst == '-' {
            if memory[ptr] == 0 {
                memory[ptr] = 255;
            } else {
                memory[ptr] -= 1;
            }
        }

        if inst == '.' {
            println!("{:>3} {}", memory[ptr], memory[ptr] as char);
        }

        if inst == ',' {

        }

        if inst == '[' {
            if memory[ptr] == 0 {
                i += 1;

                while c > 0 || code.chars().nth(i).unwrap() != ']' {
                    if code.chars().nth(i).unwrap() == '[' {
                        c += 1;
                    } else if code.chars().nth(i).unwrap() == ']' {
                        c -= 1;
                    }

                    i += 1;
                }
            }
        }

        if inst == ']' {
            if memory[ptr] == 0 {
                i -= 1;

                while c > 0 || code.chars().nth(i).unwrap() != '[' {
                    if code.chars().nth(i).unwrap() == ']' {
                        c += 1;
                    } else if code.chars().nth(i).unwrap() == '[' {
                        c -= 1;
                    }

                    i -= 1;
                }

                i -= 1;
            }
        }

        i += 1;
    }
}

fn main() {
    interpret("--[+++++++>-->+>+>+<<<->---.>--..>+.<<<.+>->>.+++[.<]".to_string());
}
