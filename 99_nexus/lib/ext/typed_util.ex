defmodule Ext.Typed_Util do
  def ast_to_name(ast) do
    Macro.to_string(ast) |> String.split(".") |> List.last()
  end

  def atom_to_rust_type(a) do
    case a do
      :p_i64 -> "i64"
      :p_i32 -> "i32"
      :p_i16 -> "i16"
      :p_i8 -> "i8"
      :p_u64 -> "u64"
      :p_u32 -> "u32"
      :p_u16 -> "u16"
      :p_u8 -> "u8"
      :p_f32 -> "OrderedFloat<f32>"
      :p_f64 -> "OrderedFloat<f64>"
      :p_binary -> "Vec<u8>"
      :p_string -> "String"
      :p_nil -> "()"
      :p_any -> "()"
      :p_pid -> "()"
    end
  end

  def tsig_to_rust_name(ast) do
    case ast do
      x when is_atom(x) -> Ext.Typed_Util.atom_to_rust_type(x)
      {:vec, x} -> "Vec<" <> tsig_to_rust_name(x) <> ">"
      _ -> Macro.to_string(ast) |> String.split(".") |> List.last()
    end
  end

  def atom_to_str(atom) do
    Atom.to_string(atom)
  end

  def fields_to_str(fields, sep) do
    flist = fields

    parts =
      for {name, tsig} <- flist do
        name = Macro.to_string(name) |> String.replace(":", "")
        tsig = Ext.Typed_Util.tsig_to_rust_name(tsig)
        "#{name}: #{tsig} "
      end
      |> Enum.join(sep)

    parts
  end

  def fields_to_str_pub(fields, sep) do
    flist = fields

    parts =
      for {name, tsig} <- flist do
        name = Macro.to_string(name) |> String.replace(":", "")
        tsig = Ext.Typed_Util.tsig_to_rust_name(tsig)
        "pub #{name}: #{tsig} "
      end
      |> Enum.join(sep)

    parts
  end
end
