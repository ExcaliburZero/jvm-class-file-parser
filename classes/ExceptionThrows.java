public class ExceptionThrows {
    void throwException() {
        try {
            assert(((String) null).length() == 0);
        } catch (Exception e) {
        }
        try {
            assert(((String) null).length() == 0);
        } catch (Exception e) {
        }
    }
}
