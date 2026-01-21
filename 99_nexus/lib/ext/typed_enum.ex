defmodule Ext.Typed_Enum do
  def build_rust_enum(name, attrs, arms) do
    enum_name = Macro.to_string(name) |> String.split(".") |> List.last()

    parts =
      for {arm_name, fields} <- arms do
        arm_name = Ext.Typed_Util.ast_to_name(arm_name)
        fields_str = Ext.Typed_Util.fields_to_str(fields, ", ")

        if Enum.count(fields) > 0 do
          "#{arm_name} { #{fields_str}  }"
        else
          "#{arm_name}"
        end
      end
      |> Enum.join(",\n")

    "
#[derive(PartialEq, Hash, Clone, PartialOrd, Eq, Ord, JsData, JsData, #{attrs})]
    pub enum #{enum_name} {
#{parts}
}"
  end

  def fn_typed_enum(name, attrs, arms) do
    code_arms =
      for {{arm_name, arm_fields}, idx} <- Enum.with_index(arms) do
        quote do
          unquote(Ext.Typed_Struct.fn_typed_struct(arm_name, attrs, arm_fields, idx, false))
        end
      end

    # [hd | tl] = Enum.reverse(for { arm_name , _ } <- arms, do: arm_name )
    # arm_names = Enum.reduce(tl, quote do unquote(hd).t end, fn lhs, acc -> quote do unquote(lhs).t | unquote(acc) end end)

    my_structs =
      for {{arm_name, _arm_fields}, idx} <- Enum.with_index(arms) do
        quote do
          {unquote(Macro.escape(idx)), unquote(arm_name)}
        end
      end

    my_structs =
      quote do
        %{
          unquote_splicing(my_structs)
        }
      end

    # @type t :: unquote(arm_names)

    q =
      quote do
        defmodule unquote(name) do
          unquote_splicing(code_arms)

          def __rust_type_info() do
            unquote(build_rust_enum(name, attrs, arms))
          end

          @__structs unquote(my_structs)

          def get_structs, do: @__structs

          def to_io_list(x) do
            [
              <<x.__struct__.__arm_id()::8>>,
              x.__struct__.to_io_list(x)
            ]
          end

          def from_binary(b) do
            <<x::8, rest::binary>> = b
            get_structs()[x].from_binary(rest)
          end
        end
      end

    # IO.puts( Macro.to_string(q) )

    {:ok, file} = File.open("/home/y/tmp/example.txt", [:append])
    IO.write(file, Macro.to_string(q))
    File.close(file)

    q
  end
end
