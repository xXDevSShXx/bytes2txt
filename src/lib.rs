pub fn encode(buf: &[u8]) -> String {
    let mut buf = buf.to_owned();
    let mut padding_bytes = 0u8;
    if !buf.len().is_multiple_of(4) {
        padding_bytes = 4 - ((buf.len() % 4) as u8);
        let mut zeros = std::iter::repeat_n(0u8, padding_bytes as usize).collect();
        buf.append(&mut zeros);
    }

    let mut parts: Vec<String> = buf
        .chunks_exact(4)
        .map(|bytes| u32_to_base95string(four_bytes_to_u32(bytes)))
        .collect();

    parts.push(padding_bytes.to_string());

    parts.concat()
}

fn four_bytes_to_u32(bytes: &[u8]) -> u32 {
    if bytes.len() != 4 {
        panic!("incorrect number of bytes provided.");
    }
    let byte1 = (bytes[0] as u32) << 24;
    let byte2 = (bytes[1] as u32) << 16;
    let byte3 = (bytes[2] as u32) << 8;
    let byte4 = bytes[3] as u32;
    byte1 + byte2 + byte3 + byte4
}

fn u32_to_base95string(value: u32) -> String {
    let mut value = value;

    let first_digit = char::from_u32(value / 81450625 + 32)
        .expect("this shouldn't fail because: u32::MAX is Smaller than 95^5"); // 95 ^ 4
    value %= 81450625;

    let second_digit = char::from_u32(value / 857375 + 32)
        .expect("this shouldn't fail because: u32::MAX is Smaller than 95^5"); // 95 ^ 3
    value %= 857375;

    let third_digit = char::from_u32(value / 9025 + 32)
        .expect("this shouldn't fail because: u32::MAX is Smaller than 95^5"); // 95 ^ 2
    value %= 9025;

    let fourth_digit = char::from_u32(value / 95 + 32)
        .expect("this shouldn't fail because: u32::MAX is Smaller than 95^5"); // 95 ^ 1

    let fifth_digit = char::from_u32(value % 95 + 32)
        .expect("this shouldn't fail because: u32::MAX is Smaller than 95^5"); // 95 ^ 0

    format!(
        "{}{}{}{}{}",
        first_digit, second_digit, third_digit, fourth_digit, fifth_digit
    )
}

pub fn decode(txt: String) -> Option<Vec<u8>> {
    if !txt.len().is_multiple_of(5) || txt.len() == 5 || !txt.is_ascii() {
        return None;
    }

    let chars = txt.chars().collect::<Vec<_>>();

    let (padding_bytes, digits) = chars
        .split_last()
        .expect("already checked size shouldn't be 0.");

    let padding_bytes = padding_bytes.to_digit(10)? as usize;

    let bytes = digits
        .chunks_exact(5)
        .flat_map(|chunk| u32_to_four_bytes(base95string_to_u32(chunk)))
        .take(txt.len() / 5 * 4 - padding_bytes)
        .collect();
    Some(bytes)
}

fn base95string_to_u32(string: &[char]) -> u32 {
    let mut string = string.iter().map(|&char| (char as u32) - 32);
    string.next().expect("size checked at compile time shouldn't fail") * 81450625 // 95 ^ 4
    + string.next().expect("size checked at compile time shouldn't fail") * 857375 // 95 ^ 3
    + string.next().expect("size checked at compile time shouldn't fail") * 9025   // 95 ^ 2
    + string.next().expect("size checked at compile time shouldn't fail") * 95     // 95 ^ 1
    + string.next().expect("size checked at compile time shouldn't fail")
}

