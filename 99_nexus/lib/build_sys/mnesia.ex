defmodule Nexus.Build_Sys.Table_Aux do
  @moduledoc "Mnesia-backed employee"
  defstruct [:key, :value]

  # ---- Mnesia metadata -------------------------------------------------
  def __mnesia_table__, do: :build_sys__aux
  def __mnesia_key__, do: :key
  def __mnesia_attributes__, do: [:key, :value]
  # --------------------------------------------------------------------
end

defmodule Nexus.Build_Sys.Table_Rust do
  @moduledoc "Mnesia-backed employee"
  defstruct [:build_name, :version, :status, :msg]

  # ---- Mnesia metadata -------------------------------------------------
  def __mnesia_table__, do: :build_sys__rust
  def __mnesia_key__, do: :build_name
  def __mnesia_attributes__, do: [:build_name, :version, :status, :msg]
  # --------------------------------------------------------------------
end

defmodule Nexus.Build_Sys.MnesiaSchema do
  @tables [
    # Company.Employee,
    # Company.Dept,
    # Company.Project,
    # Company.Manager,
    # Company.AtDep,
    # Company.InProj,
    Nexus.Build_Sys.Table_Rust,
    Nexus.Build_Sys.Table_Aux
  ]

  def init do
    :mnesia.start()
    Enum.each(@tables, &create_table/1)
  end

  defp create_table(mod) do
    table = mod.__mnesia_table__()
    attrs = mod.__mnesia_attributes__()
    type = :set

    opts = [
      attributes: attrs,
      type: type,
      # change to ram_copies if you prefer
      ram_copies: [node()]
      # disc_copies: [node()]   # change to ram_copies if you prefer
    ]

    case :mnesia.create_table(table, opts) do
      {:atomic, :ok} -> IO.puts("created #{table}")
      {:aborted, {:already_exists, ^table}} -> IO.puts("exists  #{table}")
      {:aborted, reason} -> IO.puts("failed #{table}: #{inspect(reason)}")
    end
  end
end

defmodule MyDB.MnesiaSetup do
  @moduledoc """
  A GenServer to initialize and configure Mnesia.
  """
  use GenServer

  def start_link(_) do
    GenServer.start_link(__MODULE__, [], name: __MODULE__)
  end

  @impl true
  def init(_) do
    :ok = ensure_mnesia_dir()
    :ok = :mnesia.start()
    create_schema()
    create_table()

    {:ok, %{}}
  end

  defp ensure_mnesia_dir do
    mnesia_dir = Application.get_env(:mnesia, :dir, "mnesia_data")
    File.mkdir_p!(mnesia_dir)
    Application.put_env(:mnesia, :dir, mnesia_dir)
    :ok
  end

  defp create_schema do
    case :mnesia.create_schema([node()]) do
      :ok -> :ok
      {:error, {_, {:already_exists, _}}} -> :ok
      error -> raise "Failed to create Mnesia schema: #{inspect(error)}"
    end
  end

  defp create_table do
    :mnesia.create_table(:build_status, attributes: [:build_id, :tick, :status, :msg])
    |> handle_table_creation()
  end

  defp handle_table_creation(result) do
    case result do
      :ok -> :ok
      {:atomic, :ok} -> :ok
      {:aborted, {:already_exists, _}} -> :ok
      error -> raise "Failed to create Mnesia table: #{inspect(error)}"
    end
  end
end

# :mnesia.create_table(:foo, [] )
# |> handle_table_creation()

#    :mnesia.create_table(:example_table,
#  attributes: [:id, :data],
#  disc_copies: [node()]
# )
# |> handle_table_creation()
