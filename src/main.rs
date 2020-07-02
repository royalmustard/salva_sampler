use std::env;
use nalgebra::Vector3;
mod gyptlib;

fn main() {
    env::args().for_each(|a| println!("{:?}", a));
    let obj_filename = env::args().nth(1).unwrap();
    let output_filename = env::args().nth(2).unwrap();
    let particle_radius = env::args().nth(3).unwrap().parse::<f32>().unwrap();
    let scale = env::args().nth(4).unwrap().parse::<f32>().unwrap();

    let mesh = gyptlib::trimesh_from_obj(&obj_filename).scaled(&Vector3::new(scale, scale, scale));
    
    
    let sample = salva3d::sampling::shape_volume_ray_sample(&mesh, particle_radius).unwrap();

    gyptlib::save_salva_sample(&output_filename, sample);
}
