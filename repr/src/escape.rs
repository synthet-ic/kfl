pub fn escape(c: char) -> char {
    match c {
        'b' => '\u{0008}',  // Backspace
        'f' => '\u{000C}',  // Form feed
        'n' => '\n',  // New line
        'r' => '\r',  // Carriage return
        't' => '\t',  // Tab
        'v' => '\u{000B}',  // Vertical tab
        '0' => '\0',  // Null character
        _ => panic!()
    }
}
