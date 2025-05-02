package de.codecentric.puzzle.util;

import java.util.ArrayList;
import java.util.List;

/**
 * A board of size 5 has 15 pegs:
 *
 * <pre>
 *       0 <- column  row   bit index
 *      x 1           0       0
 *     x x 2          1      1 2
 *    x o x 3         2     3 4 5
 *   x x x x 4        3    6 7 8 9
 *  x x x x x 5       4  10 1 2 3 4
 * </pre>
 */
public class BigBoard implements Board {
    private final Game game;
    private long pegs;

    /**
     * Create a start board, completely filled with pegs except at row 2, column 1.
     * @param game Specifies the size of the board.
     */
    public BigBoard(Game game) {
        this.game = game;
        pegs = 0;
        for (int r = 0; r < game.size(); r++) {
            for (int c = 0; c <= r; c++) {
                setPeg(r, c);
            }
        }
        removePeg(2, 1);
    }

    private BigBoard(Game game, long pegs) {
        this.game = game;
        this.pegs = pegs;
    }

    private void setPeg(int row, int column) {
        pegs |= 1L << position(row, column);
    }

    private void removePeg(int row, int column) {
        pegs &= ~(1L << position(row, column));
    }

    public int position(int row, int column) {
        return row * (row + 1) / 2 + column;
    }

    @Override
    public boolean isOccupied(int row, int column) {
        return (pegs & (1 << position(row, column))) != 0;
    }

    @Override
    public boolean isSolution() {
        return cardinality() == 1;
    }

    @Override
    public int asInteger() {
        throw new UnsupportedOperationException("too big for int");
    }

    @Override
    public long asLong() {
        return pegs;
    }

    @Override
    public int cardinality() {
        int n = game.numberOfFields();
        int count = 0;
        long mask = 1L;
        for (int i = 0; i < n; i++, mask <<= 1) {
            if ((pegs & mask) != 0) {
                count++;
            }
        }
        return count;
    }

    @Override
    public List<Board> nextBoards() {
        ArrayList<Board> boards = new ArrayList<>();
        for (int r = 0; r < game.size(); r++) {
            for (int c = 0; c <= r; c++) {
                // There must be a peg at the start position
                if (isOccupied(r, c)) {
                    addMove(boards, r, c, 0, 1); // right
                    addMove(boards, r, c, 0, -1); // left
                    addMove(boards, r, c, -1, 0); // up + right
                    addMove(boards, r, c, -1, -1); // up + left
                    addMove(boards, r, c, 1, 1); // down + right
                    addMove(boards, r, c, 1, 0); // down + left
                }
            }
        }
        return boards;
    }

    private void addMove(List<Board> boards, int r, int c, int y, int x) {
        // The "jump to field" must be on the board
        if (isValid(r + 2 * y, c + 2 * x)) {
            // There must be a peg on the "jump over field"
            if (isOccupied(r + y, c + x)) {
                // The "jump to field" must be empty
                if (!isOccupied(r + 2 * y, c + 2 * x)) {
                    BigBoard next = new BigBoard(game, pegs);
                    next.removePeg(r, c);
                    next.removePeg(r + y, c + x);
                    next.setPeg(r + 2 * y, c + 2 * x);
                    boards.add(next);
                }
            }
        }
    }

    private boolean isValid(int r, int c) {
        int s = game.size();
        if (r < 0 || r >= s) {
            return false;
        }
        if (c < 0 || c > r) {
            return false;
        }
        return true;
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = (int)(prime * result + pegs);
        return result;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null)
            return false;
        if (getClass() != obj.getClass())
            return false;
        BigBoard other = (BigBoard) obj;
        if (pegs != other.pegs)
            return false;
        return true;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        for (int r = 0; r < game.size(); r++) {
            for (int c = 0; c < game.size() - r - 1; c++) {
                sb.append(' ');
            }
            for (int c = 0; c <= r; c++) {
                sb.append(isOccupied(r, c) ? 'x' : 'o');
                if (c < r) {
                    sb.append(' ');
                }
            }
            sb.append(System.lineSeparator());
        }
        return sb.toString();
    }
}
