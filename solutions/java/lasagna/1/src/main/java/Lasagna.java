public class Lasagna {

    public int expectedMinutesInOven() {
        return 40;
    }

    public int remainingMinutesInOven(int minutes) {
        return 40 - minutes;
    }

    public int preparationTimeInMinutes(int layers) {
        return 2 * layers;
    }

    public int totalTimeInMinutes(int layers, int minutes) {
        return (2 * layers) + minutes;
    }
}
