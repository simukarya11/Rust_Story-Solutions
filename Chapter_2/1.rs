/*

A vector is declared and address of its elements is diplayed by iterating over vector v;
the for loop takes a read-only reference and then we display the value of those references which are 
addresses of the vector indices.

*/


fn main()
{
    let mut v:Vec<i32>=vec![11,22,33,44,55];

    let r1=&v[0];

    //Below is a method shown to iterate over a vctor with index and its value.
    for (index,num) in v.iter().enumerate()
    {
        println!("The value at index ->{} is -> {} and its address is -> {:?}",index,num,num as *const i32 );
    }
   




}