defmodule Nexus.Build_Sys.Run_Proc do
  @moduledoc """
  Runs external processes with timeout and collects output.
  """

  import Ext.Macro

  def_typed_enum(D_Run_Proc_Status, "", [
    {A__Time_Out, [job_id: :p_any, msg: :p_binary]},
    {A__Fail, [job_id: :p_any, status: :p_i64, msg: :p_binary]},
    {A__Success, [job_id: :p_any, msg: :p_binary]},
    {A__Running, [job_id: :p_any]}
  ])

  def collect_output(job_id, port, acc) do
    receive do
      {^port, {:data, data}} ->
        collect_output(job_id, port, [data | acc])

      {^port, :eof} ->
        collect_output(job_id, port, acc)

      {^port, {:exit_status, status}} ->
        close_port_safely(port)

        if status == 0 do
          %D_Run_Proc_Status.A__Success{
            job_id: job_id,
            msg: :erlang.iolist_to_binary(:lists.reverse(acc))
          }
        else
          %D_Run_Proc_Status.A__Fail{
            job_id: job_id,
            status: status,
            msg: :erlang.iolist_to_binary(:lists.reverse(acc))
          }
        end

      {:timeout, ^port} ->
        close_port_safely(port)

        %D_Run_Proc_Status.A__Time_Out{
          job_id: job_id,
          msg: :erlang.iolist_to_binary(:lists.reverse(acc))
        }


    end
  end

  defp close_port_safely(port) do
    try do
      Port.close(port)
    rescue
      _ -> :ok
    end
  end

  def kill_proc(timeout, port) do
    Process.send_after(self(), :kill_all, timeout)

    receive do
      :kill_all ->
        close_port_safely(port)
    end
  end

  def run_proc(ret_pid, job_id, timeout, cmd, dir) do
    spawn(fn ->
      send(ret_pid, %D_Run_Proc_Status.A__Running{job_id: job_id})

      port =
        Port.open({:spawn, cmd}, [
          :binary,
          :exit_status,
          :eof,
          {:cd, dir}
        ])

      spawn(fn -> kill_proc(timeout + 1000, port) end)
      Process.send_after(self(), {:timeout, port}, timeout)
      shell_output = collect_output(job_id, port, [])
      send(ret_pid, shell_output)
    end)
  end
end
