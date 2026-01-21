defmodule Nexus do
  @moduledoc """
  Documentation for `Nexus`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Nexus.hello()
      :world

  """
  def hello do
    :world
  end

  def reload_main do
    db = Fdb.Util.db()
    FdbL.Inv_Index.doc_add(db, ["dev-resumee", "inv_index"], "john", MapSet.new(["nixos", "react", "linux"]))
    dbg(FdbL.Inv_Index.term_lookup(db, ["dev-resumee", "inv_index"], "react"))
    FdbL.Inv_Index.doc_del(db, ["dev-resumee", "inv_index"], "john")
    dbg(FdbL.Inv_Index.term_lookup(db, ["dev-resumee", "inv_index"], "react"))
  end
end
