package de.codecentric.puzzle;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;

import java.util.List;

import org.junit.Test;

import de.codecentric.puzzle.util.Board;
import de.codecentric.puzzle.util.Game;

public class GameTest {

    @Test
    public void testGame() {
        String nl = System.lineSeparator();
        Game g = new Game(5);
        assertEquals(5, g.size());
        assertEquals(15, g.numberOfFields());
        Board start = g.startBoard();
        String s = start.toString();
        assertEquals("    x" + nl + "   x x" + nl + "  x o x" + nl + " x x x x" + nl + "x x x x x" + nl, s);
        assertEquals(14, start.cardinality());
        assertFalse(start.isSolution());
        List<Board> nb = start.nextBoards();
        assertEquals(2, nb.size());
        System.out.print(nb.get(0));
        System.out.print(nb.get(1));

        g = new Game(4);
        s = g.startBoard().toString();
        assertEquals("   x" + nl + "  x x" + nl + " x o x" + nl + "x x x x" + nl, s);
    }

}
