use crate::{Player, PlayerStateEnum};
use app_config::{HIGH_JUMP_TICK, HIGH_JUMP_TICK_WALK, HIGH_JUMP_WALK_THRESHOLD, JUMP_FORCE};
use bevy::prelude::*;
use bevy_rapier::prelude::*;

pub struct FallEvent;

pub struct JumpEvent {
    pub high_jump_tick: u8,
}

pub fn jump(
    mut query: Query<(&Player, &mut RigidBodyVelocity)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut psc_event: EventWriter<JumpEvent>,
) {
    if let Ok((player, mut rb_vel)) = query.single_mut() {
        let jump = keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::W);
        if !jump {
            return;
        }
        match player.state.state {
            PlayerStateEnum::Wait { .. } | PlayerStateEnum::Walk { .. } => {
                rb_vel.linvel.data.0[0][1] = JUMP_FORCE;
                let high_jump_tick = if rb_vel.linvel.data.0[0][0].abs() > HIGH_JUMP_WALK_THRESHOLD
                {
                    HIGH_JUMP_TICK_WALK
                } else {
                    HIGH_JUMP_TICK
                };
                psc_event.send(JumpEvent { high_jump_tick })
            }
            _ => {}
        }
    }
}

pub fn high_jump(
    mut query: Query<(&mut Player, &mut RigidBodyVelocity)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut player, mut rb_vel)) = query.single_mut() {
        match player.state.state {
            PlayerStateEnum::Jump {
                tick,
                high_jump_tick,
                released: false,
                impulse,
                ..
            } if tick < high_jump_tick => {
                let released = keyboard_input.just_released(KeyCode::Space)
                    || keyboard_input.just_released(KeyCode::Up)
                    || keyboard_input.just_released(KeyCode::W);
                let jump = keyboard_input.pressed(KeyCode::Space)
                    || keyboard_input.pressed(KeyCode::Up)
                    || keyboard_input.pressed(KeyCode::W);

                rb_vel.linvel.data.0[0][1] = JUMP_FORCE;
                if released {
                    player.state.state = PlayerStateEnum::Jump {
                        tick: 0,
                        high_jump_tick,
                        released,
                        impulse,
                    };
                } else if jump {
                    player.state.state = PlayerStateEnum::Jump {
                        tick: tick + 1,
                        high_jump_tick,
                        released: false,
                        impulse,
                    };
                }
            }
            PlayerStateEnum::Jump {
                tick,
                high_jump_tick,
                released: false,
                impulse,
            } if tick < high_jump_tick => {
                rb_vel.linvel.data.0[0][1] = JUMP_FORCE;
                player.state.state = PlayerStateEnum::Jump {
                    tick: 0,
                    high_jump_tick,
                    released: true,
                    impulse,
                };
            }
            _ => {}
        }
    }
}

pub fn jump_to_fall(
    query: Query<(&Player, &RigidBodyVelocity)>,
    mut fall_events: EventWriter<FallEvent>,
) {
    if let Ok((player, rb_vel)) = query.single() {
        if let PlayerStateEnum::Jump { .. } = player.state.state {
            if rb_vel.linvel.data.0[0][1] < 0. {
                fall_events.send(FallEvent);
            }
        }
    }
}
