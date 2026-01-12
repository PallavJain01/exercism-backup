public class LogLevels {

    public static String message(String logLine) {
        String message = logLine.split(":")[1].strip();

        return message;
    }

    public static String logLevel(String logLine) {
        String level = logLine.split(":")[0].strip();

        String levelFormatted = level.substring(1, level.length() - 1);

        return levelFormatted.toLowerCase();
    }

    public static String reformat(String logLine) {
        String message = message(logLine);
        String logLevel = logLevel(logLine);

        return "%s (%s)".formatted(message, logLevel);
    }
}
