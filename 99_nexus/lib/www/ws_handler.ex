defmodule Nexus.Ws_Handler do
  def init(_args) do
    dbg({:mmo_ws_handler, :ws_init, self()})
    {:ok, []}
  end

  # def handle_in({"ping", [opcode: :text]}, state) do
  #  {:reply, :ok, {:text, "pong"}, state}
  # end

  def handle_in(cmd, state) do
    case cmd do
      {"ping", [opcode: :text]} ->
        {:reply, :ok, {:text, "pong"}, state}

      {"bin_msg", [opcode: :binary]} ->
        {:reply, :ok, {:text, "bin-msg"}, state}

      _ ->
        dbg({:mmo_ws_handler, :handle_in, cmd})
    end
  end

  def handle_info(msg, state) do
    dbg({:mmo_ws_handler, :handle_info, msg})
    {:reply, :ok, {:text, "pong"}, state}
  end

  def terminate(_reason, _state) do
    dbg({:ws_terminate, self()})
  end
end

defmodule Nexus.Ws.Plug do
  use Plug.Router

  plug(:match)
  plug(:dispatch)

  get "/ws" do
    dbg({:ws, :opened})

    conn
    |> WebSockAdapter.upgrade(Nexus.Ws_Handler, [], timeout: 60_000)
    |> halt()
  end

  get "/ws_kotlin" do
    dbg({:ws_kotlin, :opened})

    conn
    |> WebSockAdapter.upgrade(Nexus.Ws_Handler, [], timeout: 60_000)
    |> halt()
  end

  match _ do
    path = conn.request_path
    t = "Not found #{path}"

    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(404, t)
  end
end
