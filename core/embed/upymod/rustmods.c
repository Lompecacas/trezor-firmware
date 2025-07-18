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

#include "librust.h"
#include "py/runtime.h"

#if MICROPY_PY_TREZORUI_API
MP_REGISTER_MODULE(MP_QSTR_trezorui_api, mp_module_trezorui_api);
#endif

#if MICROPY_PY_TREZORPROTO
MP_REGISTER_MODULE(MP_QSTR_trezorproto, mp_module_trezorproto);
#endif

#if MICROPY_PY_TREZORTRANSLATE
MP_REGISTER_MODULE(MP_QSTR_trezortranslate, mp_module_trezortranslate);
#endif

#ifdef USE_BLE
MP_REGISTER_MODULE(MP_QSTR_trezorble, mp_module_trezorble);
#endif

#if defined(TREZOR_EMULATOR) && PYOPT == 0
MP_REGISTER_MODULE(MP_QSTR_coveragedata, mp_module_coveragedata);
#endif

#if !PYOPT
MP_REGISTER_MODULE(MP_QSTR_trezorlog, mp_module_trezorlog);
#endif
