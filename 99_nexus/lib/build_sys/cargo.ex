defmodule Nexus.Build_Sys.Rust do
  use GenServer

  @type status :: :done | :waiting

  def start_link(_opts) do
    GenServer.start_link(__MODULE__, :ok, name: __MODULE__)
  end

  def rebuild_all() do
    GenServer.call(__MODULE__, :rebuild_all)
  end

  def rebuild_s_http() do
    GenServer.call(__MODULE__, :rebuild_s_http)
  end

  def rebuild_s_ffi() do
    GenServer.call(__MODULE__, :rebuild_s_ffi)
  end


  def regen_types() do
    GenServer.call(__MODULE__, :regen_types)
  end

  def tick() do
    GenServer.call(__MODULE__, :tick)
  end

  def init(:ok) do
    # Initial state
    {:ok, :done}
  end

  def handle_call(cmd, _from, state) do
    case cmd do
      :regen_types ->
        IEx.Helpers.recompile()
        Mmo.Build_Sys.Rust_Impl.rebuild(:regen_types)
        Process.send_after(self(), :tick_regen_types, 0)
      :rebuild_all ->
        Nexus.Build_Sys.Rust_Impl.rebuild(:html_gfx)
        Nexus.Build_Sys.Rust_Impl.rebuild(:html_vid)
        Nexus.Build_Sys.Rust_Impl.rebuild(:ww_logic)
        Nexus.Build_Sys.Rust_Impl.rebuild(:ww_sheet)
        Nexus.Build_Sys.Rust_Impl.rebuild(:ww_rune)
        Nexus.Build_Sys.Rust_Impl.rebuild(:ww_python)
        Nexus.Build_Sys.Rust_Impl.rebuild(:ww_sqlite)
        Process.send_after(self(), :tick_all, 0)
      :rebuild_s_ffi ->
        Nexus.Build_Sys.Rust_Impl.rebuild(:s_ffi)
        Process.send_after(self(), :tick_s_ffi, 0)
      :rebuild_s_http ->
        Nexus.Build_Sys.Rust_Impl.rebuild(:s_http)
        Process.send_after(self(), :tick_s_http, 0)
    end

    {:reply, state, state}
  end

  def handle_info(msg, state) do
    alias Nexus.Build_Sys.Run_Proc.D_Run_Proc_Status

    html_gfx = Nexus.Build_Sys.Rust_Impl.get_status(:html_gfx)
    html_vid = Nexus.Build_Sys.Rust_Impl.get_status(:html_vid)
    s_ffi = Nexus.Build_Sys.Rust_Impl.get_status(:s_ffi)
    s_http = Nexus.Build_Sys.Rust_Impl.get_status(:s_http)
    ww_logic = Nexus.Build_Sys.Rust_Impl.get_status(:ww_logic)
    ww_sheet = Nexus.Build_Sys.Rust_Impl.get_status(:ww_sheet)
    ww_rune = Nexus.Build_Sys.Rust_Impl.get_status(:ww_rune)
    ww_python = Nexus.Build_Sys.Rust_Impl.get_status(:ww_python)
    ww_sqlite = Nexus.Build_Sys.Rust_Impl.get_status(:ww_sqlite)
    rename_sh = Nexus.Build_Sys.Rust_Impl.get_status(:rename_sh)

    case msg do



      :tick_rename_sh ->
        cond do
          is_struct(rename_sh, D_Run_Proc_Status.A__Success) ->
            IO.puts("rename.sh success")
            Nexus.Build_Sys.Chrome_Restarter.send("restart-all")

          is_struct(rename_sh, D_Run_Proc_Status.A__Fail)    -> IO.puts("rename.sh fail")
          true ->
            Process.send_after(self(), :tick_rename_sh, 100)
        end


      :tick_all ->
        dbg(
          {:tick_all,
            %{
              html_gfx: html_gfx.__struct__,
              html_vid: html_vid.__struct__,
              ww_logic: ww_logic.__struct__,
              ww_sheet: ww_sheet.__struct__,
              ww_rune: ww_rune.__struct__,
              ww_python: ww_python.__struct__,
              ww_sqlite: ww_sqlite.__struct__,
          }}
        )

        cond do
          is_struct(html_gfx, D_Run_Proc_Status.A__Success) &&
          is_struct(html_vid, D_Run_Proc_Status.A__Success) &&
            is_struct(ww_logic, D_Run_Proc_Status.A__Success) &&
            is_struct(ww_sheet, D_Run_Proc_Status.A__Success) &&
            is_struct(ww_python, D_Run_Proc_Status.A__Success) &&
            is_struct(ww_sqlite, D_Run_Proc_Status.A__Success) &&
            is_struct(ww_rune, D_Run_Proc_Status.A__Success) ->
            IO.puts("exec rename.sh")
            Nexus.Build_Sys.Rust_Impl.start_build(:rename_sh, 5 * 60 * 1000, "./rename.sh", "/r/code/build/")
            Process.send_after(self(), :tick_rename_sh, 100)

          is_struct(html_gfx, D_Run_Proc_Status.A__Fail) -> IO.puts(html_gfx.msg)
          is_struct(html_vid, D_Run_Proc_Status.A__Fail) -> IO.puts(html_vid.msg)
          is_struct(ww_logic, D_Run_Proc_Status.A__Fail) -> IO.puts(ww_logic.msg)
          is_struct(ww_sheet, D_Run_Proc_Status.A__Fail) -> IO.puts(ww_sheet.msg)
          is_struct(ww_rune, D_Run_Proc_Status.A__Fail) -> IO.puts(ww_rune.msg)
          is_struct(ww_python, D_Run_Proc_Status.A__Fail) -> IO.puts(ww_python.msg)
          is_struct(ww_sqlite, D_Run_Proc_Status.A__Fail) -> IO.puts(ww_sqlite.msg)

          true ->
            Process.send_after(self(), :tick_all, 100)
        end

      :tick_s_ffi ->
        dbg({:tick_s_ffi, s_ffi.__struct__})
        cond do
          is_struct(s_ffi, D_Run_Proc_Status.A__Success) -> IO.puts("build s_ffi success")
          is_struct(s_ffi, D_Run_Proc_Status.A__Fail) -> IO.puts(s_ffi.msg)
          true ->
            Process.send_after(self(), :tick_s_ffi, 100)
        end

      :tick_s_http ->
        dbg({:tick_s_http, s_http.__struct__})
        cond do
          is_struct(s_http, D_Run_Proc_Status.A__Success) -> IO.puts("build s_http success")
          is_struct(s_http, D_Run_Proc_Status.A__Fail) -> IO.puts(s_http.msg)
          true ->
            Process.send_after(self(), :tick_s_http, 100)
        end

    end

    {:noreply, state}
  end
