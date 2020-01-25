fn strings()
{
    //compailer time build
    //str
    let s:&'static str = "hello there !"; //&str string slice //utf-8 characters
    for c in s.chars(){
        println!("{}",c)
    }
    for c in s.chars().rev(){
        println!("{}",c)
    }

    if let Some(first_char) = s.chars().nth(0){
        println!("first character is {}",first_char);
    }

    //on heap
    //String

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}",letters);

    //&str <> String
    let u:&str = &letters;
    println!("{}",u);
    //concatenation
    //String + str
    let z = letters + "abc";
    println!("{}",z);


    let abc = String::from("Hello World");
    println!("{}",abc);

    let mut abk = "hello world".to_string();
    abk.remove(0);
    abk.push_str("!!!");
    println!("{}",abk.replace("ello","goodbye"));


}

fn main() {
    strings();
}
