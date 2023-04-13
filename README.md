# Labyrinth generator and solver

Algorytm has two parts: generation and solving. Maze gets generated randomly every time and then it can be solved using multi-threading.

## Dependencies

Since the programme is written in rust, it's necessary to set up the rust toolchain. It can easily be done with a single command:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For maze rendering python is used so it's necessary to set up a `virtual environment` or use `conda`. Dependencies necessary for running the code are `matplotlib` and `maturin`.

## Useful commands

Buiding and running the programme:
```
bash run.sh
```

Only building the rust library:
```
bash run.sh -b
```

Formatting rust code in the `/lib` directory:
```
bash run.sh -f
```
