pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .flat_map(split_camelcase)
        .map(|t| t.to_lowercase())
        .collect()
}

// addEventListener -> add, Event, Listener
// HTMLElement -> HTML, Element
fn split_camelcase(token: &str) -> Vec<&str> {
    let chars: Vec<(usize, char)> = token.char_indices().collect();
    let mut parts = Vec::new();
    let mut start = 0;

    for i in 1..chars.len() {
        let (idx, c) = chars[i];
        let prev = chars[i - 1].1;
        let next_lower = chars.get(i + 1).is_some_and(|(_, n)| n.is_lowercase());

        let boundary = (prev.is_lowercase() && c.is_uppercase())
            || (prev.is_uppercase() && c.is_uppercase() && next_lower);

        if boundary {
            parts.push(&token[start..idx]);
            start = idx;
        }
    }

    parts.push(&token[start..]);
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camelcase() {
        assert_eq!(
            split_camelcase("addEventListener"),
            ["add", "Event", "Listener"]
        );
        assert_eq!(split_camelcase("HTMLElement"), ["HTML", "Element"]);
        assert_eq!(split_camelcase("HTML"), ["HTML"]);
        assert_eq!(split_camelcase("the"), ["the"]);
    }

    #[test]
    fn tokenizer() {
        assert_eq!(
            tokenize("The flex-direction of innerHTML"),
            ["the", "flex", "direction", "of", "inner", "html"]
        );
    }
}
