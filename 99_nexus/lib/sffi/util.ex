defmodule SFfi.Util do


  def wait_resp() do
    receive do
      {:ok, result} ->
        result
      {:err, err} ->
        dbg(err)
        raise err
    after
      5000 -> IO.puts("Timeout")
    end
  end



end