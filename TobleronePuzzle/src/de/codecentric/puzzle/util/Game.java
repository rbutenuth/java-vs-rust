package de.codecentric.puzzle.util;


/**
 * <a href="http://www.danobrien.ws/PegBoard.html">Stuff on the net</a>
 * For <code>size = 5</code> there are 1550 solutions according to that site.
 */
public class Game {
    private final int size;
    private final int positions;

    /**
     * @param size Edge size (number of rows).
     */
    public Game(int size) {
        this.size = size;
        positions = size * (size + 1) / 2;
    }

    /**
     * @return Edge size (number of rows).
     */
    public int size() {
        return size;
    }

    /**
     * @return Number of fields on the board.
     */
    public int numberOfFields() {
        return positions;
    }

    /**
     * @return The starting board, initial hole in the middle of the third row.
     */
    public Board startBoard() {
        if (size < 32) {
            return new SmallBoard(this);
        } else {
            return new BigBoard(this);
        }
    }
}
