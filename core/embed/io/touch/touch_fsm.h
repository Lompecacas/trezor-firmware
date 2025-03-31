/*
 * This file is part of the Trezor project, https://trezor.io/
 *
 * Copyright (c) SatoshiLabs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#pragma once

// This module is a simple finite state machine for touch events.
//
// The module is designed to be used with a touch driver that provides a
// function to read the touch state. The touch state is a bitmask that indicates
// the state of the touch panel.
//
// The module implement `touch_get_event()` function that reads the touch
// state and returns the events that happened since the last call to
// `touch_get_event()`. This function can be called from different tasks, and
// it will keep track of the state changes for each task separately,
// since each task has its own state machine.
//
// The module also registers itself as a sysevent source - SYSHANDLE_TOUCH,
// so it can be polled in the task event loop.

// Initializes the touch state machine for all tasks and
// registers the touch event source.
bool touch_fsm_init(void);

// Deinitializes the touch state machine and unregisters
// the touch event source.
void touch_fsm_deinit(void);

// The function is meant to be implemented in the touch driver code.
// It reads the touch registers and returns the last touch event.
uint32_t touch_get_state(void);
