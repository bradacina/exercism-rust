pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    match trimmed {
        _ if is_silence(trimmed) => "Fine. Be that way!",
        _ if is_shout(trimmed) && is_question(trimmed) => "Calm down, I know what I'm doing!",
        _ if is_shout(trimmed) => "Whoa, chill out!",
        _ if is_question(trimmed) => "Sure.",
        _ => "Whatever.",
    }
}

fn is_shout(message: &str) -> bool {
    let has_lowercase = message.chars().any(|c| c.is_lowercase());

    let has_alphabetic = message.chars().any(|c| c.is_alphabetic());

    !has_lowercase && has_alphabetic
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_silence(message: &str) -> bool {
    message.is_empty()
}
