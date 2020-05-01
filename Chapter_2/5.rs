/*
    A struct is initialsed and displayed;
    NOTE: We must include #[derive(Debug)]here to print the object directly using print! macro.
*/

#[derive(Debug)]
struct employment{
    name:String,
    occupation:String,
    year:i32

}

fn main()
{
 let object=employment{name:"Simuk".to_string(),
                      occupation:"Student".to_string(),
                      year:2020
                        };

println!("\nThe object has values ->{:?}\n ",object );

}