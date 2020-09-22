pub fn reply(message: &str) -> &str {
    match message {
        x if { message.replace(" ", "").trim().is_empty() } => "Fine. Be that way!",
        x if { message.chars().any(|x| x.is_alphabetic()) && message.to_uppercase() == message.to_string() && message.trim().ends_with("?") } => "Calm down, I know what I'm doing!",
        x if {  message.chars().any(|x| x.is_alphabetic()) && message.to_uppercase() == message.to_string() } => "Whoa, chill out!",
        x if { message.trim().ends_with("?") } => "Sure.",
        _ => "Whatever."
    }
}
