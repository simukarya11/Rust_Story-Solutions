
struct Data{
    x:i32,
    y:String,
    z:Vec<i32>,
}

impl Data{
    fn display(&self)
    {
        print!("\nThe structure has following values: \n", );
        print!("x-> {}\n",self.x );
        print!("y-> {}\n",self.y );
        print!("z->{:?}\n\n",self.z );
    }
}


fn main()
{   let object= Data{
                       x:4,
                       y:"Object 1".to_string(),
                       z:vec![1,2,3,4,5],
                    };

    object.display();                

}