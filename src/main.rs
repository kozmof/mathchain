mod vector;
use vector::Vec3;
use vector::distsq;

fn main() {
   let v1: Vec3<f32> = Vec3::new(2.0, 3.0, 4.0);
   let v2: Vec3<f32> = Vec3::new(5.0, 6.0, 7.0);
   let dd = distsq::<f64>(&v1.into(), &v2.into());
   println!("{}", dd.sqrt());
}
