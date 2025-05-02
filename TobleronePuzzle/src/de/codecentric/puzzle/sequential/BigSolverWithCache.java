package de.codecentric.puzzle.sequential;

import java.math.BigInteger;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

import de.codecentric.puzzle.util.Board;
import de.codecentric.puzzle.util.Game;

/**
 * A solver using a Map as cache and which counts with BigInteger. Way too slow
 * for current PCs if you try it with edge length 8 or more.
 */
public class BigSolverWithCache {
	private static class CacheMap<K, V> extends LinkedHashMap<K, V> {
		private static final long serialVersionUID = -2156293434722519244L;
		private static final int MAX_ENTRIES = 40 * 1000 * 1000;

		public CacheMap() {
			super((int) (1.5 * MAX_ENTRIES), 0.75f, true); // access order (not insertion order)
		}

		@Override
		protected boolean removeEldestEntry(Map.Entry<K, V> eldest) {
			boolean remove = size() > MAX_ENTRIES;
			return remove;
		}

	}

	static private final BigInteger MAX_LONG = BigInteger.valueOf(Long.MAX_VALUE);
	static private Map<Long, Number> cache = new CacheMap<>();
	static int levelCount;

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
		Number solutions = countSolutions(startBoard, 1);
		long end = System.currentTimeMillis();
		System.out.println("Level count is " + levelCount);
		System.out.println("Used time: " + (end - start) / 1000.0 + " seconds");
		System.out.println("Number of solutions for size " + size + ": " + solutions);
		System.out.println("Map has " + cache.size() + " entries");
	}

	private static Number countSolutions(Board start, int level) {
		if (level == 10) {
			// 22470268 positions on level 10 exist
			System.out.println("Count on level " + level + " is " + levelCount + " (" + levelCount / 224703 + "%)");
			levelCount++;
		}
		Long key = Long.valueOf(start.asLong());
		Number result;
		Number value = cache.get(key);
		if (value != null) {
			result = value;
		} else if (start.isSolution()) {
			result = Long.valueOf(1);
			cache.put(key, result);
		} else {
			BigInteger count = BigInteger.ZERO;
			List<Board> nextBoards = start.nextBoards();
			for (Board board : nextBoards) {
				Number x = countSolutions(board, level + 1);
				if (x instanceof BigInteger) {
					count = count.add((BigInteger) x);
				} else { // Long
					count = count.add(BigInteger.valueOf(x.longValue()));
				}
			}
			result = count;
			if (count.compareTo(MAX_LONG) < 0) {
				cache.put(key, Long.valueOf(count.longValue()));
			} else {
				cache.put(key, count);
			}
		}
		return result;
	}
}
