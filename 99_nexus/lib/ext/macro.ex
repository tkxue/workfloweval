defmodule Ext.Macro do
  defmacro def_big_enum(name, arms) do
    Ext.Big_Enum.fn_big_enum(name, arms)
  end

  defmacro def_typed_enum(name, attrs, arms) do
    Ext.Typed_Enum.fn_typed_enum(name, attrs, arms)
  end

  defmacro def_typed_struct(name, attrs, fields) do
    Ext.Typed_Struct.fn_typed_struct(name, attrs, fields, -1, true)
  end
end
