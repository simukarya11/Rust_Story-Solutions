/*
    Using type we can perform aliasing in Rust.
    Here we initiate types which are used further in main function to initialse a tuple.

*/


type year=i32;
type name=String;
type occupation=String;



fn main()
{
 
    let tuple:(year,name,occupation)=(2020,"Simuk".to_string(),"Student".to_string());

    println!("The values inside tuple are ->{:?}",tuple);


}