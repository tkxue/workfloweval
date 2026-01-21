defmodule Nexus.Www_Static.Plug do
  use Plug.Router

  plug(Plug.Static,
    at: "/",
    from: "/r/www/"
  )

  plug(:match)
  plug(:dispatch)

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

  post "/e/esi_browser_log" do
    # IO.puts("log event")
    # spawn(fn -> IEx.Helpers.recompile() end)
    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(200, "reloading elixir")
  end

  match _ do
    path = conn.request_path
    t = "Not found #{path}"

    conn
    |> put_resp_content_type("text/plain")
    |> send_resp(404, t)
  end
end
