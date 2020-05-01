/*
    String conversion is demonstrated in this file
    String is converted from string literal and vice versa


*/

fn main()
{
    // string literal is defined
    let str1="Hi. I am string literal";
    
    // String on heap is created by using  .to_string()
    let str2=str1.to_string();
    
    // First char of String is picked as a reference
    let n=&mut str2.chars().next().unwrap();

    // Memory addresses are displayed below
    println!("\nThe address of string literal is {:?}",str1 as *const str);
    println!("The address of first element of String on Heap is {:?}\n",n as *const char);
    
    //The line below makes a str component from String
    let s_slice: &str = &str2[..]; 
    println!("\nNew str formed by String is {}",s_slice);
}