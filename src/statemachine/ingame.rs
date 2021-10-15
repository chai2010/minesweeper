// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::DrawColors;

use crate::mouse::Mouse;

use crate::Map;

use crate::Timer;

use crate::wasm4::text;

use super::{State, Transition};

const MINES_COUNT: usize = 5;

#[derive(Clone)]
pub struct InGameState {
    map: Map,
    timer: Timer,
    mines: Vec<(usize, usize)>,
}

impl InGameState {
    pub fn new(map: Map, mines: Vec<(usize, usize)>) -> Self {
        Self {
            map,
            mines,
            timer: Timer::new(),
        }
    }

    pub fn draw(&self) {
        let map = &self.map;

        // Draw map
        map.draw(&self.mines);

        // Draw remaining mines count
        let flagged_tiles = map.count_flagged_tiles();
        let remaining_mines = MINES_COUNT - flagged_tiles;
        let s = format!("Mines:{:02}", remaining_mines);
        DrawColors.set(0x03);
        text(&s, 160 - 64, 2);

        // Draw elapsed time
        let s = format!("Time:{:3}", self.timer.get());
        text(s, 2, 2);

        // Draw game status
        if map.has_stepped_on_mine(&self.mines) {
            text("GAME OVER!!!", 2, 10);
        }

        if self.has_found_all_mines() {
            text("GAME WON!!!", 2, 10);
        }
    }

    pub fn update(mut self, _mouse: &Mouse) -> Transition {
        let has_found_all_mines = self.has_found_all_mines();
        let map = &mut self.map;

        if !map.has_stepped_on_mine(&self.mines) && !has_found_all_mines {
            if Mouse.left_clicked() {
                let (x, y) = Mouse.coordinates();
                map.handle_left_click(x, y, &self.mines);
            }
            if Mouse.right_clicked() {
                let (x, y) = Mouse.coordinates();
                map.handle_right_click(x, y);
            }
        }

        if map.has_started() && !map.has_stepped_on_mine(&self.mines) && !has_found_all_mines {
            self.timer.update();
        }

        Transition::Replace(State::InGame(self))
    }

    fn has_found_all_mines(&self) -> bool {
        self.map.count_uncovered_tiles() + MINES_COUNT == self.map.width() * self.map.height()
    }
}
