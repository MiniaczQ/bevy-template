//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::Screen;
use crate::util::ui::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);

    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, PartialEq, Eq)]
enum TitleAction {
    Play,
    Credits,
}

fn enter_title(mut commands: Commands) {
    commands
        .my_root(|children| {
            children.my_button("Play").insert(TitleAction::Play);
            children.my_button("Credits").insert(TitleAction::Credits);
        })
        .insert(StateScoped(Screen::Title));
}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: ButtonInteractionQuery<TitleAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::Playing),
                TitleAction::Credits => next_screen.set(Screen::Credits),
            }
        }
    }
}
