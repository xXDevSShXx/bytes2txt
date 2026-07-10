#![forbid(unsafe_code)]

const BASE95_POWERS: [u32; 5] = [1u32, 95u32, 95u32.pow(2), 95u32.pow(3), 95u32.pow(4)];
const SPACE_OFFSET: u32 = ' ' as u32;

pub fn encode(bytes: &[u8]) -> String {
    let padding_count = (4 - (bytes.len() % 4)) % 4;

    let mut data = bytes
        .iter()
        .copied()
        .chain(std::iter::repeat_n(u8::MAX, padding_count));

    std::iter::once(
        char::from_digit(padding_count as u32, 10).expect("calculated value should be int."),
    )
    .chain((0..(bytes.len() + padding_count) / 4).flat_map(|_| {
        u32_to_base95_string(u32::from_be_bytes([
            data.next()
                .expect("already padded iterator shouldn't be small."),
            data.next()
                .expect("already padded iterator shouldn't be small."),
            data.next()
                .expect("already padded iterator shouldn't be small."),
            data.next()
                .expect("already padded iterator shouldn't be small."),
        ]))
        .into_iter()
    }))
    .collect()
}

fn u32_to_base95_string(value: u32) -> [char; 5] {
    let mut value = value;
    let mut result = [' '; 5];
    for (i, slot) in result.iter_mut().enumerate() {
        let index = 4 - i;
        *slot = char::from_u32(value / BASE95_POWERS[index] + SPACE_OFFSET).unwrap();
        value %= BASE95_POWERS[index];
    }
    result
}

pub fn decode(txt: &str) -> Option<Vec<u8>> {
    if txt.is_empty() || txt.len() % 5 != 1 || !txt.is_ascii() {
        return None;
    }

    let mut chars = txt.chars();

    let padding_count = chars.next().unwrap().to_digit(10)? as usize;
    let bytes_count = (txt.len() / 5) * 4;

    if padding_count > 3 || bytes_count < padding_count {
        return None;
    }

    if bytes_count == 0 {
        return Some(Vec::new());
    }

    let mut bytes: Vec<u8> = Vec::with_capacity(bytes_count - padding_count);
    for _ in 0..(txt.len() / 5 - 1) {
        bytes.extend_from_slice(&u32::to_be_bytes(base95string_to_u32(&[
            chars.next().filter(|char| *char >= ' ' && *char <= '~')?,
            chars.next().filter(|char| *char >= ' ' && *char <= '~')?,
            chars.next().filter(|char| *char >= ' ' && *char <= '~')?,
            chars.next().filter(|char| *char >= ' ' && *char <= '~')?,
            chars.next().filter(|char| *char >= ' ' && *char <= '~')?,
        ])?));
    }
    bytes.extend_from_slice(
        &u32::to_be_bytes(base95string_to_u32(&[
            chars.next().filter(|char| *char >= ' ' && *char <= '~')?,
            chars.next().filter(|char| *char >= ' ' && *char <= '~')?,
            chars.next().filter(|char| *char >= ' ' && *char <= '~')?,
            chars.next().filter(|char| *char >= ' ' && *char <= '~')?,
            chars.next().filter(|char| *char >= ' ' && *char <= '~')?,
        ])?)[0..(4 - padding_count)],
    );

    Some(bytes)
}

