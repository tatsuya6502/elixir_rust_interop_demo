# -*- coding:utf-8-unix -*-

## The MIT License
##
## Copyright (C) 2016 by Tatsuya Kawano <tatsuya@hibaridb.org>
##
## Permission is hereby granted, free of charge, to any person obtaining a copy
## of this software and associated documentation files (the "Software"), to deal
## in the Software without restriction, including without limitation the rights
## to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
## copies of the Software, and to permit persons to whom the Software is
## furnished to do so, subject to the following conditions:
##
## The above copyright notice and this permission notice shall be included in
## all copies or substantial portions of the Software.
##
## THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
## IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
## FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
## AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
## LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
## OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
## THE SOFTWARE.

defmodule Pi do

  @max_procs  1024
  @timeout    60_000   # 1 minute

  @spec calc_pi(n :: non_neg_integer) :: {:ok, pi :: float}
  def calc_pi(n) do
    w = 1.0 / n
    s0 = 0.0
    # s1 = Enum.reduce(0..(n - 1), s0, fn(i, s) ->
    s1 = for_each(0, n, s0, fn(i, s) ->
      x = i * w
      s + :math.sqrt(1.0 - x * x)
    end)
    {:ok, 4.0 * w * s1}
  end

  @spec calc_pi_parallel(n :: non_neg_integer,
                         num_procs :: non_neg_integer) :: {:ok, pi :: float}
                                                           | {:error, term()}
  # num_process の値をチェックし、範囲外ならエラーを返す。
  def calc_pi_parallel(_n, num_procs) when num_procs <= 0 or num_procs > @max_procs do
    {:error,
     'Invalid num_procs #{num_procs}. It must be > 0 and <= #{@max_procs}'}
  end
  def calc_pi_parallel(n, num_procs) when rem(n, num_procs) != 0 do
    {:error, 'n #{n} must be a multiple of num_procs #{num_procs}'}
  end

  # num_process の値が範囲内なので、calc_pi_range/3 を parallel に実行する。
  def calc_pi_parallel(n, num_procs) do
    len = div(n, num_procs)
    pi = 0..(num_procs - 1)
      |> Enum.map(&(Task.async(fn() -> Pi.calc_pi_range(n, len * &1, len) end)))
      |> Enum.map(&(Task.await(&1, @timeout)))
      |> Enum.sum
    {:ok, pi}
  end

  @spec calc_pi_range(n :: non_neg_integer,
                      offset :: non_neg_integer,
                      count :: non_neg_integer) :: pi :: float
  def calc_pi_range(n, offset, count) do
    w = 1.0 / n
    s0 = 0.0
    s1 = for_each(offset, offset + count, s0, fn(i, s) ->
      x = i * w
      s + :math.sqrt(1.0 - x * x)
    end)
    4.0 * w * s1
  end

  @spec for_each(index :: integer,
                 max :: integer,
                 init_acc :: term,
                 ((i :: integer, acc0 :: term) -> acc1 :: term))
                :: final_acc :: term
  defp for_each(max, max, acc, _fun) do
    acc
  end
  defp for_each(i, max, acc, fun) do
    for_each(i + 1, max, fun.(i, acc), fun)
  end

end
