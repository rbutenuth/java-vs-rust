package de.codecentric.puzzle.sequential;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

import de.codecentric.puzzle.util.Board;
import de.codecentric.puzzle.util.Game;

/**
 * Sequential solver with {@link HashMap} as cache.
 */
public class SolverWithMapCache {
    static private Map<Board, Long> cache = new HashMap<>();

    /**
     * @param args Size of the board.
     */
    public static void main(String[] args) {
        if (args.length != 1) {
            throw new IllegalArgumentException("Expect one argument: Size of the board.");
        }
        int size = Integer.valueOf(args[0]);
        Game g = new Game(size);
        Board startBoard = g.startBoard();
        long start = System.currentTimeMillis();
        long solutions = countSolutions(startBoard);
        long end = System.currentTimeMillis();
        System.out.println("Used time: " + (end - start) / 1000.0 + " seconds");
        System.out.println("Number of solutions for size " + size + ": " + solutions);
        System.out.println("Map has " + cache.size() + " entries");
    }

    private static long countSolutions(Board start) {
        Long value = cache.get(start);
        if (value != null) {
            return value.longValue();
        } else if (start.isSolution()) {
            cache.put(start, Long.valueOf(1));
            return 1;
        } else {
            long count = 0;
            List<Board> nextBoards = start.nextBoards();
            for (Board board : nextBoards) {
                count += countSolutions(board);
            }
            cache.put(start, Long.valueOf(count));
            return count;
        }
    }
}
