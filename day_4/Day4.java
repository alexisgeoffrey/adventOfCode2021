package day_4;

import java.io.*;
import java.util.ArrayList;
import java.util.Arrays;

public class Day4 {
    public static void main(String[] args) throws IOException {
        var input = new FileReader("day_4/input.txt");
        System.out.println(bingo(input));
        input = new FileReader("day_4/input.txt");
        System.out.println(bingoLose(input));
    }

    public static int bingo(FileReader input) throws IOException {
        var inputFile = new BufferedReader(input);
        String[] numbers = inputFile.readLine().split(",");
        inputFile.readLine();
        var boards = populateBoards(inputFile);

        for (String currentNum : numbers) {
            for (String[][] currentBoard : boards) {
                int checkForBingo = -1;
                int y = 0;
                currentBoard: for (String[] currentBoardLine : currentBoard) {
                    int x = 0;
                    for (String currentLineNum : currentBoardLine) {
                        if (currentNum.equals(currentLineNum)) {
                            currentBoardLine[x] = "true";
                            checkForBingo = x;
                            break currentBoard;
                        }
                        x++;
                    }
                    y++;
                }
                if (checkForBingo != -1) {
                    if (gotBingo(currentBoard, checkForBingo, y)) {
                        return calculateUnmarked(currentBoard) * Integer.parseInt(currentNum);
                    }
                }
            }
        }

        return -1;
    }

    public static int bingoLose(FileReader input) throws IOException {
        var inputFile = new BufferedReader(input);
        String[] numbers = inputFile.readLine().split(",");
        inputFile.readLine();
        var boards = populateBoards(inputFile);
        int finalTotal = 0;

        for (String currentNum : numbers) {
            int z = 0;
            for (String[][] currentBoard : boards) {
                if (currentBoard == null) {
                    z++;
                    continue;
                }
                int checkForBingo = -1;
                int y = 0;
                currentBoard: for (String[] currentBoardLine : currentBoard) {
                    int x = 0;
                    for (String currentLineNum : currentBoardLine) {
                        if (currentNum.equals(currentLineNum)) {
                            currentBoardLine[x] = "true";
                            checkForBingo = x;
                            break currentBoard;
                        }
                        x++;
                    }
                    y++;
                }
                if (checkForBingo != -1) {
                    if (gotBingo(currentBoard, checkForBingo, y)) {
                        finalTotal = calculateUnmarked(currentBoard) * Integer.parseInt(currentNum);
                        boards[z] = null;
                    }
                }
                z++;
            }
        }

        return finalTotal;
    }

    private static int calculateUnmarked(String[][] currentBoard) {
        int total = 0;
        for (String[] currentBoardLine : currentBoard) {
            for (String currentLineNum : currentBoardLine) {
                if (currentLineNum != "true") {
                    total += Integer.parseInt(currentLineNum);
                }
            }
        }

        return total;
    }

    private static String[][][] populateBoards(BufferedReader inputFile) throws IOException {
        var boards = new ArrayList<String[][]>();
        String currentLine = inputFile.readLine();

        while (currentLine != null) {
            String[][] currentBoard = new String[5][5];
            for (int y = 0; y < 5; y++) {
                var currentRow = Arrays.stream(currentLine.split(" ")).filter(s -> !s.isEmpty()).toArray(String[]::new);
                for (int x = 0; x < currentRow.length; x++) {
                    currentBoard[y][x] = currentRow[x];
                }
                currentLine = inputFile.readLine();
            }
            boards.add(currentBoard);
            currentLine = inputFile.readLine();
        }
        String[][][] finalBoards = new String[boards.size()][][];
        return boards.toArray(finalBoards);
    }

    private static boolean gotBingo(String[][] currentBoard, int currentX, int currentY) {
        boolean isBingo = true;
        for (String currentLineNum : currentBoard[currentY]) {
            if (currentLineNum != "true") {
                isBingo = false;
                break;
            }
        }
        if (isBingo)
            return isBingo;
        isBingo = true;
        for (String[] currentLine : currentBoard) {
            if (currentLine[currentX] != "true") {
                isBingo = false;
                break;
            }
        }

        return isBingo;
    }
}
