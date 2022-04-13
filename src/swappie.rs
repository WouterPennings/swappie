use crate::swappie;
use std::collections::HashMap;

pub struct Swappie {
    counterparts: HashMap<char, char>,
}

impl Swappie {
    pub fn new() -> Swappie {
        Swappie {
            counterparts: swappie::generate_counterparts(),
        }
    }

    pub fn mirror_text(&self, input: String, allow_mangle: bool) -> Result<(String, bool), String> {
        let mut mirrored_text = String::from("");
        let mut mangle = false;
        for c in input.chars().rev() {
            match self.get_counterpart(c) {
                Ok(c) => mirrored_text.push(c),
                Err(s) => {
                    if !allow_mangle {
                        return Err(s);
                    }
                    mirrored_text.push(c);
                    mangle = true;
                }
            }
        }

        Ok((mirrored_text, mangle))
    }

    fn get_counterpart(&self, c: char) -> Result<char, String> {
        return match self.counterparts.get(&c) {
            Some(c) => Ok(c.clone()),
            None => Err(format!("No counterpart for '{}'", c)),
        };
    }
}

fn generate_counterparts() -> HashMap<char, char> {
    let mut cp: HashMap<char, char> = HashMap::new();
    // Characters lowercase
    cp.insert('a', 'ɐ');
    cp.insert('b', 'q');
    cp.insert('c', 'ɔ');
    cp.insert('d', 'p');
    cp.insert('e', 'ǝ');
    cp.insert('f', 'ɟ');
    cp.insert('g', 'ᵷ');
    cp.insert('h', 'ɥ');
    cp.insert('i', 'ᴉ');
    cp.insert('j', 'ɾ');
    cp.insert('k', 'ʞ');
    cp.insert('l', 'ꞁ');
    cp.insert('m', 'ɯ');
    cp.insert('n', 'u');
    cp.insert('o', 'o');
    cp.insert('p', 'd');
    cp.insert('q', 'b');
    cp.insert('r', 'ɹ');
    cp.insert('s', 's');
    cp.insert('t', 'ʇ');
    cp.insert('u', 'n');
    cp.insert('v', 'ʌ');
    cp.insert('w', 'ʍ');
    cp.insert('x', 'x');
    cp.insert('y', 'ʎ');
    cp.insert('z', 'z');
    // Characters uppercase
    cp.insert('A', 'Ɐ');
    cp.insert('B', 'ꓭ');
    cp.insert('C', 'Ɔ');
    cp.insert('D', 'ꓷ');
    cp.insert('E', 'Ǝ');
    cp.insert('F', 'ꓞ');
    cp.insert('G', 'ꓨ');
    cp.insert('H', 'H');
    cp.insert('I', 'I');
    cp.insert('J', 'ꓩ');
    cp.insert('K', 'ʞ');
    cp.insert('L', 'ꓶ');
    cp.insert('M', 'ꟽ');
    cp.insert('N', 'ꓠ');
    cp.insert('O', 'ꓳ');
    cp.insert('P', 'Ԁ');
    cp.insert('Q', 'Ό');
    cp.insert('R', 'ꓤ');
    cp.insert('S', 'ꓢ');
    cp.insert('T', 'ꓕ');
    cp.insert('U', 'ꓵ');
    cp.insert('V', 'ꓥ');
    cp.insert('W', 'M');
    cp.insert('X', 'X');
    cp.insert('Y', '⅄');
    cp.insert('Z', 'Z');
    // Numbers
    cp.insert('0', '0');
    cp.insert('1', 'Ɩ');
    cp.insert('2', 'ᘔ');
    cp.insert('3', 'Ɛ');
    cp.insert('4', '߈');
    cp.insert('5', 'ϛ');
    cp.insert('6', '9');
    cp.insert('7', 'ㄥ');
    cp.insert('8', '8');
    cp.insert('9', '6');
    // Special characters
    cp.insert(' ', ' ');
    cp.insert('&', '⅋');
    cp.insert('.', '˙');
    cp.insert(',', '\'');
    cp.insert('[', ']');
    cp.insert(']', '[');
    cp.insert('(', ')');
    cp.insert(')', '(');
    cp.insert('{', '}');
    cp.insert('}', '{');
    cp.insert('?', '¿');
    cp.insert('!', '¡');
    cp.insert('\'', ',');
    cp.insert('"', '„');
    cp.insert('<', '>');
    cp.insert('>', '<');
    cp.insert('_', '‾');
    cp.insert('"', '„');
    cp.insert('\\', '/');
    cp.insert(';', '؛');
    cp.insert('`', ',');
    cp.insert('‿', '⁀');
    cp.insert('⁅', '⁆');
    cp.insert('∴', '∵');

    cp
}
