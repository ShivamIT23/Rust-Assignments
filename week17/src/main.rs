fn main() {
    let longest;
    let str1 = String::from("shivam");
    {
        let str2 = String::from("gupta");
        longest = longest_str(&str1,&str2);
        println!("{}",longest);
    }
    // println!("{}",longest);
}

fn longest_str<'b> ( first : &'b String, second : &'b String) -> &'b String {
    if first.len() > second.len() {
        return first;
    }
    second
}