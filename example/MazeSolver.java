package pkg2pc.assign01;

/**
 * Assignment 01 - Implement parallel maze solving algorithm using JAVA Concurrent API
 * 
 * use maze.isNorth(x,y), maze.isSouth(x,y), etc to check if there is wall in exact direction
 * use maze.setVisited(x,y) and maze.isVisited(x,y) to check is you already have been at particular cell in the maze
 * @author Janis
 */
public class MazeSolver {
    
    Maze maze;
    
    //Current position
    int x;
    int y;
    
    public MazeSolver(Maze m){
        this.maze = m;
    }
    
    /**
     * Solve maze starting from initial positions
     * @param x starting position in x direction
     * @param y starting position in y direction
     */
    public void solveMaze(int x, int y){
        //Set current position to initial position
        this.x = x;
        this.y = y;
        
        // TODO Your algorithm here. If necessary, create another methods (for example, recursive solve method)
        
        
        // Use theses lines where necessary to draw movement of the solver
        StdDraw.setPenColor(StdDraw.BLUE);
        StdDraw.filledCircle(x + 0.5, y + 0.5, 0.25);
        
        //sleeps for 30 ms
        StdDraw.show(30);
    }
    
}
