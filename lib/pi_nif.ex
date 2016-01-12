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

defmodule PiNif do

  @on_load   :init

  @app       :pi
  @mod       PiNif
  @lib_name  'pi_nif'  # char list

  @spec calc_pi(n :: non_neg_integer) :: {:ok, pi :: float} | no_return
  def calc_pi(_n) do
    :erlang.nif_error({:nif_not_loaded, @mod})
  end

  @spec calc_pi_parallel(n :: non_neg_integer,
                         num_threads :: non_neg_integer)
                        :: {:ok, pi :: float} | {:error, term()} | no_return
  def calc_pi_parallel(_n, _num_threads) do
    :erlang.nif_error({:nif_not_loaded, @mod})
  end

  def init() do
    priv_dir = case :code.priv_dir(@app) do
                 dir when is_list(dir) ->
                   dir
                 {:error, :bad_name} ->
                   case :code.which(@mod) do
                     :bad_name ->
                       './priv'
                     :non_existing ->
                       './priv'
                     dir when is_list(dir) ->
                       :filename.join([:filename.dirname(dir), '../priv'])
                   end
               end
    so_name = :filename.join(priv_dir, 'lib' ++ @lib_name)
    :erlang.load_nif(so_name, 0)
  end

end
