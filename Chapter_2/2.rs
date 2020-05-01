/*
    A hashmap is created with key as a string and value as tuple of three strings.
    Current projects data is inserted in it.
    map is iterated using for loop and data is displayed.
*/ 

use std::collections::HashMap;

fn main()
{
    let mut map:HashMap<String,(String,String,String)>=HashMap::new();

    map.insert("Learning Rust".to_string(),("Rust".to_string(),"To get better at RUST".to_string(),"OnGoing".to_string()));
    map.insert("Solving Leetcode Problems".to_string(),("C++".to_string(),"Develop problem solving skill".to_string(),"In Progress".to_string()));
    map.insert("Learning WebDev".to_string(),("React Javascript".to_string(),"Learn to develop websites".to_string(),"In Progress".to_string()));

    print!("\n The ongoing projects are mentioned below\n");

    for (key,value) in map
    {
        println!("\nkey->{}      value->{:?}\n",key,value );
    }

    print!("\n");
}