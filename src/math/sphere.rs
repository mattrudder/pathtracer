use super::*;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl RayHitable for Sphere {
    fn ray_hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None
        }

        let d = (b.powi(2) - (a * c)).sqrt();
        let t = (-b - d) / a;
        if t < t_max && t > t_min {
            let point = r.point_at_parameter(t);
            let normal = (point - self.center) / self.radius;
            Some(RayHit { t, point, normal })
        } else {
            let t = (-b - d) / a;
            if t < t_max && t > t_min {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center) / self.radius;
                Some(RayHit { t, point, normal })
            } else {
                None
            }
        }
    }
}
