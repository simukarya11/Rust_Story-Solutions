/*
Demonstarting ownership rule..
Aliasing and mutation both does not go hand in hand in RUST. Only one thing at a time

*/

fn main() {

    let mut str1=String::from("String First. ");
    let ref1=&str1;
    let ref2=&str1;
    let ref3=&str1;
    //let ref4=&mut str1;  
    /* -->> The above line wont compile because we already have 3 read-only references active and usin
            a readable referance in line 17. So if we give a mutable reference while read-only references
            are active then there is a chance that mutable reference can change the data, before the read-only
            reference acts on the string. To avoid this confusion we cannot have a mutable reference when 
            multiple read only references are active.
    */
    println!("{}",ref1);

    let mut str2=String::from("String Second. ");
    let refer1=&mut str2;
    let refer2=&str2;

    //refer1.push_str("Extra data.");
    /*-->> The above line wont compile because while we are editing the string we have a read-only reference
           on the string. In this condition data race might occur. Data race means, while refer2 is trying to
           read data, refer1 try to change it. This might cause a memory error. To avoid this, we can either 
           have multiple read-only refernces or one mutable reference; 
    
    */
    
    println!("{}",str1);

}