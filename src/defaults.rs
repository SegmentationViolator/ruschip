//    Copyright (C) 2023 Segmentation Violator <segmentationviolator@proton.me>

//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::backend;
use crate::frontend;
use crate::ui;

pub const COLORS: frontend::Colors = frontend::Colors {
    active: ui::PRIMARY_COLOR,
    inactive: ui::SECONDARY_COLOR,
};

pub const FONT: [u8; backend::FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub const KEY_MAP: [egui::Key; backend::KEY_COUNT] = [
    egui::Key::X,    // 0
    egui::Key::Num1, // 1
    egui::Key::Num2, // 2
    egui::Key::Num3, // 3
    egui::Key::Q,    // 4
    egui::Key::W,    // 5
    egui::Key::E,    // 6
    egui::Key::A,    // 7
    egui::Key::S,    // 8
    egui::Key::D,    // 9
    egui::Key::Z,    // A
    egui::Key::C,    // B
    egui::Key::Num4, // C
    egui::Key::R,    // D
    egui::Key::F,    // E
    egui::Key::V,    // F
];
