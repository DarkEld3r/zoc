use std::fmt::{Debug};
use core::game_state::{State};
use core::unit::{UnitId};
use core::position::{MapPos};
use core::effect;
use types::{WorldPos, Time, Speed};
use geom;
use context::{Context};
use scene::{Scene, SceneNode, SceneNodeType};
use unit_type_visual_info::{UnitTypeVisualInfoManager};
use mesh_manager::{MeshIdManager, MeshManager};
use camera::{Camera};

mod remove_child;
mod add_object;
mod remove_object;
mod create_unit;
mod remove_unit;
mod remove_mesh;
mod sleep;
mod rotate_to;
mod set_color;
mod change_color;
mod move_to;
mod try_fix_attached_unit;
mod detach;
mod create_text_mesh;
mod create_node;
mod remove_node;

pub use self::remove_child::RemoveChild;
pub use self::add_object::AddObject;
pub use self::remove_object::RemoveObject;
pub use self::create_unit::CreateUnit;
pub use self::remove_unit::RemoveUnit;
pub use self::remove_mesh::RemoveMesh;
pub use self::sleep::Sleep;
pub use self::rotate_to::RotateTo;
pub use self::set_color::SetColor;
pub use self::change_color::ChangeColor;
pub use self::move_to::MoveTo;
pub use self::try_fix_attached_unit::TryFixAttachedUnit;
pub use self::detach::Detach;
pub use self::create_text_mesh::CreateTextMesh;
pub use self::create_node::CreateNode;
pub use self::remove_node::RemoveNode;

// TODO: Move to some other place
pub const WRECKS_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

// TODO: RENAME
// TODO: Move to tactical_screen.rs?
pub struct ActionContext<'a> {
    pub camera: &'a Camera,
    pub mesh_ids: &'a MeshIdManager,
    pub scene: &'a mut Scene,
    pub context: &'a mut Context,
    pub meshes: &'a mut MeshManager,
    pub visual_info: &'a UnitTypeVisualInfoManager,

    // TODO: pub state: &State, // ???
}

pub trait Action: Debug {
    fn is_finished(&self) -> bool { true }

    // TODO: I'm not sure that `begin\end` must mutate the scene
    // TODO: Can I get rid of begin and end somehow? Should I?
    fn begin(&mut self, _: &mut ActionContext) {}
    fn update(&mut self, _: &mut ActionContext, _: Time) {}
    fn end(&mut self, _: &mut ActionContext) {}
}

// TODO Черт, меня бесит что теперь повсюду будут летать
// изменяемые ссылки на ActionContext, в котором ВСЕ.
//
// По хорошему, при создании новых действий,
// ссылка должна быть только на чтение для всего,
// кроме выделение nide_id, mesh_id.
// Тут без имзеняемости, видимо, никак.
//
// Что в Action::begin и т.п. будет изменяемый &mut ActionContext
// меня уже не так волнует.
//
// Может, есть способ избавиться от mut тут?
// Эти айдишники мне нужны только же для связи Action'ов
// между собой. Хмм, могу я что-то другое использовать для этого?
//
//
// TODO: is a visualize_** function? In what file should I put it?
//
// TODO: action::Chain?
//
pub fn visualize_show_text(
    context: &mut ActionContext,
    destination: MapPos,
    text: &str,
) -> Vec<Box<Action>> {
    let node_id = context.scene.allocate_node_id();
    let mesh_id = context.meshes.allocate_id();
    let mut from = geom::map_pos_to_world_pos(destination);
    from.v.z += 0.3;
    let mut to = geom::map_pos_to_world_pos(destination);
    to.v.z += 1.5;
    let node = SceneNode {
        pos: from,
        rot: context.camera.get_z_angle(), // TODO: !?
        mesh_id: Some(mesh_id),
        color: [0.0, 0.0, 1.0, 1.0],
        node_type: SceneNodeType::Transparent,
        .. Default::default()
    };
    vec![
        CreateTextMesh::new(text.into(), mesh_id),
        CreateNode::new(node_id, node),
        MoveTo::new(node_id, Speed{n: 1.0}, to),
        RemoveNode::new(node_id),
        RemoveMesh::new(mesh_id),
    ]
}

// TODO: Remove
/*
#[derive(Debug)]
pub struct EventAttackUnitVisualizer {
    shell_move: Option<MoveHelper>,
    shell_node_id: Option<NodeId>,
    attack_info: AttackInfo,
}
*/

// this code was removed from `visualize_event_attack`
/*
    let attack_info = attack_info.clone();
    let to = WorldPos{v: from.v - geom::vec3_z(geom::HEX_EX_RADIUS / 2.0)};
    let speed = Speed{n: 1.0};
    let move_helper = MoveHelper::new(from, to, speed);
    let is_target_destroyed = defender.count - attack_info.killed <= 0;
    if attack_info.killed > 0 {
        context.text.add_text(
            defender.pos.map_pos,
            &format!("-{}", attack_info.killed),
        );
    } else {
        context.text.add_text(defender.pos.map_pos, "miss");
    }
    let is_target_suppressed = defender.morale < 50
        && defender.morale + attack_info.suppression >= 50;
    if is_target_destroyed {
        if let Some(attached_unit_id) = defender.attached_unit_id {
            let attached_unit = state.unit(attached_unit_id);
            let attached_unit_mesh_id = visual_info
                .get(attached_unit.type_id).mesh_id;
            show_unit_at(
                state,
                scene,
                attached_unit,
                attached_unit_mesh_id,
                mesh_ids.marker_mesh_id,
            );
            // TODO: fix attached unit pos
        }
    } else {
        context.text.add_text(
            defender.pos.map_pos,
            &format!("morale: -{}", attack_info.suppression),
        );
        if is_target_suppressed {
            context.text.add_text(defender.pos.map_pos, "suppressed");
        }
    }
*/

