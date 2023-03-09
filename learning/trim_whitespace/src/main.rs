fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_whitespace(test1), "We need more space.");

    let test2 = "    There's space in front.";
    assert_eq!(trim_whitespace(test2), "There's space in front.");

    let test3 = "There's space in the back.     ";
    assert_eq!(trim_whitespace(test3), "There's space in the back.");

    let test4 = "   Space is surrounding us.     ";
    assert_eq!(trim_whitespace(test4), "Space is surrounding us.");

    let test5 = "        ";
    assert_eq!(trim_whitespace(test5), "");

    let test6 = "";
    assert_eq!(trim_whitespace(test6), "");

    let test7 = "  🔥  ";
    assert_eq!(trim_whitespace(test7), "🔥");
}

fn trim_whitespace(s: &str) -> &str{
    let mut start = 0;
    let mut end = s.len();
    if end == 0 { return "" }
    for (index, b) in s.bytes().enumerate(){
        if b != b' '{
            start = index;
            break;
        }
        if index == end-1 {
            return ""
        }
    }
    for (index, b) in s.bytes().rev().enumerate(){
        if b != b' '{
            end -= index;
            break;
        }
    }
    &s[start..end] 
}
