defmodule OxcEx.Native do
  use Rustler, otp_app: :oxc_ex, crate: "oxc_ex_native"

  # When your NIF is loaded, it will override this function.
  def minify(_input, _might_be_path, _compress_options), do: :erlang.nif_error(:nif_not_loaded)
end
