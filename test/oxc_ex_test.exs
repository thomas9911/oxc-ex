defmodule OxcExTest do
  use ExUnit.Case
  doctest OxcEx

  describe "minify" do
    test "simple js" do
      js_script = """
      let x = 1;
      const a = (b) => {
        return b + x;
      }

      """

      assert {:ok, "let x=1;const a=(b)=>{return b+x}"} ==
               OxcEx.Native.minify(js_script, "test.js", OxcEx.CompressOptions.default())
    end

    test "drop_console removes console.logs" do
      js_script = __DIR__ |> Path.join("./example.js") |> File.read!()
      config = %{OxcEx.CompressOptions.default() | drop_console: true}
      assert {:ok, out} = OxcEx.Native.minify(js_script, "example.js", config)
      refute String.contains?(out, "console")
    end

    test "check all default options" do
      js_script = """
      let a = 1;
      let b = 2;
      let c = a + b;

      const FALSE = false;
      const TRUE = true;

      let sum = 0
      let i = 0;
      while (i < 10) {
        sum += i;
        i++;
      }

      if (typeof sum === "undefined") {
        console.log(sum)
      }

      """

      assert {:ok, out} =
               OxcEx.Native.minify(js_script, "example.js", OxcEx.CompressOptions.default())

      assert "let a=1,b=2,c=a+b;const FALSE=!1,TRUE=!0;let sum=0,i=0;for(;i<10;){sum+=i;i++}if(void 0===sum)console.log(sum)" ==
               out
    end

    test "invalid js" do
      js_script = """
      const a = (b) => {
        return b +
      """

      assert {:error, %{"message" => "Unexpected token"}} =
               OxcEx.Native.minify(js_script, "test.js", OxcEx.CompressOptions.default())
    end
  end
end
