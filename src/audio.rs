use crate::actions::Actions;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioApp, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

struct Walking;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<Walking>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(start_shoot_audio)
                    .with_system(start_walk_audio),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(control_shooting_sound),
            );
    }
}

fn start_shoot_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.set_volume(0.3);
    audio.play_looped(audio_assets.shooting.clone());
    audio.pause();
}

fn start_walk_audio(background: Res<AudioChannel<Walking>>, audio_assets: Res<AudioAssets>) {
    background.set_volume(0.3);
    background.play_looped(audio_assets.walking.clone());
    background.pause();
}

fn control_shooting_sound(
    actions: Res<Actions>,
    audio: Res<Audio>,
    walk_audio: Res<AudioChannel<Walking>>,
) {
    if actions.trigger_pressed {
        audio.resume();
    } else {
        audio.pause()
    }
    if actions.player_movement.is_some() {
        walk_audio.resume();
    } else {
        walk_audio.pause()
    }
}
