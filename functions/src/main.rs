
fn print_value(x:i32)
{
    println!("value = {}",x);
}
fn increase(x:&mut i32)
{
    *x += 1;
}

fn product(x:i32,y:i32) -> i32
{
     x * y
}

fn functions()
{
    print_value(35);


    let mut z = 1;
    increase(&mut z);
    println!("z = {}",z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    print!("p = {}",p);

}

fn main() {
    functions();
}
