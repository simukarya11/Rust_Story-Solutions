/* This file demonstrates
   ->declaring array and iterating over it
   ->declaring Vector and iterating over it


*/

fn main()
{
     //*************************************** Array **********************************************


    // Declaring an array that contains 32 bit integer. Initially array has size=5, all filled with 0;
    let arr:[i32;5]=[0;5];

    for num in &arr 
    {
        print!("{} ",num );
    }
    print!("\n" );
    //Declaring another array and changing its values.
    let mut arr1:[i32;6]=[0;6];
    // Because we are using mutable reference, we can edit the values in the array. We need to use
    // derefernce operator(*) to access the value pointed by refernce
    for num in &mut arr1
    {
        *num+=1;
    }
    //If no reference is given then we need to call .iter() on the array, to recive an iterator by reference
    for num in arr1.iter() 
    {
        print!("{} ",num );
    }
    print!("\n" );

    //*************************************** Vector **********************************************

    //The syntax below is used to initialize a vector if we already know what to insert
    let vec1:Vec<i32>=vec![1,2,3,4,5];

    //Another way to initialize a vector.
    let mut vec2:Vec<i32>=Vec::new();

    vec2.push(11);
    vec2.push(22);
    //No need to take reference in vector, as vectors are already allocated on heap, so the for loop
    //itself generates references to vector and then deletes it.
    for num in vec1 
    {
        print!("{} ",num );
    }
    print!("\n" );

    for num in vec2 
    {
        print!("{} ",num );
    }
    print!("\n" );

}