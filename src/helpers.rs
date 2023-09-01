pub fn add_linebreaks(string: &str, length: usize) -> String {
    let length = if length == 0 { 1 } else { length };
    if length >= string.len() {
        return string.into();
    }

    let mut last_whitespace = 0;
    let mut data: String = String::new();

    for i in 0..string.trim().len() {
        if string.chars().nth(i).unwrap_or_default().is_whitespace() {
            last_whitespace = i;
        }

        data.push(string.chars().nth(i).unwrap_or_default());
        if (i % length) == 0 && i != 0 {
            // println!("data len = {}", data.len());
            data.replace_range(last_whitespace..last_whitespace + 1, "\n");
        }
    }
    data
}

#[test]
fn test_add_linebreak() {
    let input = "123456789 123456789";
    let length = 12;
    let expected = "123456789\n123456789";
    let actual = add_linebreaks(input, length);

    assert_eq!(expected, actual);
}