// TODO: split this effect into many
pub fn visualize_effect_attacked(
    state: &State,
    context: &mut ActionContext,
    target_id: UnitId,
    effect: &effect::Attacked,
) -> Vec<Box<Action>> {
    let mut actions = vec![];
    let target = state.unit(target_id);
    actions.extend(visualize_show_text(
        context, target.pos.map_pos, "attacked"));
    if effect.killed > 0 {
        actions.extend(visualize_show_text(
            context, target.pos.map_pos, &format!("killed: {}", effect.killed)));
    } else {
        actions.extend(visualize_show_text(
            context, target.pos.map_pos, "miss")); // TODO: check position
    }
    // TODO: вертолеты, прицепы?
    let target_node_id = context.scene.unit_id_to_node_id(target_id);
    if effect.killed > 0 {
        let children = context.scene.node_mut(target_node_id).children.clone(); // TODO: remove clone
        let killed = effect.killed as usize;
        assert!(killed <= children.len());
        for i in 0 .. killed {
            if effect.leave_wrecks {
                actions.push(SetColor::new(children[i], WRECKS_COLOR));
            } else {
                {
                    let pos = context.scene.node(children[i]).pos;
                    let to = WorldPos{v: pos.v - geom::vec3_z(geom::HEX_EX_RADIUS / 2.0)};
                    actions.push(MoveTo::new(children[i], Speed{n: 1.0}, to));
                }
                actions.push(RemoveChild::new(target_node_id, 0));
            }
        }
        let is_target_destroyed = target.count - effect.killed <= 0;
        if is_target_destroyed {
            if target.attached_unit_id.is_some() {
                actions.push(RemoveChild::new(target_node_id, 0));
            }
            let marker_child_id = (children.len() - killed) as i32 - 1;
            actions.push(RemoveChild::new(target_node_id, marker_child_id));
            if !effect.leave_wrecks {
                // assert_eq!(children.len(), 0); // TODO: how can i check this now?
                actions.push(RemoveUnit::new(target_id));
            }
        }
    }
    /*
    let mut text = String::new();
    text += match effect.effect {
        Effect::Immobilized => "Immobilized",
        Effect::WeaponBroken => "WeaponBroken",
        Effect::ReducedMovement => "ReducedMovement",
        Effect::ReducedAttackPoints => "ReducedAttackPoints",
        Effect::Pinned => "Pinned",
    };
    text += ": ";
    text += match effect.time {
        effect::Time::Forever => "Forever",
        // TODO: показать число ходов:
        effect::Time::Turns(_) => "Turns(n)",
        effect::Time::Instant => "Instant",
    };
    context.text.add_text(unit_pos, &text);
    */
    // TODO: визуализировать как-то
    actions
}

/*
impl Action for EventAttackUnitVisualizer {
    fn is_finished(&self) -> bool {
        if self.attack_info.killed > 0 && !self.attack_info.leave_wrecks {
            self.move_helper.is_finished()
        } else if let Some(ref shell_move) = self.shell_move {
            shell_move.is_finished()
        } else {
            true
        }

        // TODO: воскреси старую логику
        //
        // Вообще, это странный момент: как визуализировать событие атаки,
        // если оно из засады и я вообще не могу рисовать снаряд?
        //
        // Может, надо как-то обозначать район, из которого "прилетело"?
        // В духе "случайно сдвинутый круг из 7 клеток,
        // из одной из которых и стреляли".
        //
        if let Some(ref shell_move) = self.shell_move {
            shell_move.is_finished()
        } else {
            true
        }
    }

    fn update(&mut self, _: &mut ActionContext, scene: &mut Scene, dtime: Time) {
        if let Some(ref mut shell_move) = self.shell_move {
            let shell_node_id = self.shell_node_id.unwrap();
            let mut pos = shell_move.step(dtime);
            if self.attack_info.is_inderect {
                pos.v.z += (shell_move.progress() * PI).sin() * 5.0;
            }
            scene.node_mut(shell_node_id).pos = pos;
        }
        let is_shell_ok = if let Some(ref shell_move) = self.shell_move {
            shell_move.is_finished()
        } else {
            true
        };
        if is_shell_ok && self.shell_move.is_some() {
            if let Some(shell_node_id) = self.shell_node_id {
                scene.remove_node(shell_node_id);
            }
            self.shell_move = None;
            self.shell_node_id = None;
        }
        if is_shell_ok && self.attack_info.killed > 0 {
            let step = self.move_helper.step_diff(dtime);
            let children = &mut scene.node_mut(self.defender_node_id).children;
            for i in 0 .. self.attack_info.killed as usize {
                let child = children.get_mut(i)
                    .expect("update: no child");
                if !self.attack_info.leave_wrecks {
                    child.pos.v += step;
                }
            }
        }
    }

    fn end(&mut self, _: &mut Scene) {
        if self.attack_info.killed > 0 {
            let children = &mut scene.node_mut(self.defender_node_id).children;
            let killed = self.attack_info.killed as usize;
            assert!(killed <= children.len());
            for i in 0 .. killed {
                if self.attack_info.leave_wrecks {
                    children[i].color = WRECKS_COLOR;
                } else {
                    let _ = children.remove(0);
                }
            }
        }
        if self.is_target_destroyed {
            if self.attached_unit_id.is_some() {
                scene.node_mut(self.defender_node_id).children.pop().unwrap();
            }
            // delete unit's marker
            scene.node_mut(self.defender_node_id).children.pop().unwrap();
            if !self.attack_info.leave_wrecks {
                assert_eq!(scene.node(self.defender_node_id).children.len(), 0);
                scene.remove_node(self.defender_node_id);
            }
        }
    }
}
*/