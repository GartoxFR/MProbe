import java.util.HashMap;
import java.util.Map;

/**
 * IdMap
 */
public class IdMap {
    private final Map<String, Integer> inner = new HashMap<>();
    private int nextId = 0;
    
    public Integer get(String word) {
        return this.inner.get(word);
    }

    public Integer register(String word) {
        int id = this.nextId;
        this.nextId++;
        this.inner.put(word, id);
        return id;
    }

    public Integer get_or_register(String word) {
        Integer id = this.get(word);
        if (id == null) {
            id = this.register(word);
        }

        return id;
    }
}
