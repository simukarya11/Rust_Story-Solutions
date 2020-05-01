
#[derive(Debug)]
struct Point
{
    point_3d:(i32,i32,i32), //Initialized as a tuple, with three i32 values

}

fn main()
{
   let obj1=Point{
                    point_3d:(11,9,13),    
                    };

    let obj2 =Point{
                    point_3d:(42,15,26),
    };
    // Accessing tuple through object of point
    let n1=obj2.point_3d.0 - obj1.point_3d.0;
    let n2=obj2.point_3d.1- obj1.point_3d.1;
    let n3=obj2.point_3d.2- obj1.point_3d.2;
    
    println!("{:?}",(n1,n2,n3));

}