use std::env;
use nalgebra::Vector3;
mod gyptlib;

fn main() {
    env::args().for_each(|a| println!("{:?}", a));
    if env::args().count() < 4
    {
        println!("Number of arguments entered too small! Aborting");
        print_usage();
        return
    }
    let obj_filename = env::args().nth(1).unwrap();
    let output_filename = env::args().nth(2).unwrap();
    let particle_radius = env::args().nth(3).unwrap().parse::<f32>().unwrap();
    let scale_opt = env::args().nth(4).unwrap().parse::<f32>();
    let mut scale = Vector3::new(1.0, 1.0, 1.0);
    
    if let Ok(u_scale) = scale_opt
    {
        scale = Vector3::new(u_scale, u_scale, u_scale);
    }
    println!("Loading object file");
    let mesh = gyptlib::trimesh_from_obj(&obj_filename).scaled(&scale);
    
    println!("Sampling");
    let sample = salva3d::sampling::shape_volume_ray_sample(&mesh, particle_radius).unwrap();
    println!("Saving");
    gyptlib::save_salva_sample(&output_filename, sample);
}

fn print_usage()
{
    println!("Usage: salva_sampler <Path to obj file> <Path to output file> <particle radius> Option<scale>");
}