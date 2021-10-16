// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! State machine data types and functions

use once_cell::unsync::Lazy;

use crate::graphics::DrawColors;

use crate::mouse::Mouse;

use crate::debug;

mod initial;
use initial::InitialState;

mod gameover;
use gameover::GameOverState;

mod gamewon;
use gamewon::GameWonState;

mod mainmenu;
use mainmenu::MainMenuState;

mod pause;
use pause::PauseState;

mod pregame;
use pregame::PreGameState;

mod ingame;
use ingame::InGameState;

/// The game state machine
pub static mut STATE_MACHINE: Lazy<Machine> = Lazy::new(|| Machine {
    states_stack: vec![State::Initial(InitialState::new())],
});

/// Main stackable state machine
///
/// The game state is modelled as a stackable state machine.
/// The current state is the one on top of the stack.
/// Three kinds of transition can happen:
///
/// * The current state can switch to a different state;
/// * A new state can be pushed to the stack;
/// * The current state can be popped from the stack.
pub struct Machine {
    states_stack: Vec<State>,
}

impl Machine {
    /// Draw all states in the stack
    pub fn draw(&self) {
        for state in &self.states_stack {
            let original_colors = DrawColors.get();
            state.draw();
            DrawColors.set(original_colors);
        }
    }

    /// Update the top state on the stack
    ///
    /// The update returns a transition, which might change the content of the
    /// stack.
    ///
    /// The top state is always popped from the stack.
    /// If the current state wants to remain on the stack, it must return a
    /// [`Transition::Replace`] transition containing itself.
    pub fn update(&mut self, mouse: &Mouse) {
        let state: State = self.states_stack.pop().expect("Empty state machine!!!");
        let transition: Transition = state.update(mouse);

        match transition {
            Transition::Replace(state) => {
                self.states_stack.push(state);
            }
            Transition::Push(old_state, state) => {
                // First restore old state onto stack
                self.states_stack.push(old_state);

                debug!("Pushing new state to stack");
                self.states_stack.push(state);
                debug!(
                    "Current state: {}",
                    self.states_stack.iter().last().unwrap().name()
                );
            }
            Transition::Pop => {
                debug!("Popping state from stack");
                // Already done at the beginning of this function
                debug!(
                    "Current state: {}",
                    self.states_stack.iter().last().unwrap().name()
                );
            }
        }
    }
}

/// A state transition
pub enum Transition {
    /// The current state is replaced with a new state
    ///
    /// The new state can be identical to the old state, which is functionally
    /// the same as not changing the stack.
    Replace(State),

    /// A new state is pushed on top of the current state
    Push(State, State),

    /// The current state is popped from the stack
    Pop,
}

/// A game state
///
/// Each state maintains its own state data, which is also responsible for
/// drawing and updating itself.
#[derive(Clone)]
pub enum State {
    /// The initial state, created at the state machine initialization
    Initial(InitialState),

    /// The state before the game started
    PreGame(PreGameState),

    /// The state of a running game
    InGame(InGameState),

    /// The state of a lost game
    GameOver(GameOverState),

    /// The state of a won game
    GameWon(GameWonState),

    /// The state of paused game
    Pause(PauseState),

    /// The state of main menu
    MainMenu(MainMenuState),
}

impl State {
    /// Return the name of the state
    pub fn name(&self) -> &'static str {
        match self {
            State::Initial(_) => "initial",
            State::PreGame(_) => "pre_game",
            State::InGame(_) => "in_game",
            State::GameOver(_) => "game_over",
            State::GameWon(_) => "game_won",
            State::Pause(_) => "pause",
            State::MainMenu(_) => "main_menu",
        }
    }

    /// Draw the current state
    ///
    /// This function delegates the drawing to the state data.
    pub fn draw(&self) {
        match self {
            State::Initial(s) => s.draw(),
            State::PreGame(s) => s.draw(),
            State::InGame(s) => s.draw(),
            State::GameOver(s) => s.draw(),
            State::GameWon(s) => s.draw(),
            State::Pause(s) => s.draw(),
            State::MainMenu(s) => s.draw(),
        }
    }

    /// Update the current state
    ///
    /// This function delegates the update to the state data.
    pub fn update(self, mouse: &Mouse) -> Transition {
        match self {
            State::Initial(state) => state.update(mouse),
            State::PreGame(state) => state.update(mouse),
            State::InGame(state) => state.update(mouse),
            State::GameOver(state) => state.update(mouse),
            State::GameWon(state) => state.update(mouse),
            State::Pause(state) => state.update(mouse),
            State::MainMenu(state) => state.update(mouse),
        }
    }
}
