/*
 * cube-engine
 *
 * Copyright (C) 2019 SOFe
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub struct IntPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct FloatPos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct CubePos {
    pub batch: IntPos,
    pub local_x: u8,
    pub local_y: u8,
    pub local_z: u8,
}

pub struct CubePrecisePos {
    pub cube: CubePos,
    pub face: u8,
    pub precise_x: f32,
    pub precise_y: f32,
}
