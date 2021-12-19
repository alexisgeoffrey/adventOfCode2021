defmodule Day8 do
  use Application

  def start(_type, _args) do
    file = File.read!(Path.expand("./input/input.txt"))

    Task.start(fn -> IO.puts("Count 1s, 4s, 7s, and 8s: #{Day8.countSomeDigits(file)}") end)
    Task.start(fn -> IO.puts("Output value total: #{Day8.sumAllDigits(file)}") end)
  end

  @spec countSomeDigits(binary) :: integer
  def countSomeDigits(file) do
    file
    |> String.split("\n")
    |> Enum.flat_map(
      &(String.split(&1, "|")
        |> tl()
        |> Enum.flat_map(fn e -> String.split(e) end))
    )
    |> Enum.reduce(0, fn d, acc ->
      case String.length(d) do
        x when x === 2 or x === 3 or x === 4 or x === 7 -> acc + 1
        _ -> acc
      end
    end)
  end

  @spec sumAllDigits(binary) :: list
  def sumAllDigits(file) do
    file
    |> String.split("\n")
    |> Enum.map(
      &(String.split(&1, " | ")
        |> hd())
    )
  end

  @spec digitMatch(binary) :: any
  defp digitMatch(input) do
    input
  end
end
