defmodule DataFusion.Native do
  use Rustler, otp_app: :data_fusion, crate: "fusion"

  def df_from_csv(_filename), do: err()

  def df_from_parquet(_filename), do: err()

  def df_names(_df), do: err()

  def df_n_rows(_df), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end

