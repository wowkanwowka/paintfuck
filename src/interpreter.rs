use std::fmt::Debug;

pub struct Interpreter {
    cur_x_: i64,
    cur_y_: i64,
    code_: String,
    cur_cmd_index_: i64,
    iterations_: i64,
    width_: i64,
    height_: i64,
    one_bits_: std::collections::HashSet<(i64, i64)>,
}

fn find_match(
    text: &String,
    to_match_char: char,
    match_char: char,
    start_index: i64,
    step: i64) -> Result<i64, std::io::Error> {

    assert_ne!(to_match_char, match_char);
    assert_eq!(text.chars().nth(start_index as usize).expect(format!("couldn't get char at index {}", start_index).as_str()), to_match_char);
    let mut cur_index = start_index + step;
    let mut stack: i64 = 1;
    let is_inbounds = |index: i64| -> bool {
        println!("checking if {} is inbounds", index);
        index > -1 && index < text.len() as i64
    };
    while is_inbounds(cur_index) && stack > 0 {
        println!("stack is {}", stack);
        if text.chars().nth(cur_index as usize ).expect(format!("couldn't get char at index {}", cur_index).as_str()) == to_match_char {
            stack += 1;
        } else if text.chars().nth( cur_index as usize ).expect(format!("couldn't get char at index {}", cur_index).as_str()) == match_char {
            stack -= 1;
        }
        if stack > 0 {
            cur_index += step;
        }
    };
    if !is_inbounds(cur_index) {
       return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "no match found") );
    }
    Ok(cur_index)
}

impl Interpreter {
    pub fn new( code: &str, iterations: i64, width: i64, height: i64 ) -> Interpreter {
        Interpreter {
            cur_x_: 0,
            cur_y_: 0,
            code_: code.chars().collect(),
            cur_cmd_index_: 0,
            iterations_: iterations,
            width_: width,
            height_: height,
            one_bits_: std::collections::HashSet::new()
        }
    }

    fn pointer_is_within_range(&mut self) -> bool {
        self.cur_x_ >= 0
            && self.cur_x_ < self.width_
            && self.cur_y_ >= 0
            && self.cur_y_ < self.height_
    }

    fn process_west(&mut self) -> bool {
        self.cur_x_ -= 1;
        self.pointer_is_within_range()
    }

    fn process_east(&mut self) -> bool {
        self.cur_x_ += 1;
        self.pointer_is_within_range()
    }

    fn process_north(&mut self) -> bool {
        self.cur_y_ -= 1;
        self.pointer_is_within_range()
    }

    fn process_south(&mut self) -> bool {
        self.cur_y_ += 1;
        self.pointer_is_within_range()
    }

    fn process_flip(&mut self) -> bool {
        let key = (self.cur_x_, self.cur_y_);
        if self.one_bits_.contains(&key) {
            println!("contained");
            self.one_bits_.remove(&key);
        } else {
            println!("didn't contain");
            self.one_bits_.insert(key);
        }
        println!("trying to return true");
        true
    }

//    fn process_lbrace(&mut self) -> bool {
//
//    }
//
//    fn process_rbrace(&mut self) -> bool {
//
//    }

    pub fn process_next(&mut self) -> Result<bool, std::io::Error> {
        let cmd = self.code_.chars().nth( self.cur_cmd_index_ as usize).expect("couldn't pop from code");
        self.cur_cmd_index_ += 1;
        self.iterations_ -= 1;
        let mut is_ok = true;
        if cmd == 'w' {
            println!("westside");
            is_ok &= self.process_west();
        } else if cmd == 'e' {
            println!("eastside");
            is_ok &= self.process_east();
        } else if cmd == 'n' {
            println!("northside");
            is_ok &= self.process_north();
        } else if cmd == 's' {
            println!("southside");
            is_ok &= self.process_south();
        } else if cmd == '*' {
            is_ok = is_ok && self.process_flip();
        } else {
            println!("noside");
            self.iterations_ += 1;
            return Err(std::io::Error::new( std::io::ErrorKind::InvalidInput,"wrong character occurred in code sequence"));
        }
        if is_ok {
            return Ok(self.code_.len() > 0 && self.iterations_ > 0);
        }
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"something went wrong"))
    }

    pub fn print_internal_state(&self) {
        println!(
            "cur x is {}\n\
            cur y is {}\n\
            code is {}\n\
            height is {}\n\
            width is {}\n\
            iterations are {}\n",
            self.cur_x_,
            self.cur_y_,
            self.code_,
            self.height_,
            self.width_,
            self.iterations_
        )
    }

    pub fn get_state(&self) -> String {
        let mut result = String::new();
        for i in 0..self.height_ {
            for j in 0..self.width_ {
                if self.one_bits_.contains(&(j, i)) {
                    result += "1 ";
                } else {
                    result += "0 ";
                }
            }
            result += "\n";
        }
        result
    }

    pub fn print_state(&self) {
        println!("{}", self.get_state())
    }

}

#[cfg(test)]
pub fn find_match_wrapper(
    text: &String,
    to_match_char: char,
    match_char: char,
    start_index: i64,
    step: i64) -> Result<i64, std::io::Error> {

    find_match(text, to_match_char, match_char, start_index, step)
}
