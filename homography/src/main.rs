extern crate nalgebra as na;

fn main(){

    let x00:f64 = 120.; let y00:f64 = 350.;
    let x01:f64 = 100.; let y01:f64 =  50.;
    let x02:f64 = 600.; let y02:f64 = 200.;
    let x03:f64 = 500.; let y03:f64 = 500.;

    let x10:f64 =  50.; let y10:f64 = 400.;
    let x11:f64 =  50.; let y11:f64 =  50.;
    let x12:f64 = 500.; let y12:f64 =  50.;
    let x13:f64 = 500.; let y13:f64 = 400.;


    let a = na::DMatrix::from_iterator(8,8,[
        x00, 0., x01, 0., x02, 0., x03, 0.,
        y00, 0., y01, 0., y02, 0., y03, 0.,
        1., 0., 1., 0., 1., 0., 1., 0.,
        0., x00, 0., x01, 0., x02, 0., x03,
        0., y00, 0., y01, 0., y02, 0., y03,
        0., 1., 0., 1., 0., 1., 0., 1.,
        -x00*x10, -x00*y10, -x01*x11, -x01*y11, -x02*x12, -x02*y12, -x03*x13, -x03*y13,
        -y00*x10, -y00*y10, -y01*x11, -y01*y11, -y02*x12, -y02*y12, -y03*x13, -y03*y13
    ].iter().cloned());
    println!("A:{}",a);

    let b = na::DVector::from_iterator(8,[
        x10, y10, x11, y11, x12, y12, x13, y13
    ].iter().cloned());
    println!("B:{}",b);

    let inv_a = a.try_inverse().unwrap();
    println!("invA:{}",inv_a);

    let h = inv_a.clone()*b.clone();
    println!("x:{}", inv_a*b);
    println!("x:{}", h);


    let homography = na::Matrix3::new(
        h[0], h[1], h[2], 
        h[3], h[4], h[5], 
        h[6], h[7], 1.
    );
    println!("Homography:{}", homography);


    let x0 = na::DVector::from_iterator(3,[
        x00, y00, 1.
    ].iter().cloned());
    println!("x0:{}",x0);

    let xx = homography.clone()*x0.clone();
    println!("x10:{}", xx[0]/xx[2]);
    println!("y10:{}", xx[1]/xx[2]);


}