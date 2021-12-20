defmodule Day8 do
  use Application

  def start(_type, _args) do
    file = File.read!(Path.expand("./input/input.txt"))
    result1 = countSomeDigits(file)
    result2 = sumAllDigits(file)

    Task.start(fn -> IO.puts("Count 1s, 4s, 7s, and 8s: #{result1}") end)
    Task.start(fn -> IO.puts("Output value total: #{result2}") end)
  end

  @spec countSomeDigits(binary) :: integer
  def countSomeDigits(file) do
    file
    |> String.split("\n")
    |> Stream.flat_map(
      &(String.split(&1, "|")
        |> tl()
        |> Enum.flat_map(fn e -> String.split(e) end))
    )
    |> Enum.reduce(0, fn d, acc ->
      length = String.length(d)

      if length === 2 or length === 3 or length === 4 or length === 7 do
        acc + 1
      else
        acc
      end
    end)
  end

  @spec sumAllDigits(binary) :: integer
  def sumAllDigits(file) do
    file
    |> String.split("\n")
    |> Stream.map(fn line ->
      l = String.split(line, "|")

      # IO.puts(l)

      oneAndFour =
        hd(l)
        |> String.split()
        |> Enum.reduce(%{}, fn e, acc ->
          case String.length(e) do
            2 -> Map.put(acc, 1, String.graphemes(e) |> Enum.sort() |> to_string)
            4 -> Map.put(acc, 4, String.graphemes(e) |> Enum.sort() |> to_string)
            _ -> acc
          end
        end)

      fullMap =
        hd(l)
        |> String.split()
        |> Enum.reduce(oneAndFour, fn e, acc ->
          elem = e |> String.graphemes() |> Enum.sort() |> to_string()
          one = Map.get(oneAndFour, 1)
          four = Map.get(oneAndFour, 4)

          case {String.length(e), one |> numInCommon(e), four |> numInCommon(e)} do
            {5, 1, 2} -> Map.put(acc, 2, elem)
            {5, 2, 3} -> Map.put(acc, 3, elem)
            {5, 1, 3} -> Map.put(acc, 5, elem)
            {6, 1, 3} -> Map.put(acc, 6, elem)
            {3, 2, 2} -> Map.put(acc, 7, elem)
            {7, 2, 4} -> Map.put(acc, 8, elem)
            {6, 2, 4} -> Map.put(acc, 9, elem)
            {6, 2, 3} -> Map.put(acc, 0, elem)
            _ -> acc
          end
        end)

      tl(l)
      |> List.first()
      |> String.split()
      |> Enum.map(fn signal ->
        sorted = String.graphemes(signal) |> Enum.sort() |> to_string()

        Map.filter(fullMap, fn {_key, val} ->
          sorted == val
        end)
        |> Map.keys()
        |> List.first()
      end)
      |> Enum.join()
      |> String.to_integer()
    end)
    |> Enum.sum()
  end

  defp numInCommon(oneOrFour, current) do
    sorted = String.graphemes(current) |> Enum.sort()

    sorted
    |> Enum.reduce(0, fn e, acc ->
      if String.contains?(oneOrFour, e |> to_string) do
        acc + 1
      else
        acc
      end
    end)
  end
end
