const REFERENCE_SET: [&str; 95] = [
    " ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?", "@", "A", "B", "C", "D", "E",
    "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X",
    "Y", "Z", "[", "\\", "]", "^", "_", "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k",
    "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~",
];

pub fn encode(buf: &[u8]) -> String {
    buf.iter().map(get_string).collect::<Vec<_>>().join("")
}

fn get_string(byte: &u8) -> String {
    let byte = *byte as usize;
    format!("{}{}", REFERENCE_SET[byte / 95], REFERENCE_SET[byte % 95])
}

pub fn decode(txt: String) -> Option<Vec<u8>> {
    if (txt.len() % 2) != 0 {
        return None;
    }

    todo!()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = 4;
//         assert_eq!(result, 4);
//     }
// }
