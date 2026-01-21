

defmodule SFfi.InvIndex do

  def term_lookup(path, term) do
    SFfi.Raw.inv_index__term_lookup(self(), path, term)
    receive do
      {:ok, result} -> result
    after
      5000 -> IO.puts("Timeout")
    end
  end

  def doc_del(path, doc_id) do
    SFfi.Raw.inv_index__doc_del(self(), path, doc_id)
    receive do
      {:ok, result} -> result
    after
      5000 -> IO.puts("Timeout")
    end
  end

  def doc_add(path, doc_id, terms) do
    SFfi.Raw.inv_index__doc_add(self(), path, doc_id, terms)
    receive do
      {:ok, result} -> result
    after
      5000 -> IO.puts("Timeout")
    end
  end


  def test() do
    SFfi.InvIndex.doc_add("resumee", "john", ["react", "java", "cpp"])
    SFfi.InvIndex.doc_add("resumee", "dan", ["react", "cljs", "scala"])
    SFfi.InvIndex.term_lookup("resumee", "cpp")

  end


end

