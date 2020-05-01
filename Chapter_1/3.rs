/*
Declaring a struct and using its indexing.
*/
#[derive(Debug)]
struct Point{
    x:i32,
    y:i32,
    z:i32,

}

fn main()
{
    //Below is a way to initalize an object for a structure.
    let obj1=Point{
                        x:2,
                        y:3,
                        z:4                  
                        };

   let obj2=Point{
                    x:22,
                    y:34,
                    z:45                  
                       };


    print!("{}\n",obj2.x-obj1.x );
    print!("{}\n",obj2.y-obj1.y );
    print!("{}\n",obj2.z- obj1.z );
    
    //To successfully compile this file we need to include #[derive(Debug)]. By doing this we can access display 
    // function even for user defined types
    
                        
}