defmodule Day9 do
  use Application

  def start(_type, _args) do
    file = File.read!(Path.expand("./input/input.txt"))
    # result1 = countSomeDigits(file)
    # result2 = sumAllDigits(file)

    # Task.start(fn -> IO.puts("Count 1s, 4s, 7s, and 8s: #{result1}") end)
    # Task.start(fn -> IO.puts("Output value total: #{result2}") end)
  end
end
