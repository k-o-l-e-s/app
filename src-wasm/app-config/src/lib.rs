use bevy_rapier::na::Vector2;

pub const MOVE_IMPULSE_MULTIPLIER_GROUND: f32 = 0.5 * GROUND_FRICTION_MULTIPLIER * RAPIER_GRAVITY;
pub const MOVE_IMPULSE_MULTIPLIER_GROUND_RUN: f32 =
    0.55 * GROUND_FRICTION_MULTIPLIER * RAPIER_GRAVITY;
pub const MOVE_IMPULSE_MULTIPLIER_AIR: f32 = 2700.;
pub const MOVE_IMPULSE_MULTIPLIER_AIR_RUN: f32 = 2800.;

pub const LINVEL_CAP_WALK: f32 = 20.;
pub const LINVEL_CAP_RUN: f32 = 35.;
pub const LINVEL_CAP_STOOP: f32 = 10.;

pub const RUN_THRESHOLD: f32 = 30.;
pub const HIGH_JUMP_WALK_THRESHOLD: f32 = 7.;

pub const JUMP_FORCE: f32 = 37.;
pub const HIGH_JUMP_TICK: u8 = 15;
pub const HIGH_JUMP_TICK_WALK: u8 = 22;

pub const RAPIER_SCALE: f32 = 10.;
pub const RAPIER_GRAVITY: f32 = 200.;
pub const RAPIER_GRAVITY_VECTOR: Vector2<f32> = Vector2::new(0., -RAPIER_GRAVITY);

const GROUND_FRICTION_MULTIPLIER: f32 = 30.;
pub const GROUND_FRICTION: f32 = GROUND_FRICTION_MULTIPLIER / RAPIER_GRAVITY;

// TODO run animation
// TODO run turn animation
// TODO run animation from velocity
