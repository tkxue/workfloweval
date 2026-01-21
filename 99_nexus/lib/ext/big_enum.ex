defmodule Ext.Big_Enum do
  def build_rust_big_enum(name, arms) do
    enum_name = Macro.to_string(name) |> String.split(".") |> List.last()

    arms =
      for arm <- arms,
          do:
            Macro.to_string(arm)
            |> String.split(".")
            |> List.last()
            |> String.split(":")
            |> List.last()

    arms = Enum.join(arms, ",\n")

    "
    #[repr(u16)]
    #[derive(PartialEq, Hash, Copy, Clone, PartialOrd, Eq, Ord, BigEnum )]
    pub enum #{enum_name} {
      #{arms}
    }

    impl T_BigEnum for #{enum_name} {}
"
  end

  def fn_big_enum(name, arms) do
    to_u16 = Map.new(Enum.with_index(arms))
    from_u16 = Map.new(Enum.with_index(arms), fn {k, v} -> {v, k} end)

    quote do
      defmodule unquote(name) do
        @__to_u16 unquote(Macro.escape(to_u16))
        @__from_u16 unquote(Macro.escape(from_u16))

        def get__to_u16(), do: @__to_u16
        def get__from_u16(), do: @__from_u16

        def to_u16(x) do
          get__to_u16()[x]
        end

        def from_u16(x) do
          get__from_u16()[x]
        end

        def to_io_list(x) do
          <<to_u16(x)::little-16>>
        end

        def from_binary(io_list) do
          <<x::little-16, rest::binary>> = io_list
          {get__from_u16()[x], rest}
        end

        def __rust_type_info() do
          unquote(build_rust_big_enum(name, arms))
        end
      end
    end
  end
end
