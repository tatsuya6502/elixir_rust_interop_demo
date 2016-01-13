# Elixir Rust Interop Demo

This project demonstrates calling Rust functions from Elixir using
Erlang NIF interface. It depends on
[ruster_unsafe](https://crates.io/crates/ruster_unsafe/) crate,
a low level bindings to Erlang NIF API for Rust.


## Recently Tested With

- Elixir 1.2.0 on Erlang/OTP 18.2.1 with dirty schedulers and HiPE
  enabled
- Rust 1.5


## Requirements

- Elixir 1.2.0 or newer is recommended.
- Erlang/OTP 17.0 or newer. 18.0 or newer is recommended because
  Elixir 1.2.0 requires 18.0.
- Rust 1.5 or newer is recommended.


### Erlang/OTP Build Options

These Rust functions take advantage of experimental "dirty schedulers"
in Erlang/OTP 17.0 or newer, so please enable this feature.

- Erlang/OTP must be built with `--enable-dirty-schedulers`
- Optionally, Erlang/OTP can be build with `--enable-hipe`, or
  with both `--enable-hipe --enable-native-libs`


## Running the Program

First of all, clone and build the project. This will build both Rust
and Elixir stuffs.

```shell-session
git clone https://github.com/tatsuya6502/elixir_rust_interop_demo.git
cd elixir_rust_interop_demo
iex -S mix
```


### Calling a Regular Elixir Function

Try to call an Elixir function to calculate an approximate number of
Pi in parallel (10 concurrent processes).

```iex
iex> :timer.tc(fn() -> Pi.calc_pi_parallel(1_000_000_000, 10) end)
{29319387, {:ok, 3.141592655589816}}
```


### Enable HiPE

You may also try HiPE. From another terminal window, compile Pi module
with hipe option.

```shell-session
cd elixir_rust_interop_demo
ERL_COMPILER_OPTIONS="[native, {hipe, [o3]}]" elixirc -o _build/dev/lib/elixir_rust_interop_demo/ebin/ lib/pi.ex
```

Then run it.

```iex
iex> l Pi
iex> :code.is_module_native(Pi)
true
iex> :timer.tc(fn() -> Pi.calc_pi_parallel(1_000_000_000, 10) end)
{15226457, {:ok, 3.141592655589816}}
```


### Calling a Rust Function from Elixir via Erlang NIF interface

Calling a Rust function from an Elixir function to calculate an
approximate number of Pi in parallel (10 concurrent threads).

```iex
iex> :timer.tc(fn() -> PiNif.calc_pi_parallel(1_000_000_000, 10) end)
{1209160, {:ok, 3.141592655589816}}
```

