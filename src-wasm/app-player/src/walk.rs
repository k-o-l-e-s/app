use crate::{Player, PlayerState, PlayerStateChangeEvent};
use bevy::prelude::*;
use bevy_rapier::prelude::*;

pub fn walk_animation(
    mut query: Query<(
        &mut Player,
        &mut Timer,
        &RigidBodyVelocity,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    time: Res<Time>,
    mut psc_event: EventWriter<PlayerStateChangeEvent>,
    assets: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (mut player, mut timer, rb_vel, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if rb_vel.linvel.data.0[0][0] > f32::EPSILON {
            sprite.flip_x = false;
        } else if rb_vel.linvel.data.0[0][0] < -f32::EPSILON {
            sprite.flip_x = true;
        }
        match player.state {
            PlayerState::Walk(frame) => {
                if rb_vel.linvel.data.0[0][0].abs() <= f32::EPSILON {
                    timer.reset();
                    player.state = PlayerState::Wait;
                    psc_event.send(PlayerStateChangeEvent {
                        state: PlayerState::Wait,
                    });
                    return;
                }
                timer.tick(time.delta() * rb_vel.linvel.data.0[0][0].abs() as u32);
                if timer.finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    let (handle, state_index) = match frame {
                        0 => (assets.load("MW_Player_MarioMdl_walk.1_0.png"), 1),
                        _ => (assets.load("MW_Player_MarioMdl_walk.0_0.png"), 0),
                    };
                    let idx = texture_atlas.get_texture_index(&handle).unwrap_or_default();
                    sprite.index = idx as u32;
                    player.state = PlayerState::Walk(state_index);
                }
            }
            PlayerState::Wait => {
                if rb_vel.linvel.data.0[0][0].abs() > f32::EPSILON {
                    player.state = PlayerState::Walk(0);
                    psc_event.send(PlayerStateChangeEvent {
                        state: PlayerState::Walk(0),
                    });
                }
            }
            _ => {}
        }
    }
}
