use std::mem;

fn main() {
    let hello_u_int:u8 = 128; //unsigned 8 bit integer inmutable
    let hello_int:i8 = -128; //signed 8 bit integer inmutable
    println!("helloUint = {}",hello_u_int);
    println!("helloInt = {}", hello_int);

    let mut hello_mut_int:i8 = 2; // signed 8 bit integer mutable

    println!("helloMutInt = {}",hello_mut_int);

    hello_mut_int = 26;

    println!("helloMutInt = {}",hello_mut_int);

    let mut hello_int_32 = 323498032; //32 bit signed integer mutable

    println!("hello_int_32 = {}, size = {} bytes",hello_int_32,mem::size_of_val(&hello_int_32));

    hello_int_32 = -1;

    println!("hello_int_32 = {}", hello_int_32);

    //i8 u8 i16 u16 i32 u32 i64 u64
    //isize usize

    let try_size:isize = 11;
    let size_of_try = mem::size_of_val(&try_size);

    println!("try_size = {}, take up {} bytes, {}-bit OS",try_size,size_of_try,size_of_try * 8);

    let try_char:char = 'c';

    println!("try_char = {}, size = {} bytes",try_char, mem::size_of_val(&try_char));

    let try_float = 2.5;

    println!("try_float = {} size = {} bytes",try_float, mem::size_of_val(&try_float));

    let try_bool = false;

    println!("try_bool = {} size = {} bytes",try_bool, mem::size_of_val(&try_bool));

}
