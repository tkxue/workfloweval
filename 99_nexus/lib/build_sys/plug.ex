defmodule Nexus.Build_Sys.Ws_Handler do
  def init(_args) do
    Nexus.Build_Sys.Chrome_Restarter.join(self())
    {:ok, []}
  end

  def handle_in(cmd, state) do

    case cmd do
      {"{\"type\":\"ping\"}", [opcode: :text]} -> {:reply, :ok, {:text, "pong"}, state}
      _ ->
        dbg(cmd)
        {:reply, :ok, {:text, "pong"}, state}
    end

    {:reply, :ok, {:text, "pong"}, state}
  end

  def handle_info(cmd, state) do
    case cmd do
      {:send, msg} ->
        dbg({:send, msg})
        {:push, [{:text, msg}], state}
      _ ->
        dbg({:unknown, cmd})
        {:ok, state}
    end
  end

  def get_all_modified() do
    # modified =
    #  for {mod, path} <- :code.all_loaded(),
    #      String.starts_with?(Atom.to_string(mod), "Elixir.Nexus"),
    #      is_list(path) and Enum.all?(path, &is_integer/1),
    #      path = List.to_string(path),
    #      {:ok, %File.Stat{mtime: mtime}} <- [File.stat(path)],
    #      Code.ensure_loaded?(mod),
    #      # load_time = mod.__info__(:compile)[:time],
    #      # load_time = :erlang.module_info(mod)[:compile][:time],
    #      # mtime > load_time,
    #      do: mod
    # dbg( for x <- modified, do: Atom.to_string(x) )
    # dbg( for x <- modified, do: :code.module_status(x)  )
    # modified
  end
end

defmodule Nexus.Build_Sys.Plug do
  use Plug.Router

  plug(:match)
  plug(:dispatch)

  get "/ws" do
    conn
    |> WebSockAdapter.upgrade(Nexus.Build_Sys.Ws_Handler, [], timeout: 60_000)
    |> halt()
  end

  get "/" do
    file_path = "/r/www/index.html"

    if File.regular?(file_path) do
      conn
      |> put_resp_content_type(MIME.from_path(file_path))
      |> put_resp_header("cache-control", "public, max-age=86400")
      |> send_file(200, file_path)
    else
      # Logger.warning("Root file not found: #{file_path}")
      send_resp(conn, 404, "Not Found")
    end
  end

  get "/nexus/restart_server" do
    IO.puts("restarting server")
    System.halt()

    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(200, "firing off restart_server")
  end

  get "/nexus/rebuild_all" do
    Nexus.Build_Sys.Rust.rebuild_all()

    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(200, "firing off rebuild all")
  end

  get "/nexus/rebuild_s_http" do
    Nexus.Build_Sys.Rust.rebuild_s_http()

    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(200, "firing off rebuild server")
  end

  get "/nexus/rebuild_s_ffi" do
    Nexus.Build_Sys.Rust.rebuild_s_ffi()

    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(200, "firing off rebuild ffi")
  end


  get "/nexus/rebuild_logic" do
    Nexus.Build_Sys.Rust.rebuild_logic()

    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(200, "firing off rebuild logic")
  end

  get "/nexus/regen_types" do
    IO.puts("regen types")
    Nexus.Build_Sys.Rust.regen_types()

    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(200, "firing off regen types")
  end

  get "/nexus/reload_ex" do
    IO.puts("reloading elixir")
    spawn(fn -> IEx.Helpers.recompile() end)

    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(200, "reloading elixir")
  end

  get "/goto-file/:line/:fname" do
    fname_regex = ~r/^\/r\/code\/[a-zA-Z_\/0-9]*\.rs$/
    line_regex = ~r/^[0-9]*$/

    fname = Path.expand(Base.decode64!(conn.params["fname"]))
    line = conn.params["line"]

    fname_match = fname =~ fname_regex
    line_match = line =~ line_regex

    cond do
      fname_match && line_match ->
        dbg({:goto_file, fname, line})
        System.cmd("/home/y/.nix-profile/bin/code", ["--goto", "#{fname}:#{line}"])
        # System.cmd("/home/y/local/rr/RustRover-2025.3.1/bin/rustrover.sh", ["--line", line, fname])

      !fname_match ->
        dbg({:invalid, :goto_file, :fname, fname})

      !line_match ->
        dbg({:invalid, :goto_file, :line, line})

      true ->
        dbg("confused")
    end

    conn
    |> put_resp_content_type("text/html")
    |> send_resp(200, "<script>window.close()</script>")
  end

  match _ do
    path = conn.request_path
    t = "Not found #{path}"

    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(404, t)
  end
end
