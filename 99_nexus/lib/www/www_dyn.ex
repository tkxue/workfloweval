defmodule Nexus.Www_Dyn.Plug do
  use Plug.Router


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


  get "/doc_search/all" do
    lst = DocSearch.Repo.all(DocSearch.Doc)
    out = Enum.map(lst, fn x -> Map.take(x, [:file_name, :num_pages]) end)
    json_string = Jason.encode!(out)
    conn
    |> put_resp_content_type("application/json")
    |> send_resp(200, json_string)
  end

  @valid_fname ~r/^[a-zA-Z0-9_.\-]*$/


  get "/doc_search/png/:fname" do
    fname = conn.params["fname"]
    if fname =~ @valid_fname do
      file_path = "/r/code/data/png/" <> fname
      if File.regular?(file_path) do
        conn
        |> put_resp_content_type(MIME.from_path(file_path))
        |> put_resp_header("cache-control", "public, max-age=86400")
        |> send_file(200, file_path)
      else
        send_resp(conn, 404, "Not Found")
      end
    else
      conn
      |> send_resp(:forbidden, "|#{fname}| does not match regex")
    end
  end

  get "/doc_search/json/:fname" do
    fname = conn.params["fname"]
    if fname =~ @valid_fname do
      file_path = "/r/code/data/json/" <> fname
      if File.regular?(file_path) do
        conn
        |> put_resp_content_type(MIME.from_path(file_path))
        |> put_resp_header("cache-control", "public, max-age=86400")
        |> send_file(200, file_path)
      else
        send_resp(conn, 404, "Not Found")
      end
    else
      conn
      |> send_resp(:forbidden, "|#{fname}| does not match regex")
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
