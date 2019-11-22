use std::io;

fn main() {
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence)
        .expect("Failed to read line!");
    for c in sentence.to_lowercase().chars(){
        if c == '\n' || c == '\r'{
            break;
        }
        if c.is_alphabetic() == true{
            print!(":regional_indicator_{}: ",c);
        }else{
            match c{
                ' ' => print!(":boom: "),
                '0' => print!(":zero: "),
                '1' => print!(":one: "),
                '2' => print!(":two: "),
                '3' => print!(":three: "),
                '4' => print!(":four: "),
                '5' => print!(":five: "),
                '6' => print!(":six: "),
                '7' => print!(":seven: "),
                '8' => print!(":eight: "),
                '9' => print!(":nine: "),
                '!' => print!(":exclamation: "),
                '?' => print!(":question: "),
                '#' => print!(":hash: "),
                '*' => print!(":asterick: "),
                _ => print!("`{}` ", c),
            }
        }
    }
}
