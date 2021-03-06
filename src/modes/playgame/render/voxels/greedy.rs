use super::modelsize::ModelSize;
use bengine::Palette;
use std::collections::{HashMap, HashSet};

pub type VoxMap = HashMap<i32, u8>;

pub fn greedy_cubes(
    cube_index: &mut VoxMap,
    output: &mut Vec<f32>,
    size: &ModelSize,
    palette: &Palette,
) {
    let invisible = cube_index
        .iter()
        .filter(|(idx, _)| {
            cube_index.contains_key(&(*idx - 1))
                && cube_index.contains_key(&(*idx + 1))
                && cube_index.contains_key(&(*idx - size.x as i32))
                && cube_index.contains_key(&(*idx + size.x as i32))
                && cube_index.contains_key(&(*idx + (size.x * size.x) as i32))
                && cube_index.contains_key(&(*idx - (size.x * size.x) as i32))
        })
        .map(|(idx, _)| *idx)
        .collect::<HashSet<i32>>();
    //println!("Invisibility cull: {}", invisible.len());
    cube_index.retain(|idx, _| !invisible.contains(idx));

    loop {
        let min_iter = cube_index.keys().min();
        if min_iter.is_none() {
            break;
        } else {
            let idx = *min_iter.unwrap();
            let mat_idx = cube_index.remove(&idx).unwrap();

            let (x, y, z) = size.idxmap(idx as u32);
            let width = grow_right(cube_index, idx as u32, mat_idx);
            let height = grow_down(cube_index, idx as u32, width, mat_idx, size);
            let depth = grow_in(cube_index, idx as u32, width, height, mat_idx, size);
            //let depth = 1;

            super::cube::add_cube_geometry(
                output,
                x as f32,
                y as f32,
                z as f32,
                width as f32,
                height as f32,
                depth as f32,
                mat_idx,
                palette,
            );
        }
    }
}

fn grow_right(cube_index: &mut VoxMap, idx: u32, mat: u8) -> u32 {
    let mut width = 1;
    let mut candidate_idx = idx + 1;

    while cube_index.contains_key(&(candidate_idx as i32))
        && cube_index[&(candidate_idx as i32)] == mat
    {
        cube_index.remove(&(candidate_idx as i32));
        width += 1;
        candidate_idx += 1;
    }

    width
}

fn grow_down(cube_index: &mut VoxMap, idx: u32, width: u32, mat: u8, size: &ModelSize) -> u32 {
    let mut height = 1;
    let mut candidate_idx = idx + size.x;
    'outer: loop {
        for cidx in candidate_idx..candidate_idx + width {
            if !cube_index.contains_key(&(cidx as i32)) {
                break 'outer;
            }
            if cube_index[&(cidx as i32)] != mat {
                break 'outer;
            }
        }

        for cidx in candidate_idx..candidate_idx + width {
            cube_index.remove(&(cidx as i32));
        }
        height += 1;
        candidate_idx += size.x;
    }
    height
}

fn grow_in(
    cube_index: &mut VoxMap,
    idx: u32,
    width: u32,
    height: u32,
    mat: u8,
    size: &ModelSize,
) -> u32 {
    let mut depth = 1;
    let layer_size = size.x * size.y;
    let mut candidate_idx = idx + layer_size;
    'outer: loop {
        for x in 0..width {
            for y in 0..height {
                let cidx = candidate_idx + x + (y * size.x);
                if !cube_index.contains_key(&(cidx as i32)) {
                    break 'outer;
                }
                if cube_index[&(cidx as i32)] != mat {
                    break 'outer;
                }
            }
        }

        for x in 0..width {
            for y in 0..height {
                let cidx = candidate_idx + x + (y * size.x);
                cube_index.remove(&(cidx as i32));
            }
        }
        depth += 1;
        candidate_idx += layer_size;
    }
    depth
}
