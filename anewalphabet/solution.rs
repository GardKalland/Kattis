use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut translation_map: HashMap<char, &'static str> = HashMap::new();
    translation_map.insert('a', "@");
    translation_map.insert('b', "8");
    translation_map.insert('c', "(");
    translation_map.insert('d', "|)");
    translation_map.insert('e', "3");
    translation_map.insert('f', "#");
    translation_map.insert('g', "6");
    translation_map.insert('h', "[-]");
    translation_map.insert('i', "|");
    translation_map.insert('j', "_|");
    translation_map.insert('k', "|<");
    translation_map.insert('l', "1");
    translation_map.insert('m', "[]\\/[]");
    translation_map.insert('n', "[]\\[]");
    translation_map.insert('o', "0");
    translation_map.insert('p', "|D");
    translation_map.insert('q', "(,)");
    translation_map.insert('r', "|Z");
    translation_map.insert('s', "$");
    translation_map.insert('t', "']['");
    translation_map.insert('u', "|_|");
    translation_map.insert('v', "\\/");
    translation_map.insert('w', "\\/\\/");
    translation_map.insert('x', "}{");
    translation_map.insert('y', "`/");
    translation_map.insert('z', "2");

    let mut input = String::new(); 
    io::stdin().read_to_string(&mut input).unwrap();

    let mut output = String::new();
    for ch in input.chars() {
        let lower_ch = ch.to_lowercase().next().unwrap();
        match translation_map.get(&lower_ch) {
            Some(value) => output.push_str(value),
            None => output.push(ch),
        }
    }

    println!("{}", output);
}
 
