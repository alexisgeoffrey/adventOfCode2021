import java.io.*;
import java.util.ArrayList;

public class Day1 {
    public static void main(String[] args) throws IOException {
        BufferedReader input = new BufferedReader(new FileReader("./input.txt"));
        System.out.println(countDepthIncrease(input));
        input = new BufferedReader(new FileReader("./input.txt"));
        System.out.println(countSlidingDepthIncrease(input));
    }

    public static int countDepthIncrease(BufferedReader inputFile) throws IOException {
        int numIncreases = 0;
        String currentLine = inputFile.readLine();
        int prevDepth = Integer.parseInt(currentLine);
        currentLine = inputFile.readLine();

        while(currentLine != null) {
            int currentDepth = Integer.parseInt(currentLine);
            if(currentDepth > prevDepth) numIncreases++;
            prevDepth = currentDepth;
            currentLine = inputFile.readLine();
        }

        return numIncreases;
    }

    public static int countSlidingDepthIncrease(BufferedReader inputFile) throws IOException {
        int numIncreases = 0;
        ArrayList<Integer> inputList = new ArrayList<Integer>();
        String currentLine = inputFile.readLine();

        while(currentLine != null) {
            inputList.add(Integer.parseInt(currentLine));
            currentLine = inputFile.readLine();
        }

        int start = 0;
        int end = 2;
        int prevDepth = Integer.MAX_VALUE;
        int currentDepth;
        while(end < inputList.size()) {
            currentDepth = 0;
            for(int x = start; x <= end; x++) {
                currentDepth += inputList.get(x);
            }
            if(currentDepth > prevDepth) numIncreases++;
            prevDepth = currentDepth;
            start++;
            end++;
        }

        return numIncreases;
    }
}