end

defmodule Nexus.Build_Sys.Rust_Impl do
  import Ext.Macro
  use GenServer

  def_typed_enum(D_Rust_Compile_Status, "", [
    {Empty, []},
    {Building, [pid: :p_pid]},
    {Success, [msg: :p_binary]},
    {Fail, [msg: :p_binary]}
  ])

  def_typed_struct(Rust_Compile_Full_Status, "",
    html_gfx: D_Rust_Compile_Status,
    html_vid: D_Rust_Compile_Status,
    s_ffi: D_Rust_Compile_Status,
    s_http: D_Rust_Compile_Status,
    ww_logic: D_Rust_Compile_Status,
    ww_sheet: D_Rust_Compile_Status,
    ww_rune: D_Rust_Compile_Status,
    ww_python: D_Rust_Compile_Status,
    ww_sqlite: D_Rust_Compile_Status,
    rename_sh: D_Rust_Compile_Status
  )

  @type build_target :: :html_gfx | :html_vid | :ww_logic | :ww_sheet | :ww_rune | :ww_python | :ww_sqlite |  :rename_sh | :s_ffi | :s_http

  defguardp valid_target(x) when x in [:html_gfx, :html_vid, :ww_sheet, :ww_logic, :ww_rune, :ww_python, :ww_sqlite, :rename_sh, :s_ffi, :s_http]

  @spec rebuild(build_target) :: term()
  def rebuild(target) when valid_target(target) do
    GenServer.call(__MODULE__, {:build, target})
  end

  @spec get_status(term()) :: D_Rust_Compile_Status.t()
  def get_status(target) when valid_target(target) do
    GenServer.call(__MODULE__, {:get_status, target})
  end

  @spec set_status(build_target, D_Rust_Compile_Status.t()) :: term()
  def set_status(target, build_status) when valid_target(target) do
    GenServer.call(__MODULE__, {:set_status, target, build_status})
  end

  def start_link(_opts) do
    GenServer.start_link(__MODULE__, :ok, name: __MODULE__)
  end

  def get_state do
    GenServer.call(__MODULE__, :get_state)
  end

  # Server Callbacks
  def init(:ok) do
    {:ok,
     %Rust_Compile_Full_Status{
       html_gfx: %D_Rust_Compile_Status.Empty{},
       html_vid: %D_Rust_Compile_Status.Empty{},
       s_ffi: %D_Rust_Compile_Status.Empty{},
       s_http: %D_Rust_Compile_Status.Empty{},
       ww_logic: %D_Rust_Compile_Status.Empty{},
       ww_sheet: %D_Rust_Compile_Status.Empty{},
       ww_rune: %D_Rust_Compile_Status.Empty{},
       ww_python: %D_Rust_Compile_Status.Empty{},
       ww_sqlite: %D_Rust_Compile_Status.Empty{},
       rename_sh: %D_Rust_Compile_Status.Empty{},
     }}
  end

  def start_build(job_id, timeout, cmd, dir) do
    spawn(fn ->
      Nexus.Build_Sys.Run_Proc.run_proc(self(), job_id, timeout, cmd, dir)

      receive do
        x ->
          Nexus.Build_Sys.Rust_Impl.set_status(job_id, x)
      end

      receive do
        x ->
          Nexus.Build_Sys.Rust_Impl.set_status(job_id, x)
      end
    end)
  end

  def handle_call(cmd, _from, state) do
    case {cmd, state} do
      {{:get_status, :html_gfx}, full_status} -> {:reply, full_status.html_gfx, state}
      {{:get_status, :html_vid}, full_status} -> {:reply, full_status.html_vid, state}
      {{:get_status, :s_ffi}, full_status} -> {:reply, full_status.s_ffi, state}
      {{:get_status, :s_http}, full_status} -> {:reply, full_status.s_http, state}
      {{:get_status, :ww_logic}, full_status} -> {:reply, full_status.ww_logic, state}
      {{:get_status, :ww_sheet}, full_status} -> {:reply, full_status.ww_sheet, state}
      {{:get_status, :ww_rune}, full_status} -> {:reply, full_status.ww_rune, state}
      {{:get_status, :ww_python}, full_status} -> {:reply, full_status.ww_python, state}
      {{:get_status, :ww_sqlite}, full_status} -> {:reply, full_status.ww_sqlite, state}
      {{:get_status, :rename_sh}, full_status} -> {:reply, full_status.rename_sh, state}

      {{:set_status, :html_gfx, s}, full_status} -> {:reply, nil, %{full_status | html_gfx: s}}
      {{:set_status, :html_vid, s}, full_status} -> {:reply, nil, %{full_status | html_vid: s}}
      {{:set_status, :s_ffi, s}, full_status} -> {:reply, nil, %{full_status | s_ffi: s}}
      {{:set_status, :s_http, s}, full_status} -> {:reply, nil, %{full_status | s_http: s}}
      {{:set_status, :ww_logic, s}, full_status} -> {:reply, nil, %{full_status | ww_logic: s}}
      {{:set_status, :ww_sheet, s}, full_status} -> {:reply, nil, %{full_status | ww_sheet: s}}
      {{:set_status, :ww_rune, s}, full_status} -> {:reply, nil, %{full_status | ww_rune: s}}
      {{:set_status, :ww_python, s}, full_status} -> {:reply, nil, %{full_status | ww_python: s}}
      {{:set_status, :ww_sqlite, s}, full_status} -> {:reply, nil, %{full_status | ww_sqlite: s}}
      {{:set_status, :rename_sh, s}, full_status} -> {:reply, nil, %{full_status | rename_sh: s}}


      {{:build, :html_gfx}, _full_status} ->
        start_build(:html_gfx, 5 * 60 * 1000, "make prod 2>&1", "/r/code/build/b_html_gfx")
        {:reply, :ok, state}

      {{:build, :html_vid}, _full_status} ->
        start_build(:html_vid, 5 * 60 * 1000, "make prod 2>&1", "/r/code/build/b_html_vid")
        {:reply, :ok, state}

      {{:build, :s_ffi}, _full_status} ->
        start_build(:s_ffi, 5 * 60 * 1000, "make prod 2>&1", "/r/code/build/b_s_ffi")
        {:reply, :ok, state}

      {{:build, :s_http}, _full_status} ->
        start_build(:s_http, 5 * 60 * 1000, "make prod 2>&1", "/r/code/build/b_s_http")
        {:reply, :ok, state}


      {{:build, :ww_logic}, _full_status} ->
        start_build(:ww_logic, 5 * 60 * 1000, "make prod 2>&1", "/r/code/build/b_ww_logic")
        {:reply, :ok, state}

      {{:build, :ww_sheet}, _full_status} ->
        start_build(:ww_sheet, 5 * 60 * 1000, "make prod 2>&1", "/r/code/build/b_ww_sheet")
        {:reply, :ok, state}

      {{:build, :ww_rune}, _full_status} ->
        start_build(:ww_rune, 5 * 60 * 1000, "make prod 2>&1", "/r/code/build/b_ww_rune")
        {:reply, :ok, state}


      {{:build, :ww_python}, _full_status} ->
        start_build(:ww_python, 5 * 60 * 1000, "make prod 2>&1", "/r/code/build/b_ww_python")
        {:reply, :ok, state}

      {{:build, :ww_sqlite}, _full_status} ->
        start_build(:ww_sqlite, 5 * 60 * 1000, "make prod 2>&1", "/r/code/build/b_ww_sqlite")
        {:reply, :ok, state}


      _ ->
        dbg(cmd)
        {:reply, nil, state}
    end
  end

  def handle_cast({:update, new_value}, _state) do
    {:noreply, %{value: new_value}}
  end
end
