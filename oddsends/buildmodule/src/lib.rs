pub mod greetings{

    pub mod english;
    pub mod turkish;
}

#[test]
fn english_greeting_correct()
{
    assert_eq!("hello",greetings::english::hello());
}
#[test]
#[should_panic]
fn english_greeting_false()
{
    assert_eq!("hellofailed",greetings::english::hello());
}