fn u32_to_four_bytes(value: u32) -> [u8; 4] {
    let byte4 = (value << 24) >> 24;
    let byte3 = (value << 16) >> 24;
    let byte2 = (value << 8) >> 24;
    let byte1 = value >> 24;
    [byte1 as u8, byte2 as u8, byte3 as u8, byte4 as u8]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_bytes_to_u32_test() {
        let bytes: [u8; 4] = [8, 27, 16, 250];
        assert_eq!(four_bytes_to_u32(&bytes), 135991546);
    }

    #[test]
    fn four_bytes_to_u32_zero_test() {
        let bytes: [u8; 4] = [0, 0, 0, 0];
        assert_eq!(four_bytes_to_u32(&bytes), 0);
    }

    #[test]
    fn four_bytes_to_u32_max_test() {
        let bytes: [u8; 4] = [255, 255, 255, 255];
        assert_eq!(four_bytes_to_u32(&bytes), 0xFFFFFFFF);
    }

    #[test]
    fn u32_to_base95string_zero_test() {
        // Value 0 should produce all spaces (REFERENCE_SET[0] = "     ")
        assert_eq!(u32_to_base95string(0), "     ");
    }

    #[test]
    fn u32_to_base95string_one_test() {
        // Value 1 should produce "    !"
        assert_eq!(u32_to_base95string(1), "    !");
    }

    #[test]
    fn u32_to_base95string_test_value() {
        // Value 135991546 should produce "!_Z={"
        assert_eq!(u32_to_base95string(135991546), "!_Z={");
    }

    #[test]
    fn encode_four_bytes_test() {
        // [8, 27, 16, 250] -> 135991546 -> "!_Z={" + padding 0
        assert_eq!(encode(&[8, 27, 16, 250]), "!_Z={0");
    }

    #[test]
    fn encode_empty_test() {
        // Empty input produces padding byte "0" (no chunks, 0 extra padding needed)
        assert_eq!(encode(&[]), "0");
    }

    #[test]
    fn encode_less_than_four_bytes_test() {
        // 1 byte [72] -> [72, 0, 0, 0] -> 1207959552 (big-endian) -> ".nuxc" + padding 3
        assert_eq!(encode(&[72]), ".nuxc3");
    }

    #[test]
    fn encode_pads_with_zeros_test() {
        // 2 bytes [0, 0] -> [0, 0, 0, 0] -> 0 -> "     " + padding 2
        assert_eq!(encode(&[0, 0]), "     2");
        // 3 bytes [0, 0, 0] -> [0, 0, 0, 0] -> 0 -> "     " + padding 1
        assert_eq!(encode(&[0, 0, 0]), "     1");
    }

    #[test]
    fn encode_multiple_chunks_test() {
        // 8 bytes -> two chunks of 4
        // [1, 2, 3, 4] -> 16909060 -> " 3dW*"
        // [5, 6, 7, 8] -> 84281096 -> "!#<[I"
        // + padding 0
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(encode(&bytes), " 3dW*!#<[I0");
    }

    #[test]
    fn base95string_to_u32_zero_test() {
        // "     " (5 spaces) should produce 0
        // Space = ASCII 32, so char - 32 = 0 for all
        assert_eq!(base95string_to_u32(&[' ', ' ', ' ', ' ', ' ']), 0);
    }

    #[test]
    fn base95string_to_u32_one_test() {
        // "    !" (4 spaces + !) should produce 1
        // ! = ASCII 33, so char - 32 = 1
        assert_eq!(base95string_to_u32(&[' ', ' ', ' ', ' ', '!']), 1);
    }

    #[test]
    fn base95string_to_u32_test_value() {
        // "!_Z={" should produce 135991546
        // ! = 1, _ = 63, Z = 58, = = 29, { = 91
        assert_eq!(base95string_to_u32(&['!', '_', 'Z', '=', '{']), 135991546);
    }

    #[test]
    fn u32_to_four_bytes_test() {
        // Value 135991546 should produce [8, 27, 16, 250]
        assert_eq!(u32_to_four_bytes(135991546), [8, 27, 16, 250]);
    }

    #[test]
    fn u32_to_four_bytes_zero_test() {
        // Value 0 should produce [0, 0, 0, 0]
        assert_eq!(u32_to_four_bytes(0), [0, 0, 0, 0]);
    }
}
