package de.codecentric.puzzle.sequential;

import de.codecentric.puzzle.util.Board;
import de.codecentric.puzzle.util.Game;

/**
 * Simple sequential solver.
 */
public class SimpleSequentialSolver {

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
        int solutions;
        solutions = countSolutions(startBoard);
        long end = System.currentTimeMillis();
        System.out.println("Used time: " + (end - start) / 1000.0 + " seconds");
        System.out.println("Number of solutions for size " + size + ": " + solutions);
    }

    private static int countSolutions(Board start) {
        if (start.isSolution()) {
            return 1;
        } else {
            int count = 0;
            for (Board board : start.nextBoards()) {
                count += countSolutions(board);
            }
            return count;
        }
    }
}
