defmodule Ext.Typed_Struct do
  @doc """
  ```
  """

  def code_for_write(typespec) do
    case typespec do
      :p_i64 ->
        quote do
          <<x::integer-size(64)-little>>
        end

      :p_i32 ->
        quote do
          <<x::integer-size(32)-little>>
        end

      :p_i16 ->
        quote do
          <<x::integer-size(16)-little>>
        end

      :p_i8 ->
        quote do
          <<x::integer-size(8)-little>>
        end

      :p_u64 ->
        quote do
          <<x::unsigned-integer-size(64)-little>>
        end

      :p_u32 ->
        quote do
          <<x::unsigned-integer-size(32)-little>>
        end

      :p_u16 ->
        quote do
          <<x::unsigned-integer-size(16)-little>>
        end

      :p_u8 ->
        quote do
          <<x::unsigned-integer-size(8)-little>>
        end

      :p_f32 ->
        quote do
          <<x::float-32-little>>
        end

      :p_f64 ->
        quote do
          <<x::float-64-little>>
        end

      :p_binary ->
        quote do
          [<<byte_size(x)::unsigned-integer-size(64)-little>>, x]
        end

      :p_string ->
        quote do
          [<<byte_size(x)::unsigned-integer-size(64)-little>>, x]
        end

      :p_nil ->
        quote do
          <<>>
        end

      :p_any ->
        quote do
          <<>>
        end

      :p_pid ->
        quote do
          <<>>
        end
    end
  end

  def code_for_read(typespec) do
    case typespec do
      :p_i64 ->
        quote do
          <<__x::integer-size(64)-little, __rest::binary>> = __rest
        end

      :p_i32 ->
        quote do
          <<__x::integer-size(32)-little, __rest::binary>> = __rest
        end

      :p_i16 ->
        quote do
          <<__x::integer-size(16)-little, __rest::binary>> = __rest
        end

      :p_i8 ->
        quote do
          <<__x::integer-size(8)-little, __rest::binary>> = __rest
        end

      :p_u64 ->
        quote do
          <<__x::unsigned-integer-size(64)-little, __rest::binary>> = __rest
        end

      :p_u32 ->
        quote do
          <<__x::unsigned-integer-size(32)-little, __rest::binary>> = __rest
        end

      :p_u16 ->
        quote do
          <<__x::unsigned-integer-size(16)-little, __rest::binary>> = __rest
        end

      :p_u8 ->
        quote do
          <<__x::unsigned-integer-size(8)-little, __rest::binary>> = __rest
        end

      :p_f32 ->
        quote do
          <<__x::float-32-little, __rest::binary>> = __rest
        end

      :p_f64 ->
        quote do
          <<__x::float-64-little, __rest::binary>> = __rest
        end

      :p_binary ->
        quote do
          <<__n::unsigned-integer-size(64)-little, __x::binary-size(__n), __rest::binary>> =
            __rest
        end

      :p_string ->
        quote do
          <<__n::unsigned-integer-size(64)-little, __x::binary-size(__n), __rest::binary>> =
            __rest
        end

      :p_nil ->
        quote do
          __x = 0
        end

      :p_any ->
        quote do
          __x = 0
        end

      :p_pid ->
        quote do
          __x = 0
        end
    end
  end

  def fn_typed_struct(name, attrs, fields, arm_id, is_rust_struct) do
    enforced_list = fields |> Enum.map(&elem(&1, 0))
    # field_specs = fields
    field_vals = Enum.map(fields, fn {field, _} -> field end)

    x =
      if is_rust_struct do
        quote do
          def __rust_type_info() do
            unquote(build_rust_struct(name, attrs, fields))
          end
        end
      else
        quote do
          def __arm_id(), do: unquote(Macro.escape(arm_id))
        end
      end

    to_io_list =
      for {field, typespec} <- fields do
        f =
          quote do
            x.unquote(field)
          end

        case typespec do
          ts when is_atom(ts) ->
            c = code_for_write(ts)

            quote do
              x = unquote(f)
              unquote(c)
            end

          {:vec, ts} ->
            quote do
              [
                <<length(unquote(f))::unsigned-integer-size(64)-little>>
                | for x <- unquote(f) do
                    unquote(ts).to_io_list(x)
                  end
              ]
            end

          {:__aliases__, _, _} ->
            quote do
              unquote(typespec).to_io_list(unquote(f))
            end
        end
      end

    from_binary_bind =
      for {field, typespec} <- fields do
        field_name = {field, [], Elixir}

        case typespec do
          ts when is_atom(ts) ->
            quote do
              unquote(code_for_read(ts))
              unquote(field_name) = __x
            end

          {:vec, ts} ->
            quote do
              <<n::unsigned-integer-size(64)-little, __rest::binary>> = __rest

              {__lst, __rest} =
                Enum.reduce(1..n, {[], __rest}, fn _, {cur, __rest} ->
                  {__val, __rest} = unquote(ts).from_binary(__rest)
                  {[__val | cur], __rest}
                end)

              unquote(field_name) = Enum.reverse(__lst)
            end

          {:__aliases__, _, _} ->
            quote do
              {unquote(field_name), __rest} = unquote(typespec).from_binary(__rest)
            end
        end
      end

    from_binary_build =
      for {field, _typespec} <- fields do
        field_name = {field, [], Elixir}

        quote do
          {unquote(field), unquote(field_name)}
        end
      end

    # @type t :: %__MODULE__{unquote_splicing(field_specs)}

    q =
      quote do
        defmodule unquote(name) do
          @enforce_keys unquote(enforced_list)
          defstruct unquote(field_vals)
          unquote(x)

          def to_io_list(x) do
            [unquote_splicing(to_io_list)]
          end

          def from_binary(__rest) do
            unquote_splicing(from_binary_bind)

            {
              %unquote(name){unquote_splicing(from_binary_build)},
              __rest
            }
          end
        end
      end

    {:ok, file} = File.open("/home/y/tmp/example.txt", [:append])
    IO.write(file, Macro.to_string(q))
    File.close(file)

    q
  end

  def build_rust_struct(name, attrs, fields) do
    struct_name = Ext.Typed_Util.ast_to_name(name)
    parts = Ext.Typed_Util.fields_to_str_pub(fields, ",\n")
    "
#[derive(PartialEq, Hash, Clone, PartialOrd, Eq, Ord, JsData, JsData, #{attrs})]
    pub struct #{struct_name} {
 #{parts}
 }"
  end
end
