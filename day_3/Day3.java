import java.io.*;
import java.util.*;

public class Day3 {
    public static void main(String[] args) throws IOException {
        var input = new FileReader("./input.txt");
        System.out.println(powerCalculation(input));
        input = new FileReader("./input.txt");
        System.out.println(lifeSupportCalculation(input));
    }

    public static Long powerCalculation(FileReader input) throws IOException {
        var inputFile = new BufferedReader(input);
        var powerCount = new ArrayList<Integer>();
        String currentLine = inputFile.readLine();
        for(int x = 0; x < currentLine.length(); x++) {
            powerCount.add(Integer.parseInt(String.valueOf(currentLine.charAt(x))));
        }
        currentLine = inputFile.readLine();
        int numLines = 1;

        while(currentLine != null) {
            numLines++;
            for(int x = 0; x < currentLine.length(); x++) {
                powerCount.set(x, powerCount.get(x) + Integer.parseInt(String.valueOf(currentLine.charAt(x))));
            }
            currentLine = inputFile.readLine();
        }
        String stringGamma = "";
        String stringEpsilon = "";

        for(int x = 0; x < powerCount.size(); x++) {
            if(powerCount.get(x) > (numLines / 2)) {
                stringGamma += "1";
                stringEpsilon += "0";
            }
            else {
                stringGamma += "0";
                stringEpsilon += "1";
            }
        }
        Long gamma = Long.parseLong(stringGamma, 2);
        Long epsilon = Long.parseLong(stringEpsilon, 2);
        Long finalNum = gamma * epsilon;
        

        return finalNum;
    }

    public static Long lifeSupportCalculation(FileReader input) throws IOException {
        var inputFile = new BufferedReader(input);
        var gammaMap = new HashMap<String, Integer>();
        var epsilonMap = new HashMap<String, Integer>();
        String currentLine = inputFile.readLine();
        int lineLength = currentLine.length();

        while(currentLine != null) {
            gammaMap.put(currentLine, 1);
            epsilonMap.put(currentLine, 1);
            currentLine = inputFile.readLine();
        }
        
        for(int i = 0; i < lineLength; i++) {
            int gammaPowerCount = 0;
            var gammaOccurrenceIterator = gammaMap.keySet().iterator();
            while(gammaOccurrenceIterator.hasNext()) {
                currentLine = gammaOccurrenceIterator.next();
                int currentDigit = Integer.parseInt(String.valueOf(currentLine.charAt(i)));
                if(currentDigit == 1) gammaPowerCount++;
                else gammaPowerCount--;
            }
            int epsilonPowerCount = 0;
            var epsilonOccurrenceIterator = epsilonMap.keySet().iterator();
            while(epsilonOccurrenceIterator.hasNext()) {
                currentLine = epsilonOccurrenceIterator.next();
                int currentDigit = Integer.parseInt(String.valueOf(currentLine.charAt(i)));
                if(currentDigit == 1) epsilonPowerCount++;
                else epsilonPowerCount--;
            }

            String stringGamma = "";
            String stringEpsilon = "";

            if(gammaPowerCount >= 0) stringGamma = "1";
            else stringGamma = "0";

            if(epsilonPowerCount >= 0) stringEpsilon = "0";
            else stringEpsilon = "1";

            var gammaIterator = gammaMap.keySet().iterator();
            while(gammaIterator.hasNext() && gammaMap.size() > 1) {
                currentLine = gammaIterator.next();
                if(!stringGamma.equals(String.valueOf(currentLine.charAt(i)))) {
                    gammaIterator.remove();
                }
            }
            var epsilonIterator = epsilonMap.keySet().iterator();
            while(epsilonIterator.hasNext() && epsilonMap.size() > 1) {
                currentLine = epsilonIterator.next();
                if(!stringEpsilon.equals(String.valueOf(currentLine.charAt(i)))) {
                    epsilonIterator.remove();
                }
            }
        }

        return Long.parseLong(epsilonMap.keySet().iterator().next(), 2) * Long.parseLong(gammaMap.keySet().iterator().next(), 2);
    }
}
