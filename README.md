# Elixir Rust Interop Demo

This project demonstrates to call Rust functions from Elixir using
Erlang NIF interface. It depends on
[ruster_unsafe](https://crates.io/crates/ruster_unsafe/) crate,
a low level bindings to Erlang NIF API for Rust.


## Recently Tested With

- Elixir 1.2.0 on Erlang/OTP 18.2.1
- Rust 1.5


## Requirements

- Elixir 1.2.0 or newer is recommended.
- Erlang/OTP 17.0 or newer. Note Elixir 1.2.0 requires OTP 18.0 or
  newer.
  * Erlang/OTP must be built with `--enable-dirty-schedulers`
  * Optionally, Erlang/OTP can be build with `--enable-hipe` and
    `--enable-native-libs`
- Rust 1.5 or newer is recommended.


## Running the Program

Clone and build the project. This will build both Rust and Elixir
stuffs.

```shell-session
git clone https://github.com/tatsuya6502/elixir_rust_interop_demo.git
cd elixir_rust_interop_demo
iex -S mix
```


### Calling a Regular Elixir Function

Try to call an Elixir function to calculate an approximate number of
Pi in parallel. (Run 10 processes)

```iex
iex> :timer.tc(fn() -> Pi.calc_pi_parallel(1_000_000_000, 10) end)
{29319387, {:ok, 3.1415926265444543}}
```


### Enable HiPE

You may also try to enable HiPE. From another terminal window, compile
Pi module with hipe option.

```shell-session
cd elixir_rust_interop_demo
ERL_COMPILER_OPTIONS="[native, {hipe, [o3]}]" elixirc -o _build/dev/lib/elixir_rust_interop_demo/ebin/ lib/pi.ex
```


### Calling a Rust Function from Elixir via Erlang NIF interface

Calling a Rust function from an Elixir function to calculate an
approximate number of Pi in parallel. (Run 10 threads)

```iex
iex> :timer.tc(fn() -> PiNif.calc_pi_parallel(1_000_000_000, 10) end)
{1209160, {:ok, 3.141592655589816}}
```

###
