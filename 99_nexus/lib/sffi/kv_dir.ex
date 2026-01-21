defmodule SFfi.KVShell do

  def ls(path ) do
    SFfi.Raw.kv_dir__ls(self(), path)
    res = SFfi.Util.wait_resp()
  end

  def rm(path ) do
    SFfi.Raw.kv_dir__rm(self(), path)
    res = SFfi.Util.wait_resp()
  end

end
