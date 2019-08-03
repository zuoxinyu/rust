## Jzon
A simple and ease-of-use JSON library in Rust.

## Sample Results
Sample files from [JSON\_checker](http://www.json.org/JSON\_checker/).
P.S.: `fail01.json` is excluded as it is relaxed in RFC7159. `fail18.json` is excluded as depth of JSON is not specified.

## TODO
- TODO: impl Display trait with more options
- TODO: impl Index trait with lifetime
- TODO: impl Iterator trait
- TODO: impl Deref trait
- TODO: impl From trait
- FIXME: float point number parsing precision

### Roundtrip
| file                |       passed       |   size |      cost |
| :------------------ | :----------------: | -----: | --------: |
| roundtrip01.json    | :heavy_check_mark: |   6.0B |   1.030µs |
| roundtrip02.json    | :heavy_check_mark: |   6.0B | 290.000ns |
| roundtrip03.json    | :heavy_check_mark: |   7.0B | 280.000ns |
| roundtrip04.json    | :heavy_check_mark: |   3.0B | 380.000ns |
| roundtrip05.json    | :heavy_check_mark: |   7.0B | 810.000ns |
| roundtrip06.json    | :heavy_check_mark: |   2.0B | 150.000ns |
| roundtrip07.json    | :heavy_check_mark: |   2.0B |   2.610µs |
| roundtrip08.json    | :heavy_check_mark: |   5.0B | 860.000ns |
| roundtrip09.json    | :heavy_check_mark: |  13.0B |   1.570µs |
| roundtrip10.json    | :heavy_check_mark: |  22.0B |   1.540µs |
| roundtrip11.json    | :heavy_check_mark: |   4.0B | 360.000ns |
| roundtrip12.json    | :heavy_check_mark: |  13.0B | 480.000ns |
| roundtrip13.json    | :heavy_check_mark: |  22.0B | 610.000ns |
| roundtrip14.json    | :heavy_check_mark: |  22.0B | 620.000ns |
| roundtrip15.json    | :heavy_check_mark: |   3.0B | 350.000ns |
| roundtrip16.json    | :heavy_check_mark: |  12.0B | 420.000ns |
| roundtrip17.json    | :heavy_check_mark: |  12.0B | 380.000ns |
| roundtrip18.json    | :heavy_check_mark: |  21.0B | 470.000ns |
| roundtrip19.json    | :heavy_check_mark: |  21.0B | 460.000ns |
| roundtrip20.json    | :heavy_check_mark: |   5.0B |   8.420µs |
| roundtrip21.json    | :heavy_check_mark: |   6.0B | 410.000ns |
| roundtrip22.json    | :heavy_check_mark: |   8.0B | 510.000ns |
| roundtrip23.json    | :heavy_check_mark: |   9.0B | 470.000ns |
| roundtrip24.json    | :heavy_check_mark: |   8.0B |   7.750µs |
| roundtrip25.json    | :heavy_check_mark: |  24.0B |   1.090µs |
| roundtrip26.json    | :heavy_check_mark: |  25.0B | 780.000ns |
| roundtrip27.json    | :heavy_check_mark: |  24.0B | 710.000ns |

### Corner Cases
| file                |       passed       |   size |      cost |
| :------------------ | :----------------: | -----: | --------: |
| fail01_EXCLUDE.json |        :x:         |  60.0B |   1.090µs |
| fail02.json         | :heavy_check_mark: |  17.0B | 840.000ns |
| fail03.json         | :heavy_check_mark: |  37.0B | 210.000ns |
| fail04.json         | :heavy_check_mark: |  16.0B | 760.000ns |
| fail05.json         | :heavy_check_mark: |  24.0B | 820.000ns |
| fail06.json         | :heavy_check_mark: |  26.0B | 240.000ns |
| fail07.json         | :heavy_check_mark: |  26.0B | 850.000ns |
| fail08.json         | :heavy_check_mark: |  16.0B | 670.000ns |
| fail09.json         | :heavy_check_mark: |  22.0B |   1.160µs |
| fail10.json         | :heavy_check_mark: |  58.0B |   1.180µs |
| fail11.json         | :heavy_check_mark: |  29.0B |   1.110µs |
| fail12.json         | :heavy_check_mark: |  31.0B | 710.000ns |
| fail13.json         | :heavy_check_mark: |  43.0B |   1.269µs |
| fail14.json         | :heavy_check_mark: |  31.0B |   1.060µs |
| fail15.json         | :heavy_check_mark: |  34.0B | 700.000ns |
| fail16.json         | :heavy_check_mark: |   8.0B | 160.000ns |
| fail17.json         | :heavy_check_mark: |  34.0B | 700.000ns |
| fail18_EXCLUDE.json |        :x:         |  50.0B |   8.630µs |
| fail19.json         | :heavy_check_mark: |  22.0B | 530.000ns |
| fail20.json         | :heavy_check_mark: |  23.0B | 510.000ns |
| fail21.json         | :heavy_check_mark: |  32.0B | 590.000ns |
| fail22.json         | :heavy_check_mark: |  33.0B | 820.000ns |
| fail23.json         | :heavy_check_mark: |  20.0B | 700.000ns |
| fail24.json         | :heavy_check_mark: |  16.0B | 150.000ns |
| fail25.json         | :heavy_check_mark: |  29.0B | 200.000ns |
| fail26.json         | :heavy_check_mark: |  38.0B | 380.000ns |
| fail27.json         | :heavy_check_mark: |  14.0B | 310.000ns |
| fail28.json         | :heavy_check_mark: |  15.0B | 330.000ns |
| fail29.json         | :heavy_check_mark: |   4.0B | 250.000ns |
| fail30.json         | :heavy_check_mark: |   5.0B | 260.000ns |
| fail31.json         | :heavy_check_mark: |   7.0B | 220.000ns |
| fail32.json         | :heavy_check_mark: |  40.0B |   1.360µs |
| fail33.json         | :heavy_check_mark: |  12.0B | 790.000ns |
| pass01.json         | :heavy_check_mark: |   1.4K |  50.879µs |
| pass02.json         | :heavy_check_mark: |  52.0B |   3.450µs |
| pass03.json         | :heavy_check_mark: | 148.0B |   3.400µs |

### Big Files
| file                |       passed       |   size |      cost |
| :------------------ | :----------------: | -----: | --------: |
| canada.json         | :heavy_check_mark: |   2.1M |  28.860ms |
| twitter.json        | :heavy_check_mark: | 616.7K |   6.235ms |
| citm_catalog.json   | :heavy_check_mark: |   1.6M |   9.141ms |
