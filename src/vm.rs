use std::io::{Read, Write};


pub struct Machine<'a> {
    int_ptr: usize,
    data_ptr: usize,
    memory: [u8; 30000],
    buffer: [u8; 1],
    code: &'a str,
    input: Box<Read>,
    output: Box<Write>,
}

impl Machine<'_> {
    pub fn new(code: &str, input: Box<Read>, output: Box<Write>) -> Machine {
        Machine {
            int_ptr: 0,
            data_ptr: 0,
            memory: [0; 30000],
            buffer: [0; 1],
            code: code,
            input: input,
            output: output,
        }
    }

    pub fn execute(&mut self) {
        while self.int_ptr < self.code.len() {
            let instruction = self.code.chars().nth(self.int_ptr);
            
            match instruction {
                Some('+') => self.memory[self.data_ptr] += 1,
                Some('-') => self.memory[self.data_ptr] -= 1,
                Some('>') => self.data_ptr += 1,
                Some('<') => self.data_ptr -= 1,
                Some(',') => self.read_char(),
                Some('.') => self.put_char(),
                Some('[') => self.jmp_zero(),
                Some(']') => self.jmp_not_zero(),
                None => panic!("error parsing the source code"),
                _ => {}
            };

            self.int_ptr += 1;
        }
    }

    fn jmp_zero(&mut self) {
        if self.memory[self.data_ptr] == 0 {
            let mut depth = 1;
            while depth != 0 {
                self.int_ptr += 1;
                let instruction = self.code.chars().nth(self.int_ptr);
                match instruction {
                    Some('[') => depth += 1,
                    Some(']') => depth -= 1,
                    None => panic!("error parsing the source code"),
                    _ => {}
                }
            }
        };
    }

    fn jmp_not_zero(&mut self) {
        if self.memory[self.data_ptr] != 0 {
            let mut depth = 1;
            while depth != 0 {
                self.int_ptr -= 1;
                let instruction = self.code.chars().nth(self.int_ptr);
                match instruction {
                    Some(']') => depth += 1,
                    Some('[') => depth -= 1,
                    None => panic!("error parsing the source code"),
                    _ => {}
                }
            }
        };
    }

    fn read_char(&mut self) {
        //self.buffer[0] = self.memory[self.data_ptr]
        // let buf : &mut [u8] = ;
        let r = self.input.read(&mut (self.buffer));
        match r {
            // This probably errors out
            Ok(bytes_read) => {
                if bytes_read == 1 {
                    self.memory[self.data_ptr] = self.buffer[0]
                } else {
                    panic!("wrong number of bytes read")
                }
            }
            Err(e) => panic!(e),
        }
    }

    fn put_char(&mut self) {
        self.buffer[0] = self.memory[self.data_ptr];
        let r = self.output.write(&(self.buffer));
        match r {
            Ok(bytes_written) => {
                if bytes_written != 1 {
                    panic!("wrong number of bytes written")
                }
            }
            Err(e) => panic!(e),
        }
        let f = self.output.flush();
        match f {
            Ok(()) => {},
            Err(e) => panic!(e),
        }
    }
}
