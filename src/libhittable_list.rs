use crate::{libhittable::{hittable, hit_record}, libray::ray};

pub struct hittable_list {
    pub objects: Vec<hittable>
}

impl hittable_list {
    pub fn new() -> hittable_list {
        hittable_list {
            objects: Vec::new()
        }
    }

    pub fn clear(mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: hittable) {
        self.objects.push(object);
    }

    pub fn hit (&self, r: ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        let mut temp_rec = hit_record::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let mut temp_rec = temp_rec.clone();
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}