public class Wx {
    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("axwy  <write MB>");
        }

        // Java doesn't support array bigger than 2GB
        int write = 0;

        try {
            write = Integer.parseInt(args[0]);
        } catch (NumberFormatException ex) {
            System.err.println("axwy <write MB>");
            System.exit(1);
        }

        write *= 1024 * 1024;

        byte[] arr = new byte[write];

        for(int i = 0; i < write; i++) {
            arr[i] = 42;
        }
    }
}
