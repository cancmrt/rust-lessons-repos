
const MY_CC_CONST:u8 = 6; // no fixed address;

static MY_STATIC_CONST:u8 = 7; //fixed address memory;

static mut MY_STATIC_CONST_UNSAFE:u8 = 8; //fixed address memory this can be cause multiple race condition and unstable changes;

fn scope_and_shadowing()
{
    let a = 123;
    {
        let b = 456;

        println!("inside, b = {}",b);
        
        println!("inside a = {}",a);

        let a =  77;

        println!("inside a = {}",a);

    }
    println!("outside a = {}",a);
}

fn main() {
    scope_and_shadowing();
    println!("{}",MY_CC_CONST);
    println!("{}",MY_STATIC_CONST);

    unsafe{
        println!("{}",MY_STATIC_CONST_UNSAFE);
    }
    
}
