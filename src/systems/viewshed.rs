use crate::systems::REGION;
use legion::*;
use legion::systems::Schedulable;
use nox_components::*;
use nox_planet::{Region};
use nox_spatial::{mapidx, REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};
use cgmath::{Vector3, InnerSpace};

pub fn build() -> impl Schedulable {
    SystemBuilder::new("calendar")
        .with_query(<(Entity, Read<Position>, Write<FieldOfView>)>::query())
        .with_query(<Read<Light>>::query())
        .build(|_, ecs, _, (fov_list, _)| {
            let mut entities = Vec::<Entity>::new();

            fov_list
                .iter_mut(ecs)
                .filter(|(_, _, fov)| fov.is_dirty)
                .for_each(|(entity, pos, mut fov)| {
                    //println!("{:?}", fov);
                    fov.visible_tiles.clear();
                    let radius = fov.radius as i32;
                    reveal(pos.get_idx(), &mut *fov);
                    let radius_range = (0i32 - radius)..radius;
                    for z in radius_range {
                        for i in (0i32 - radius)..radius {
                            internal_view_to(&*pos, &mut *fov, i as i32, radius as i32, z as i32);
                            internal_view_to(
                                &*pos,
                                &mut *fov,
                                i as i32,
                                0i32 - radius as i32,
                                z as i32,
                            );
                            internal_view_to(&*pos, &mut *fov, radius as i32, i as i32, z as i32);
                            internal_view_to(
                                &*pos,
                                &mut *fov,
                                0i32 - radius as i32,
                                i as i32,
                                z as i32,
                            );
                        }
                    }
                    fov.is_dirty = false;
                    entities.push(*entity);
                }
            );
            entities.iter().for_each(|e| {
                if ecs.entry_ref(*e).unwrap().get_component::<Light>().is_ok() {
                    crate::messaging::lights_changed();
                    println!("Lights changed");
                }
            })
        }
    )
}

#[inline]
fn internal_view_to(pos: &Position, fov: &mut FieldOfView, x: i32, y: i32, z: i32) {
    let radius = fov.radius as f32;
    let start = pos.as_vec3() + Vector3::new(0.5, 0.5, 0.5);
    let end: Vector3<f32> = (x as f32 + start.x, y as f32 + start.y, z as f32 + start.z).into();
    let mut blocked = false;
    let mut last_z = f32::floor(start.z) as i32;
    line_func_3d(start, end, |pos| {
        if pos.x > 0.0
            && pos.x < REGION_WIDTH as f32
            && pos.y > 0.0
            && pos.y < REGION_HEIGHT as f32
            && pos.z > 0.0
            && pos.z < REGION_DEPTH as f32
        {
            let distance = (pos - start).map(|n| n.abs()).magnitude();
            if distance < radius {
                let idx = mapidx(pos.x as usize, pos.y as usize, pos.z as usize);
                if !blocked {
                    reveal(idx, fov);
                }

                let fz = f32::floor(pos.z) as i32;
                // Block on entering a solid tile
                if REGION.read().flag(idx, Region::SOLID) {
                    blocked = true;
                    reveal(idx, fov);
                } else if fz < last_z {
                    // Check if we're trying to go through a floor
                    if REGION.read().is_floor(idx) {
                        blocked = true;
                        reveal(idx, fov);
                    }
                } else if z > last_z {
                    // Check if we're trying to go through a ceiling
                    if REGION.read().is_floor(idx + (REGION_WIDTH * REGION_HEIGHT))
                    {
                        blocked = true;
                        reveal(idx, fov);
                    }
                }

                last_z = fz;
            }
        }
    });
}

fn line_func_3d<F: FnMut(Vector3<f32>)>(start: Vector3<f32>, end: Vector3<f32>, mut func: F) {
    //println!("{:?} -> {:?}", start, end);
    let mut pos = start.clone();
    let length = (start - end).map(|n| n.abs()).magnitude();
    //println!("{:?}", length);
    let step = (start - end) / length;
    for _ in 0..=f32::floor(length) as usize {
        pos += step;
        func(pos);
    }
}

fn reveal(idx: usize, view: &mut FieldOfView) {
    REGION.write().revealed[idx] = true; // TODO: Make conditional
    view.visible_tiles.insert(idx);
}
