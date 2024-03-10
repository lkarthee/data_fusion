defmodule DataFusion.DataFrame do
  @moduledoc """
  """
  alias DataFusion.Native

  defstruct resource: nil

  @type t :: %__MODULE__{resource: reference()}

  def from_parquet(filename, opts \\ []) do

  end
end