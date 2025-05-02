package de.codecentric.puzzle.sequential;

import java.util.Arrays;
import java.util.List;

import de.codecentric.puzzle.util.Board;
import de.codecentric.puzzle.util.Game;

/**
 * Solver wich uses an array of longs for the cache.
 */
public class SolverWithArrayCache {

	/**
	 * @param args Size of the board.
	 */
	public static void main(String[] args) {
		if (args.length != 1) {
			throw new IllegalArgumentException("Expect one argument: Size of the board.");
		}
		for (int count = 0; count < 3; count++) {
			int size = Integer.valueOf(args[0]);

			long start = System.currentTimeMillis();
			Game g = new Game(size);
			int cacheEntries = 1 << g.numberOfFields();
			System.out.println("Cache has " + cacheEntries + " entries (" + (cacheEntries * 8L / (1024 * 1024)) + " Mbytes)");
			long[] cache = new long[cacheEntries];
			Arrays.fill(cache, -1);
			Board startBoard = g.startBoard();
			long solutions = countSolutions(cache, startBoard);
			long end = System.currentTimeMillis();
			System.out.println("Used time: " + (end - start) / 1000.0 + " seconds");
			System.out.println("Number of solutions for size " + size + ": " + solutions);
		}
	}

	private static long countSolutions(long[] cache, Board start) {
		int asInt = start.asInteger();
		if (cache[asInt] != -1) {
			return cache[asInt];
		} else if (start.isSolution()) {
			cache[asInt] = 1;
			return 1;
		} else {
			long count = 0;
			List<Board> nextBoards = start.nextBoards();
			for (Board board : nextBoards) {
				count += countSolutions(cache, board);
			}
			cache[asInt] = count;
			return count;
		}
	}
}
/*
 * Cache has 268435456 entries (2048 Mbytes) Used time: 36.099 seconds Number of
 * solutions for size 7: 0
 */