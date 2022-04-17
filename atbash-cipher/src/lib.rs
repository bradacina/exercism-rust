use std::iter;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = plain
        .to_ascii_lowercase()
        .as_str()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(atbash_map)
        .enumerate()
        .flat_map(|(i, v)| {
            if (i + 1) % 5 == 0 {
                iter::once(v).chain(Some(' '))
            } else {
                iter::once(v).chain(None)
            }
        })
        .collect::<String>();

    result.truncate(result.trim_end().len());
    result
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(atbash_map)
        .collect()
}

fn atbash_map(c: char) -> char {
    if c.is_ascii_digit() {
        c
    } else {
        ((b'z' - (c as u8) + b'a') as char)
    }
}
