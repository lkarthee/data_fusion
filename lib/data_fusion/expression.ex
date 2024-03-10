defmodule DataFusion.Expression do
  @moduledoc false

  defstruct resource: nil

  @type t :: %__MODULE__{resource: reference()}
  
end