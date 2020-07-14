use imgui::*;
use legion::prelude::*;
use nox_components::*;

pub fn draw_tooltips(ecs: &World, mouse_world_pos: &(usize, usize, usize), imgui: &Ui) {
    if imgui.io().want_capture_mouse {
        return;
    }

    let mut lines: Vec<(bool, String)> = Vec::new();

    use nox_planet::Region;
    use nox_spatial::{mapidx, REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};
    if mouse_world_pos.0 > 0
        && mouse_world_pos.0 < REGION_WIDTH
        && mouse_world_pos.1 > 0
        && mouse_world_pos.1 < REGION_HEIGHT
        && mouse_world_pos.2 > 0
        && mouse_world_pos.2 < REGION_DEPTH
    {
        let idx = mapidx(mouse_world_pos.0, mouse_world_pos.1, mouse_world_pos.2);
        let r = crate::systems::REGION.read();
        let mut l = String::new();
        if r.flag(idx, Region::SOLID) {
            l += "SOLID|";
        }
        if r.flag(idx, Region::CAN_STAND_HERE) {
            l += "ST|";
        }
        if r.flag(idx, Region::CAN_GO_NORTH) {
            l += "N|";
        }
        if r.flag(idx, Region::CAN_GO_SOUTH) {
            l += "S|";
        }
        if r.flag(idx, Region::CAN_GO_EAST) {
            l += "E|";
        }
        if r.flag(idx, Region::CAN_GO_WEST) {
            l += "W|";
        }
        if r.flag(idx, Region::CAN_GO_UP) {
            l += "U|";
        }
        if r.flag(idx, Region::CAN_GO_DOWN) {
            l += "D|";
        }
        if !l.is_empty() {
            lines.push((false, l));
        }
    }

    <(Read<Name>, Read<Position>, Read<Identity>)>::query()
        .iter_entities(&ecs)
        .filter(|(_, (_, pos, _))| pos.contains_point(mouse_world_pos))
        .for_each(|(entity, (name, _, identity))| {
            lines.push((true, format!("{}", name.name)));

            <Read<Description>>::query()
                .iter_entities(&ecs)
                .filter(|(e, _)| *e == entity)
                .for_each(|(_, d)| {
                    lines.push((false, format!("{}", d.desc)));
                });

            <(Read<Name>, Read<Position>)>::query()
                .iter(&ecs)
                .filter(|(_, store)| store.is_in_container(identity.id))
                .for_each(|(name, _)| {
                    lines.push((false, format!(" - {}", name.name)));
                });
        });

    if !lines.is_empty() {
        let im_lines: Vec<(bool, ImString)> = lines
            .iter()
            .map(|(heading, s)| (*heading, ImString::new(s)))
            .collect();

        let size = crate::engine::DEVICE_CONTEXT.read().as_ref().unwrap().size;
        let mouse_pos = imgui.io().mouse_pos;
        let vsize = im_lines
            .iter()
            .map(|(_, s)| imgui.calc_text_size(s, false, 150.0)[1] + 10.0)
            .sum();

        let tip_pos = [
            f32::min(size.width as f32 - 300.0, mouse_pos[0]),
            f32::min(size.height as f32 - vsize, mouse_pos[1]),
        ];

        imgui::Window::new(im_str!("### tooltip"))
            .no_decoration()
            .size([300.0, vsize], Condition::Always)
            .collapsed(false, Condition::Always)
            .position(tip_pos, Condition::Always)
            .no_inputs()
            .build(imgui, || {
                im_lines.iter().for_each(|(heading, text)| {
                    if *heading {
                        imgui.text_colored([1.0, 1.0, 0.0, 1.0], text);
                    } else {
                        imgui.text_wrapped(text);
                    }
                });
            });
    }
}