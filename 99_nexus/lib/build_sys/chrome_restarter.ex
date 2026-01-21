# lib/my_app/pg_server.ex
defmodule Nexus.Build_Sys.Chrome_Restarter do
  @moduledoc """
  """
  use GenServer

  def start_link(_opts) do
    GenServer.start_link(__MODULE__, :ok, name: __MODULE__)
  end

  @impl true
  def init(_opts) do
    {:ok, _} = :pg.start_link(:build_sys__chrome_restarter)
    {:ok, nil}
  end

  @spec join(pid) :: :ok | {:error, term()}
  def join(pid) when is_pid(pid) do
    # IO.puts("Buidl_Sys.Chrome_Restarter join")
    GenServer.call(__MODULE__, {:join, pid})
  end

  @spec send(term()) :: :ok
  def send(msg) do
    GenServer.call(__MODULE__, {:send, msg})
  end

  # ------------------------------------------------------------------
  # GenServer callbacks
  # ------------------------------------------------------------------

  @impl true
  def handle_call(cmd, _from, state) do
    case cmd do
      {:join, pid} ->
        result = :pg.join(:build_sys__chrome_restarter, :main, pid)
        {:reply, result, state}

      {:send, msg} ->
        for pid <- :pg.get_members(:build_sys__chrome_restarter, :main) do
          Kernel.send(pid, {:send, msg})
        end

        {:reply, :ok, state}
    end
  end
end
