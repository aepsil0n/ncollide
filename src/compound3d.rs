extern crate "nalgebra" as na;
extern crate ncollide;

use na::{Iso3, Vec3};
use ncollide::shape::{Cuboid, Compound, CompoundData};

fn main() {
    // Delta transformation matrices.
    let delta1 = Iso3::new(Vec3::new(0.0f32, -1.5, 0.0), na::zero());
    let delta2 = Iso3::new(Vec3::new(-1.5f32, 0.0, 0.0), na::zero());
    let delta3 = Iso3::new(Vec3::new(1.5f32, 0.0,  0.0), na::zero());

    // 1) Initialize the CompoundData.
    let mut compound_data = CompoundData::new();
    compound_data.push_shape(delta1, Cuboid::new(Vec3::new(1.5, 0.25, 0.25)), 1.0);
    compound_data.push_shape(delta2, Cuboid::new(Vec3::new(0.25, 1.5, 0.25)), 1.0);
    compound_data.push_shape(delta3, Cuboid::new(Vec3::new(0.25, 1.5, 0.25)), 1.0);

    // 2) Create the compound shape.
    let compound = Compound::new(compound_data);

    assert!(compound.shapes().len() == 3)
}