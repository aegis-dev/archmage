//
// Copyright © 2020-2022  Egidijus Lileika
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

use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum SysCall {
    None = 0x00,
    RenderVRAM = 0x01,
    PollInputEvents = 0x02,
    // UpdateCursorState = 0x03,
    // GetMouseButtonState= 0x04,
}