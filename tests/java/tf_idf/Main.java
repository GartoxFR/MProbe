import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.AbstractMap;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.regex.Pattern;
import java.util.stream.Collector;
import java.util.stream.Collectors;

/**
 * Main
 */
// Système
public class Main {
    static Map<Integer, Integer> parseFile(String filename, IdMap ids) {
        Map<Integer, Integer> termCount = new HashMap<>();
        try (BufferedReader file = new BufferedReader(new FileReader(filename))) {

            String line;
            while ((line = file.readLine()) != null) {
                for (String word : Pattern.compile("\\W+", Pattern.UNICODE_CHARACTER_CLASS).split(line)) {

                    if (word.isEmpty()) {
                        continue;
                    }

                    int id = ids.get_or_register(word.toLowerCase());
                    termCount.compute(id, (key, count) -> count == null ? 1 : count + 1);
                }
            }

        } catch (IOException exception) {
            System.err.println("Error opening " + filename + ": " + exception.getMessage());
            return null;
        }

        return termCount;
    }

    static double computeScore(String query, Map<Integer, Integer> termCount, Map<Integer, Integer> documentTermCount,
            int documentCount, IdMap ids) {
        double score = 0.0;

        int totalTermCount = termCount.values().stream().reduce(Integer::sum).orElseGet(() -> 1);

        for (String word : Pattern.compile("\\W+", Pattern.UNICODE_CHARACTER_CLASS).split(query)) {
            if (word.isEmpty()) {
                continue;
            }

            Integer id = ids.get(word.toLowerCase());
            if (id == null || !termCount.containsKey(id)) {
                continue;
            }

            double tf = (double) termCount.get(id).intValue() / (double) totalTermCount;
            double idf = Math.log((double) documentCount / (double) documentTermCount.get(id).intValue()) 
                / Math.log(2);

            score += tf * idf;
        }

        return score;
    }

    public static void main(String[] args) {
        if (args.length < 2) {
            System.err.println("tf_idf <query> <FILES...>");
            System.exit(1);
        }

        IdMap ids = new IdMap();
        Map<String, Map<Integer, Integer>> filesTermCount = new HashMap<>();
        Map<Integer, Integer> documentTermCount = new HashMap<>();

        for (int i = 1; i < args.length; i++) {
            Map<Integer, Integer> termCount = parseFile(args[i], ids);
            if (termCount == null) {
                continue;
            }

            for (int id : termCount.keySet()) {
                documentTermCount.compute(id, (key, count) -> count == null ? 1 : count + 1);
            }

            filesTermCount.put(args[i], termCount);
        }

        List<Entry<String, Double>> scores = filesTermCount.entrySet().stream()
                .map((entry) -> new AbstractMap.SimpleEntry<>(entry.getKey(),
                        computeScore(args[0], entry.getValue(), documentTermCount, filesTermCount.size(), ids)))
                .sorted(Comparator.comparingDouble(Entry<String, Double>::getValue).reversed())
                .collect(Collectors.toList());

        int limit = Math.min(20, scores.size());
        for (int i = 0; i < limit; i++) {
            Entry<String, Double> entry = scores.get(i);
            System.out.println(entry.getKey() + ": " + entry.getValue());
        }
    }
}
