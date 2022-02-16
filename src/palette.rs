//
// Copyright © 2020-2021  Egidijus Lileika
//
// This file is part of Archmage - Fantasy Virtual Machine
//
// Archmage is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Archmage is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Archmage. If not, see <https://www.gnu.org/licenses/>.
//

use flask::color::Color;

// https://lospec.com/palette-list/color-graphics-adapter
pub fn archmage_palette() -> Vec<Color> {
    let mut palette = vec![];

    palette.push(Color::from_hex(0x000000));
    palette.push(Color::from_hex(0x555555));
    palette.push(Color::from_hex(0xAAAAAA));
    palette.push(Color::from_hex(0xFFFFFF));
    palette.push(Color::from_hex(0x0000AA));
    palette.push(Color::from_hex(0x5555FF));
    palette.push(Color::from_hex(0x00AA00));
    palette.push(Color::from_hex(0x55FF55));
    palette.push(Color::from_hex(0x00AAAA));
    palette.push(Color::from_hex(0x55FFFF));
    palette.push(Color::from_hex(0xAA0000));
    palette.push(Color::from_hex(0xFF5555));
    palette.push(Color::from_hex(0xAA00AA));
    palette.push(Color::from_hex(0xFF55FF));
    palette.push(Color::from_hex(0xAA5500));
    palette.push(Color::from_hex(0xFFFF55));

    palette
}