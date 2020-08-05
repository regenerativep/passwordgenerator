use rand::Rng;

const PASSWORD_LENGTH: u32 = 16;
const ALLOWED_CHARACTERS: [char; 80] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '[', ']', '{', '}', ';', '\'', ':', '"', '\\', '|', ',', '<', '.', '>', '/', '?', '`', '~'
];
const SURROUND_ORDER: [(isize, isize); 8] = [
    (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)
];
struct KeyboardKeys {
    lower: [[Option<char>; 13]; 4], //row major order (for the sake of easily defining keyboards within the code)
    upper: [[Option<char>; 13]; 4],
    midpoint: usize,
}
impl KeyboardKeys {
    fn qwerty() -> KeyboardKeys {
        KeyboardKeys {
            lower: [
                [Some('`'), Some('1'), Some('2'), Some('3'), Some('4'), Some('5'), Some('6'), Some('7'), Some('8'), Some('9'), Some('0'), Some('-'), Some('=')],
                [None, Some('q'), Some('w'), Some('e'), Some('r'), Some('t'), Some('y'), Some('u'), Some('i'), Some('o'), Some('p'), Some('['), Some(']')],
                [None, Some('a'), Some('s'), Some('d'), Some('f'), Some('g'), Some('h'), Some('j'), Some('k'), Some('l'), Some(';'), Some('\''), Some('\\')],
                [None, Some('z'), Some('x'), Some('c'), Some('v'), Some('b'), Some('n'), Some('m'), Some(','), Some('.'), Some('/'), None, None]
            ],
            upper: [
                [Some('~'), Some('!'), Some('@'), Some('#'), Some('$'), Some('%'), Some('^'), Some('&'), Some('*'), Some('('), Some(')'), Some('_'), Some('+')],
                [None, Some('Q'), Some('W'), Some('E'), Some('R'), Some('T'), Some('Y'), Some('U'), Some('I'), Some('O'), Some('P'), Some('{'), Some('}')],
                [None, Some('A'), Some('S'), Some('D'), Some('F'), Some('G'), Some('H'), Some('J'), Some('K'), Some('L'), Some(':'), Some('"'), Some('|')],
                [None, Some('Z'), Some('X'), Some('C'), Some('V'), Some('B'), Some('N'), Some('M'), Some('<'), Some('>'), Some('?'), None, None]
            ],
            midpoint: 6,
        }
    }
}
fn main() {
    let keyboard = KeyboardKeys::qwerty();
    let password = generate_password(&keyboard, PASSWORD_LENGTH);
    println!("{}", password);
}

fn generate_password(keyboard: &KeyboardKeys, pass_len: u32) -> String {
    let mut password = String::new();
    for _ in 0..pass_len {
        let last_char = password.chars().last();
        password.push(get_random_char(keyboard, last_char));
    }
    return password;
}
fn get_chars(chars: [[Option<char>; 13]; 4]) -> Vec<char> {
    let mut found_chars = Vec::new();
    for row in chars.iter() {
        for c in row.iter() {
            match c {
                Some(val) => found_chars.push(*val),
                None => { },
            };
        }
    }
    return found_chars;
}
enum CharSide {
    Left, Right,
}
impl CharSide {
    fn other(&self) -> CharSide {
        match self {
            CharSide::Left => CharSide::Right,
            CharSide::Right => CharSide::Left,
        }
    }
}
fn get_side_chars_from_case(side: &CharSide, chars: [[Option<char>; 13]; 4], midpoint: usize) -> Vec<char> {
    let mut found_chars: Vec<char> = Vec::new();
    for row in chars.iter() {
        for (i, c) in row.iter().enumerate() {
            match c {
                Some(val) => {
                    match side {
                        CharSide::Left => {
                            if i < midpoint as usize {
                                found_chars.push(*val);
                            }
                        },
                        CharSide::Right => {
                            if i >= midpoint as usize {
                                found_chars.push(*val);
                            }
                        }
                    }
                },
                None => { }, 
            }
        }
    }
    return found_chars;
}
fn get_char_side(c: char, keyboard: &KeyboardKeys) -> CharSide {
    let mut left_chars = get_side_chars_from_case(&CharSide::Left, keyboard.upper, keyboard.midpoint);
    left_chars.append(&mut get_side_chars_from_case(&CharSide::Left, keyboard.lower, keyboard.midpoint));
    if left_chars.contains(&c) {
        CharSide::Left
    }
    else {
        CharSide::Right
    }
}
fn get_character_position_from_case(c: char, chars: [[Option<char>; 13]; 4]) -> Option<(isize, isize)> {
    for (j, row) in chars.iter().enumerate() {
        for (i, found_c) in row.iter().enumerate() {
            match found_c {
                Some(val) => {
                    if *val == c {
                        return Some((i as isize, j as isize));
                    }
                }, None => { },
            }
        }
    }
    return None;
}
fn get_character_position(c: char, keyboard: &KeyboardKeys) -> Option<(isize, isize)> {
    match get_character_position_from_case(c, keyboard.upper) {
        Some(val) => return Some(val),
        None => return get_character_position_from_case(c, keyboard.lower)
    }
}
fn get_character_at(pos: (isize, isize), chars: [[Option<char>; 13]; 4]) -> Option<char> {
    match chars.get(pos.1 as usize) {
        Some(row) => match row.get(pos.0 as usize) {
            Some(c_opt) => match c_opt {
                Some(c) => Some(*c),
                None => None,
            },
            None => None,
        },
        None => None,
    }
}
fn get_surrounding_characters(c: char, keyboard: &KeyboardKeys) -> Vec<char> {
    let mut found_chars = Vec::new();
    let pos = get_character_position(c, keyboard);
    match pos {
        Some(val) => {
            for addpos in SURROUND_ORDER.iter() {
                let target_pos = (val.0 + addpos.0, val.1 + addpos.1);
                match get_character_at(target_pos, keyboard.upper) {
                    Some(found_c) => found_chars.push(found_c),
                    None => { },
                }
                match get_character_at(target_pos, keyboard.lower) {
                    Some(found_c) => found_chars.push(found_c),
                    None => { },
                }
            }
        }, None => { },
    }
    return found_chars;
}
fn get_random_char(keyboard: &KeyboardKeys, last_char_opt: Option<char>) -> char {
    //new key must be either on the other side of the keyboard or next to the last key
    let found_char: char = match last_char_opt {
        None => {
            //get a random key
            let mut all_chars = get_chars(keyboard.lower);
            all_chars.append(&mut get_chars(keyboard.upper));
            all_chars[rand::thread_rng().gen_range(0, all_chars.len())]
        },
        Some(last_char) => {
            let other_side = get_char_side(last_char, keyboard).other();
            let mut possible_chars = get_side_chars_from_case(&other_side, keyboard.upper, keyboard.midpoint);
            possible_chars.append(&mut get_side_chars_from_case(&other_side, keyboard.lower, keyboard.midpoint));
            possible_chars.append(&mut get_surrounding_characters(last_char, keyboard));
            possible_chars[rand::thread_rng().gen_range(0, possible_chars.len())]
        },
    };
    if ALLOWED_CHARACTERS.contains(&found_char) {
        return found_char;
    }
    else {
        return get_random_char(keyboard, last_char_opt);
    }
}