defmodule OxcEx.CompressOptions do
  defstruct booleans: true,
            drop_debugger: true,
            drop_console: false,
            evaluate: true,
            join_vars: true,
            loops: true,
            typeofs: true

  def default do
    %OxcEx.CompressOptions{}
  end
end
