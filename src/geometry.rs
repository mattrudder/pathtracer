use super::{Collidable, Material, Sphere, SceneItem, RayHit, Ray};
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Sphere(Sphere),
}

#[derive(Debug, Clone, Copy)]
pub struct Geometry {
    primitive: Primitive,
    material: Material,
}

impl Geometry {
    pub fn from_sphere(sphere: Sphere, material: Material) -> Geometry {
        Geometry { primitive: Primitive::Sphere(sphere), material }
    }
}

impl Collidable<Ray> for Geometry {
    type Output = Option<RayHit>;

    fn hit(&self, r: Ray) -> Option<RayHit> {
        match self.primitive {
            Primitive::Sphere(sphere) => sphere.hit(r)
        }
    }
}

impl SceneItem for Geometry {
    fn get_material(&self) -> Material {
        self.material
    }
}
