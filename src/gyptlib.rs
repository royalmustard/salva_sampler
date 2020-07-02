use std::path::Path;
use std::fs::File;
use tobj;
use ncollide3d::shape::{TriMesh};
use nalgebra::{Point3};


pub fn trimesh_from_obj(path: &str) -> TriMesh<f32>
{
    let obj_file = tobj::load_obj(&Path::new(path), false);
    let (models, _materials) = obj_file.unwrap();
    let mesh = &models[0].mesh;

    let mut positions = Vec::<Point3<f32>>::new();
    let mut indices = Vec::<Point3<usize>>::new();

    for i in (0..mesh.positions.len()).step_by(3)
    {
        positions.push(Point3::new(mesh.positions[i], mesh.positions[i+1], mesh.positions[i+2]));
    }
    
    for i in (0..mesh.indices.len()).step_by(3)
    {
        indices.push(Point3::new(mesh.indices[i] as usize, mesh.indices[i+1] as usize, mesh.indices[i+2] as usize));
    }


    TriMesh::new(positions, indices, None)
}

pub fn save_salva_sample(path: &str, sample: Vec<Point3<f32>>)
{
    let json_file_path = Path::new(path);
    let json_file = File::create(json_file_path).expect("Error creating file!");
    serde_json::to_writer(json_file, &sample).unwrap();
}
