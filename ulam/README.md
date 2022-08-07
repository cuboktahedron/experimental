## Ulam

## 素数を反時計回りに四角形上に敷き詰める(ウラムの螺旋)

```
./main --gp=1:10000
```

<img src="https://raw.githubusercontent.com/cuboktahedron/experimental/main/ulam/imgs/primes-spiral4-800-1_10000.png" width="400" height="400">

```
./main_gif --gp=1:10000 --animation=500:1 --interval=200 --wait-after=3000
```

<img src="https://raw.githubusercontent.com/cuboktahedron/experimental/main/ulam/imgs/primes-spiral4-800-1_10000.gif" width="400" height="400">

## 素数を反時計回りに六角形上に敷き詰める

```
/main --tile=spiral6 --gp=1:10000
```

<img src="https://raw.githubusercontent.com/cuboktahedron/experimental/main/ulam/imgs/primes-spiral6-800-1_10000.png" width="400" height="400">


## 倍数をジグザグに正方形上に敷き詰める

```
./times_spiral_gif --tile=zigzag4 --interval=200 --wait-after=3000 --from=1 --to=100000
```

<img src="https://raw.githubusercontent.com/cuboktahedron/experimental/main/ulam/imgs/times-zigzag4-800-1_100000.gif" width="400" height="400">
