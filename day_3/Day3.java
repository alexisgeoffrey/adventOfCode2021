package day_3;

import java.io.*;
import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;

public class Day3 {
    public static void main(String[] args) throws IOException, InterruptedException {
        var input = new FileReader("day_3/input.txt");
        System.out.println(powerCalculation(input));
        input = new FileReader("day_3/input.txt");
        System.out.println(lifeSupportCalculation(input));
    }

    public static Long powerCalculation(FileReader input) throws IOException {
        var inputFile = new BufferedReader(input);
        var powerCount = new ArrayList<Integer>();
        String currentLine = inputFile.readLine();
        for (int x = 0; x < currentLine.length(); x++) {
            powerCount.add(Integer.parseInt(String.valueOf(currentLine.charAt(x))));
        }
        currentLine = inputFile.readLine();
        int numLines = 1;

        while (currentLine != null) {
            numLines++;
            for (int x = 0; x < currentLine.length(); x++) {
                powerCount.set(x, powerCount.get(x) + Integer.parseInt(String.valueOf(currentLine.charAt(x))));
            }
            currentLine = inputFile.readLine();
        }
        String stringGamma = "";
        String stringEpsilon = "";

        for (int x = 0; x < powerCount.size(); x++) {
            if (powerCount.get(x) > (numLines / 2)) {
                stringGamma += "1";
                stringEpsilon += "0";
            } else {
                stringGamma += "0";
                stringEpsilon += "1";
            }
        }
        Long gamma = Long.parseLong(stringGamma, 2);
        Long epsilon = Long.parseLong(stringEpsilon, 2);
        Long finalNum = gamma * epsilon;

        return finalNum;
    }

    public static Long lifeSupportCalculation(FileReader input) throws IOException, InterruptedException {
        var inputFile = new BufferedReader(input);
        var gammaSet = new HashSet<String>();
        var epsilonSet = new HashSet<String>();
        String currentLine = inputFile.readLine();
        int lineLength = currentLine.length();

        while (currentLine != null) {
            gammaSet.add(currentLine);
            epsilonSet.add(currentLine);
            currentLine = inputFile.readLine();
        }

        for (int x = 0; x < lineLength; x++) {
            final int i = x;
            Thread gamma = new Thread(() -> calcSet(gammaSet, true, i));
            Thread epsilon = new Thread(() -> calcSet(epsilonSet, false, i));
            gamma.start();
            epsilon.start();
            gamma.join();
            epsilon.join();
        }

        return Long.parseLong(gammaSet.iterator().next(), 2) * Long.parseLong(epsilonSet.iterator().next(), 2);

    }

    private static void calcSet(Set<String> set, boolean isGamma, int index) {
        var powerCount = new AtomicInteger();
        final int i = index;

        set.parallelStream().forEach(
                (line) -> powerCount
                        .addAndGet(Integer.parseInt(String.valueOf(line.charAt(i))) == 1 ? 1 : -1));

        String string = powerCount.get() >= 0 ? isGamma ? "1" : "0" : isGamma ? "0" : "1";

        var iter = set.iterator();

        while (iter.hasNext() && set.size() > 1) {
            if (!String.valueOf(iter.next().charAt(i)).equals(string))
                iter.remove();
        }
        if (set.size() == 1)
            return;
    }
}
