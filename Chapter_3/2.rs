/*
  CLOSURE is defined in this file. Closures are like C++ function poointers.
  We can use entire function as a variable. Rather than storing data, variable will store code.

*/
fn main()
{

    let closure= |msg:&str| {
        let s1 = std::iter::repeat('-').take(msg.len() +2 ).collect::<String>();
        let s2 = std::iter::repeat('-').take(msg.len() +2).collect::<String>();
        println!(" {} \n {} \n {}",s1,&msg,s2 );};

     closure("Hi..!");   

}