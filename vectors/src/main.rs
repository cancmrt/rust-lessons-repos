fn vectors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}",a);

    a.push(44);

    println!("a = {:?}",a);

    let idx:usize = 2;

    println!("a[idx] = {}",a[idx]);

    a[idx] = 22;

    println!("a[idx] = {}",a[idx]);

    //a.get(6); // this is return Option type
    match a.get(6)
    {
        Some(x) =>println!("a[6] = {}",x),
        None => println!("error, o such elements")
    }

    for x in &a {
        println!("{}",x);
    }

    let last_elem = a.pop();

    match last_elem{
        Some(x) => println!("Pop element{}",x),
        None => println!("ELement is")
    };

    while let Some(x) = a.pop()
    {
        println!("{}",x);
    }


}


fn main() {
    vectors();
}
