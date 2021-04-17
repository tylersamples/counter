defmodule Counter do
  use Rustler, otp_app: :counter, crate: :counter

  defp err, do: :erlang.nif_error(:nif_not_loaded)

  def new(), do: err()

  def increment(ref), do: err()

  def decrement(ref), do: err()

  def read(ref), do: err()
end
