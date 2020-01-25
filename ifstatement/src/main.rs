fn if_statemenet(){
    let temp = 15;

    if temp > 30
    {
        println!("Really Hot Outside");
    }
    else if temp  < 10 {
        println!("Really Cold!");
    }
    else{
        println!("Temprature is Ok!");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};

    println!("Today is {}", day);

    println!("is it {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"});

}


fn main() {
    if_statemenet();
}
