

defmodule SFfi.Raw do
  def init, do: :erlang.load_nif("/r/code/build/b_s_ffi/libb_s_ffi", 0)

  def priv__init_once, do: :erlang.nif_error(:not_loaded)
  def nif__hello_from_rust, do: :erlang.nif_error(:not_loaded)

  def inv_index__doc_add(_pid, _path, _doc_id, _terms), do: err()
  def inv_index__doc_del(_pid, _path, _doc_id), do: err()
  def inv_index__term_lookup(_pid, _path, _term), do: err()


  def job_queue__inc_if_equal(_pid, _path, _v), do: err()
  def job_queue__get_clock(_pid, _path), do: err()

  def kv_dir__ls(_pid, _path), do: err()
  def kv_dir__rm(_pid, _path), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)

end



