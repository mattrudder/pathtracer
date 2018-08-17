use super::math::{Collidable, Sphere, RayHit, Ray};
use super::material::Material;
use std::sync::Arc;

pub trait SceneItem: Collidable<Ray> {
    fn get_material(&self) -> Arc<dyn Material>;
}


pub struct SphereGeometry {
    pub sphere: Sphere,
    pub material: Arc<dyn Material>,
}

impl SphereGeometry {
    pub fn new(sphere: Sphere, material: Arc<dyn Material>) -> SphereGeometry {
        SphereGeometry { sphere, material }
    }
}

impl Collidable<Ray> for SphereGeometry {
    type Output = Option<RayHit>;

    fn hit(&self, r: Ray) -> Option<RayHit> {
        self.sphere.hit(r)
    }
}

impl SceneItem for SphereGeometry {
    fn get_material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }
}
