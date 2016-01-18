// -*- coding:utf-8-unix -*-

// The MIT License
//
// Copyright (C) 2016 by Tatsuya Kawano <tatsuya@hibaridb.org>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#[macro_use]
extern crate ruster_unsafe;
use ruster_unsafe::*;

use std::mem::uninitialized;

extern crate libc;
use libc::c_double;

mod pi;

static mut ok_atom:    ERL_NIF_TERM = 0 as ERL_NIF_TERM;
static mut error_atom: ERL_NIF_TERM = 0 as ERL_NIF_TERM;

nif_init!(b"Elixir.PiNif\0",
          Some(load),    // on load
          None,          // on reload
          Some(upgrade), // on upgrade
          None,          // on unload
          nif!(b"calc_pi\0",
               1,
               calc_pi,
               ERL_NIF_DIRTY_JOB_CPU_BOUND),
          nif!(b"calc_pi_parallel\0",
               2,
               calc_pi_parallel,
               ERL_NIF_DIRTY_JOB_CPU_BOUND)
         );

/// static な変数にアトムを設定する。
extern "C" fn load(env: *mut ErlNifEnv,
                   _priv_data: *mut *mut c_void,
                   _load_info: ERL_NIF_TERM)-> c_int {
    unsafe {
        ok_atom    = enif_make_atom(env, b"ok\0"    as *const u8);
        error_atom = enif_make_atom(env, b"error\0" as *const u8)
    }
    0
}

/// 何もしない
extern "C" fn upgrade(_env: *mut ErlNifEnv,
                      _priv_data: *mut *mut c_void,
                      _old_priv_data: *mut *mut c_void,
                      _load_info: ERL_NIF_TERM)-> c_int {
    0
}

/// Elixir: @spec calc_pi_nif(n :: non_neg_integer) :: {:ok, pi :: float} | no_return
extern "C" fn calc_pi(env: *mut ErlNifEnv,
                      argc: c_int,
                      args: *const ERL_NIF_TERM) -> ERL_NIF_TERM {

	let mut n: c_int = unsafe { uninitialized() };
    if argc != 1
        || 0 == unsafe { enif_get_int(env, *args, &mut n) }
        || n <= 0 {
        return unsafe { enif_make_badarg(env) };
	}

    match pi::calc_pi(n as u32) {
        Ok(pi) =>
            make_ok_result(env, unsafe { &enif_make_double(env, pi as c_double) } ),
        Err(reason) =>
            make_error_result(env, &reason),
    }
}

/// Elixir: @spec calc_pi_parallel(n :: non_neg_integer,
///                                num_threads :: non_neg_integer)
///                               :: {:ok, pi :: float} | {:error, term()} | no_return
extern "C" fn calc_pi_parallel(env: *mut ErlNifEnv,
                               argc: c_int,
                               args: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
	let mut n: c_int = unsafe { uninitialized() };
    let mut num_threads: c_int = unsafe { uninitialized() };
    if argc != 2
        || 0 == unsafe { enif_get_int(env, *args, &mut n) }
        || 0 == unsafe { enif_get_int(env, *args.offset(1), &mut num_threads) }
        || n <= 0 {
        return unsafe { enif_make_badarg(env) };
    }

    match pi::calc_pi_parallel(n as u32, num_threads as u32) {
        Ok(pi) =>
            make_ok_result(env, unsafe { &enif_make_double(env, pi as c_double) }),
        Err(reason) =>
            make_error_result(env, &reason),
    }
}

fn make_ok_result(env: *mut ErlNifEnv, result: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
    let tuple_elements = unsafe { [ok_atom, *result] };
    unsafe { enif_make_tuple_from_array(env, tuple_elements.as_ptr(), 2) }
}

fn make_error_result(env: *mut ErlNifEnv, reason: &str) -> ERL_NIF_TERM {
    let reason_str = unsafe { enif_make_string_len(env, reason.as_ptr(), reason.len(),
                                                   ErlNifCharEncoding::ERL_NIF_LATIN1) };
    let tuple_elements = [unsafe { error_atom }, reason_str];
    unsafe { enif_make_tuple_from_array(env, tuple_elements.as_ptr(), 2) }
}
