use crate::GameState;
use bevy::prelude::*;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub grabbed_mouse: bool,
    pub trigger_pressed: bool,
}

fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
) {
    if GameControl::Up.just_released(&keyboard_input, &buttons)
        || GameControl::Up.pressed(&keyboard_input, &buttons)
        || GameControl::Left.just_released(&keyboard_input, &buttons)
        || GameControl::Left.pressed(&keyboard_input, &buttons)
        || GameControl::Down.just_released(&keyboard_input, &buttons)
        || GameControl::Down.pressed(&keyboard_input, &buttons)
        || GameControl::Right.just_released(&keyboard_input, &buttons)
        || GameControl::Right.pressed(&keyboard_input, &buttons)
    {
        let mut player_movement = Vec2::ZERO;

        if GameControl::Up.just_released(&keyboard_input, &buttons)
            || GameControl::Down.just_released(&keyboard_input, &buttons)
        {
            if GameControl::Up.pressed(&keyboard_input, &buttons) {
                player_movement.y = 1.;
            } else if GameControl::Down.pressed(&keyboard_input, &buttons) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if GameControl::Up.just_pressed(&keyboard_input, &buttons) {
            player_movement.y = 1.;
        } else if GameControl::Down.just_pressed(&keyboard_input, &buttons) {
            player_movement.y = -1.;
        } else {
            player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }

        if GameControl::Right.just_released(&keyboard_input, &buttons)
            || GameControl::Left.just_released(&keyboard_input, &buttons)
        {
            if GameControl::Right.pressed(&keyboard_input, &buttons) {
                player_movement.x = 1.;
            } else if GameControl::Left.pressed(&keyboard_input, &buttons) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if GameControl::Right.just_pressed(&keyboard_input, &buttons) {
            player_movement.x = 1.;
        } else if GameControl::Left.just_pressed(&keyboard_input, &buttons) {
            player_movement.x = -1.;
        } else {
            player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        }

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            actions.player_movement = Some(player_movement);
        }
    } else {
        actions.player_movement = None;
    }

    if GameControl::MouseGrab.just_released(&keyboard_input, &buttons) {
        if actions.grabbed_mouse {
            actions.grabbed_mouse = false;
        } else {
            actions.grabbed_mouse = true;
        }
    }

    if GameControl::Trigger.pressed(&keyboard_input, &buttons) {
        actions.trigger_pressed = true;
    } else {
        actions.trigger_pressed = false;
    }
}

enum GameControl {
    Up,
    Down,
    Left,
    Right,
    MouseGrab,
    Trigger,
}

impl GameControl {
    fn just_released(
        &self,
        keyboard_input: &Res<Input<KeyCode>>,
        buttons: &Res<Input<MouseButton>>,
    ) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_released(KeyCode::W)
                    || keyboard_input.just_released(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_released(KeyCode::S)
                    || keyboard_input.just_released(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
            GameControl::MouseGrab => keyboard_input.just_released(KeyCode::Escape),
            GameControl::Trigger => buttons.just_released(MouseButton::Left),
        }
    }

    fn pressed(
        &self,
        keyboard_input: &Res<Input<KeyCode>>,
        buttons: &Res<Input<MouseButton>>,
    ) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
            GameControl::MouseGrab => keyboard_input.pressed(KeyCode::Escape),
            GameControl::Trigger => buttons.pressed(MouseButton::Left),
        }
    }

    fn just_pressed(
        &self,
        keyboard_input: &Res<Input<KeyCode>>,
        buttons: &Res<Input<MouseButton>>,
    ) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_pressed(KeyCode::S)
                    || keyboard_input.just_pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
            GameControl::MouseGrab => keyboard_input.just_pressed(KeyCode::Escape),
            GameControl::Trigger => buttons.just_pressed(MouseButton::Left),
        }
    }
}
