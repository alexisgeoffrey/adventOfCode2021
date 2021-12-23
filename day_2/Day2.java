package day_2;

import java.io.*;

public class Day2 {
    public static void main(String[] args) throws IOException {
        var input = new BufferedReader(new FileReader("day_2/input.txt"));
        System.out.println(pathReader(input));
        input = new BufferedReader(new FileReader("day_2/input.txt"));
        System.out.println(aimedPathReader(input));
    }

    public static int pathReader(BufferedReader inputFile) throws IOException {
        String currentLine = inputFile.readLine();
        int depth = 0;
        int position = 0;

        while (currentLine != null) {
            String[] lineTokens = currentLine.split(" ");
            int value = Integer.parseInt(lineTokens[1]);
            switch (lineTokens[0]) {
                case "forward":
                    position += value;
                    break;
                case "down":
                    depth += value;
                    break;
                case "up":
                    depth -= value;
                    break;
                default:
                    break;
            }
            currentLine = inputFile.readLine();
        }

        return position * depth;
    }

    public static int aimedPathReader(BufferedReader inputFile) throws IOException {
        String currentLine = inputFile.readLine();
        int depth = 0;
        int position = 0;
        int aim = 0;

        while (currentLine != null) {
            String[] lineTokens = currentLine.split(" ");
            int value = Integer.parseInt(lineTokens[1]);
            switch (lineTokens[0]) {
                case "forward":
                    position += value;
                    depth += aim * value;
                    break;
                case "down":
                    aim += value;
                    break;
                case "up":
                    aim -= value;
                    break;
                default:
                    break;
            }
            currentLine = inputFile.readLine();
        }

        return position * depth;
    }
}
