import maze


def main():
    m = maze.Maze(10)

    # Prints visited cells
    print('visited')
    for row in m.visited:
        print(row)

    # Prints the direction arrays
    print('north')
    for row in m.north:
        print(row)
    print('east')
    for row in m.east:
        print(row)
    print('south')
    for row in m.south:
        print(row)
    print('west')
    for row in m.west:
        print(row)

    pass

if __name__ == '__main__':
    main()