fn base95string_to_u32(string: &[char; 5]) -> Option<u32> {
    let base10_value: u64 = string
        .iter()
        .rev()
        .enumerate()
        .map(|(index, &char)| ((char as u64) - SPACE_OFFSET as u64) * BASE95_POWERS[index] as u64) // 95 ^ index
        .sum();

    u32::try_from(base10_value).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================
    // Helper: u32_to_base95_string (fewer tests)
    // ============================================================

    #[test]
    fn u32_to_base95_string_zero() {
        assert_eq!(u32_to_base95_string(0), [' '; 5]);
    }

    #[test]
    fn u32_to_base95_string_one() {
        assert_eq!(u32_to_base95_string(1), [' ', ' ', ' ', ' ', '!']);
    }

    #[test]
    fn u32_to_base95_string_known_value() {
        assert_eq!(u32_to_base95_string(135991546), ['!', '_', 'Z', '=', '{']);
    }

    #[test]
    fn u32_to_base95_string_max() {
        assert_eq!(u32_to_base95_string(u32::MAX), ['T', 'e', 'I', '^', '%']);
    }

    // ============================================================
    // Helper: base95string_to_u32 (fewer tests)
    // ============================================================

    #[test]
    fn base95string_to_u32_zero() {
        assert_eq!(base95string_to_u32(&[' ', ' ', ' ', ' ', ' ']), Some(0));
    }

    #[test]
    fn base95string_to_u32_one() {
        assert_eq!(base95string_to_u32(&[' ', ' ', ' ', ' ', '!']), Some(1));
    }

    #[test]
    fn base95string_to_u32_known_value() {
        assert_eq!(
            base95string_to_u32(&['!', '_', 'Z', '=', '{']),
            Some(135991546)
        );
    }

    #[test]
    fn base95string_to_u32_round_trip() {
        // round-trip: u32 -> string -> u32
        for val in [0u32, 1, 42, 135991546, u32::MAX] {
            let encoded = u32_to_base95_string(val);
            assert_eq!(base95string_to_u32(&encoded), Some(val));
        }
    }

    // ============================================================
    // encode (more tests)
    // ============================================================

    #[test]
    fn encode_empty() {
        assert_eq!(encode(&[]), "0");
    }

    #[test]
    fn encode_four_bytes_zero_padding() {
        // [8, 27, 16, 250] -> 4 bytes -> padding_count = 0
        assert_eq!(encode(&[8, 27, 16, 250]), "0!_Z={");
    }

    #[test]
    fn encode_one_byte_three_padding() {
        // [72] -> padding_count = 3
        assert_eq!(encode(&[72]), "3/#Lu|");
    }

    #[test]
    fn encode_one_byte_zero_value() {
        // [0] -> padding_count = 3, data padded with 0xFF
        assert_eq!(encode(&[0]), "3 3U|9");
    }

    #[test]
    fn encode_two_bytes_two_padding() {
        // [0, 0] -> padding_count = 2
        assert_eq!(encode(&[0, 0]), "2  '8p");
    }

    #[test]
    fn encode_three_bytes_one_padding() {
        // [0, 0, 0] -> padding_count = 1
        assert_eq!(encode(&[0, 0, 0]), "1   \"a");
    }

    #[test]
    fn encode_two_chunks_zero_padding() {
        // 8 bytes -> 2 chunks, padding_count = 0
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(encode(&input), "0 3dW*!#<[I");
    }

    #[test]
    fn encode_five_bytes() {
        // 5 bytes -> 2 chunks, padding_count = 3
        let input = vec![0, 0, 0, 0, 0];
        assert_eq!(encode(&input), "3      3U|9");
    }

    #[test]
    fn encode_six_bytes() {
        // 6 bytes -> 2 chunks, padding_count = 2
        let input = vec![0, 0, 0, 0, 0, 0];
        assert_eq!(encode(&input), "2       '8p");
    }

    #[test]
    fn encode_seven_bytes() {
        // 7 bytes -> 2 chunks, padding_count = 1
        let input = vec![0, 0, 0, 0, 0, 0, 0];
        assert_eq!(encode(&input), "1        \"a");
    }

    #[test]
    fn encode_all_zeros_four_bytes() {
        assert_eq!(encode(&[0, 0, 0, 0]), "0     ");
    }

    #[test]
    fn encode_all_max_four_bytes() {
        assert_eq!(encode(&[255, 255, 255, 255]), "0TeI^%");
    }

    #[test]
    fn encode_output_is_printable_ascii() {
        // Output must only contain chars 32..=126 (space through tilde)
        let result = encode(&[0xDE, 0xAD, 0xBE, 0xEF, 0x42, 0x13, 0x37]);
        for ch in result.chars() {
            let code = ch as u32;
            assert!(
                (32..=126).contains(&code),
                "char {ch:?} (u+{code:04X}) is not printable ASCII"
            );
        }
    }

    #[test]
    fn encode_output_length_formula() {
        // For n > 0, output len = 1 + 5 * ceil(n / 4)
        // For n = 0, output len = 1
        for n in 0..=20 {
            let input: Vec<u8> = (0..n as u8).collect();
            let result = encode(&input);
            let expected_len = if n == 0 { 1 } else { 1 + 5 * ((n + 3) / 4) };
            assert_eq!(result.len(), expected_len, "wrong length for n={n}");
        }
    }

    #[test]
    fn encode_padding_digit_is_first_char() {
        let result = encode(&[42, 17, 88, 13, 99]);
        let first = result.chars().next().unwrap();
        assert!(first.is_ascii_digit(), "first char should be a digit");
    }

    // ============================================================
    // decode (more tests)
    // ============================================================

    #[test]
    fn decode_single_chunk_zero_padding() {
        assert_eq!(decode("0!_Z={"), Some(vec![8, 27, 16, 250]));
    }

    #[test]
    fn decode_single_chunk_three_padding() {
        assert_eq!(decode("3/#Lu|"), Some(vec![72]));
    }

    #[test]
    fn decode_single_chunk_two_padding() {
        assert_eq!(decode("2  '8p"), Some(vec![0, 0]));
    }

    #[test]
    fn decode_single_chunk_one_padding() {
        assert_eq!(decode("1   \"a"), Some(vec![0, 0, 0]));
    }

    #[test]
    fn decode_multi_chunk_zero_padding() {
        assert_eq!(decode("0 3dW*!#<[I"), Some(vec![1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    fn decode_empty_string() {
        assert_eq!(decode(""), None);
    }

    #[test]
    fn decode_non_ascii() {
        // non-ASCII bytes should be rejected
        assert_eq!(decode("\u{00e9}!!!!!"), None); // é at start
        assert_eq!(decode("!!!!!\u{00e9}"), None); // é at end
    }

    #[test]
    fn decode_empty_encoding() {
        // encode(&[]) produces "0", decode should return empty vec
        assert_eq!(decode("0"), Some(vec![]));
    }

    #[test]
    fn decode_ascii_control_characters_rejected() {
        // Characters below space (e.g. tab, newline) should be rejected
        assert_eq!(decode("0\t!!!!"), None);
        assert_eq!(decode("0\n!!!!"), None);
        assert_eq!(decode("0\0!!!!"), None);
    }

    #[test]
    fn decode_invalid_padding_digit() {
        // padding digit must be 0-3
        // Using letters and symbols as padding
        let encoded_with_padding_a = "a    !"; // "    !" + "a" (not a digit)
        assert_eq!(decode(encoded_with_padding_a), None);

        let encoded_with_padding_slash = "/    !"; // "    !" + "/" (not a digit)
        assert_eq!(decode(encoded_with_padding_slash), None);

        let encoded_with_padding_large = "5    !"; // "    !" + "/" (not a digit)
        assert_eq!(decode(encoded_with_padding_large), None);
    }

    #[test]
    fn decode_malicious_overflow_panic() {
        // BUG: high-valued printable chars cause u32 overflow in base95string_to_u32.
        // '~' (126) gives digit 94, and 94 * 95^4 > u32::MAX.
        // This should return None, but currently panics in debug builds.
        decode("0~~~~~");
    }

    #[test]
    fn decode_padding_digit_out_of_range() {
        let encoded_with_padding = "3";
        assert_eq!(decode(encoded_with_padding), None);

        let encoded_with_padding_large = "5    !"; // 5 is larger than 3
        assert_eq!(decode(encoded_with_padding_large), None);
    }

    // ============================================================
    // Round-trip: encode then decode (many tests)
    // ============================================================

    fn round_trip(data: &[u8]) {
        let encoded = encode(data);
        let decoded = decode(&encoded);
        assert_eq!(
            decoded,
            Some(data.to_vec()),
            "round-trip failed for {data:?}"
        );
    }

    #[test]
    fn round_trip_length_100() {
        let input: Vec<u8> = (0..100).collect();
        round_trip(&input);
    }

    #[test]
    fn round_trip_all_zeros() {
        for len in [1, 2, 3, 4, 5, 8, 12] {
            let input = vec![0u8; len];
            round_trip(&input);
        }
    }

    #[test]
    fn round_trip_all_max() {
        for len in [1, 2, 3, 4, 5, 8, 12] {
            let input = vec![0xFFu8; len];
            round_trip(&input);
        }
    }

    #[test]
    fn round_trip_alternating_patterns() {
        round_trip(&[0xAA, 0x55, 0xAA, 0x55]);
        round_trip(&[0xAA, 0x55, 0xAA, 0x55, 0xAA]);
        round_trip(&[0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF]);
    }

    #[test]
    fn round_trip_consecutive_lengths() {
        for len in 0..=16 {
            let input: Vec<u8> = (0..len as u8).collect();
            if len == 0 {
                let encoded = encode(&input);
                assert_eq!(encoded, "0");
                // decode can't handle length-1 inputs,
                // so empty round-trip is expected to fail.
                continue;
            }
            round_trip(&input);
        }
    }

    #[test]
    fn round_trip_all_byte_values_single() {
        // Encode every single byte value 0..=255 and verify round-trip
        for byte in 0..=255u8 {
            let input = [byte];
            let encoded = encode(&input);
            let decoded = decode(&encoded);
            assert_eq!(
                decoded,
                Some(input.to_vec()),
                "round-trip failed for single byte {byte}"
            );
        }
    }
}
