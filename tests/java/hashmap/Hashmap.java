import java.util.HashMap;

public class Hashmap {
    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("hashmap  <count>");
        }

        // Java doesn't support array bigger than 2GB
        int write = 0;

        try {
            write = Integer.parseInt(args[0]);
        } catch (NumberFormatException ex) {
            System.err.println("hashmap  <count>");
            System.exit(1);
        }

        HashMap<String, Integer> map = new HashMap<>();
        for (int i = 0; i < write; i++) {
            map.put(Integer.toString(i), i);
        }
    }
}
