package de.codecentric.puzzle.util;

import java.util.List;

/**
 * A board of size 5 has 15 pegs:
 *
 * <pre>
 *       0 <- column
 *      x 1        0      0
 *     x x 2       1     1 2
 *    x o x 3      2    3 4 5
 *   x x x x 4     3   6 7 8 9
 *  x x x x x      4  0 1 2 3 4
 * </pre>
 */
public interface Board {

    /**
     * @param row Row, starts at 0 (top)
     * @param column Column, starts at 0 (left)
     * @return Is there a peg at the position?
     */
    public boolean isOccupied(int row, int column);

    /**
     * @return Is this a solution (means: Exactly one peg on the board).
     */
    public boolean isSolution();

    /**
     * @return Represention with pegs as bits in an int. Throws {@link UnsupportedOperationException} for big boards.
     */
    public int asInteger();

    /**
     * @return Represention with pegs as bits in a long.
     */
    public long asLong();

    /**
     * @return Number of pegs on the board.
     */
    public int cardinality();

    /**
     * @return Boards which follow from current position with one move.
     */
    public List<Board> nextBoards();
}
