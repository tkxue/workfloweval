defmodule Nexus.Application do
  @moduledoc false
  use Application

  @impl true
  def start(_type, _args) do

    # Fdb.Util.init()
    # SFfi.Raw.init()
    # SFfi.Raw.priv__init_once()

    children = [
      {Bandit, plug: Nexus.Www_Static.Plug, port: 3000, ip: :loopback},
      {Bandit, plug: Nexus.Www_Static.Plug, port: 3001, ip: :loopback},
      {Bandit, plug: Nexus.Www_Static.Plug, port: 3002, ip: :loopback},
      {Bandit, plug: Nexus.Ws.Plug, port: 3010},
      {Bandit, plug: Nexus.Build_Sys.Plug, port: 3020, ip: :loopback},


      {Bandit, plug: Nexus.Www_Dyn.Plug, port: 3050, ip: :loopback},

      {Nexus.Build_Sys.Rust, []},
      {Nexus.Build_Sys.Rust_Impl, []},
      {Nexus.Build_Sys.Chrome_Restarter, []},
      # NFdb.Repo
      # Nwet.Repo,
      # DocSearch.Repo
      # { Mmo.Rust_Bridge, [] },
      # { Mmo.Rust_Logger, [] },
      # { Mmo.Rust_Status, [] },
      # { Mmo.Ets.Manager, [] },
      # { Mmo.Ets.Loc__Earth_Tile, [] },
      # { Mmo.Ets.Room__Id_Avatar, [] },
      # { Mmo.Ets.Id_Avatar__Avatar, [] },
      # { Mmo.Server, [] },
    ]

    opts = [strategy: :one_for_one, name: Nexus.Supervisor, max_restarts: 10, max_seconds: 5]
    Supervisor.start_link(children, opts)
  end
end
