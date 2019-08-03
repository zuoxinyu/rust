## Jzon
A simple and ease-of-use JSON library in Rust.

## TODO";
- TODO: impl Display trait with more options
- TODO: impl Index trait with lifetime
- TODO: impl Iterator trait
- TODO: impl Deref trait
- TODO: impl From trait
- FIXME: float point number parsing precision

## Sample Results
Sample files from [JSON\_checker](http://www.json.org/JSON\_checker/).
P.S.: `fail01.json` is excluded as it is relaxed in RFC7159. `fail18.json` is excluded as depth of JSON is not specified.

### Roundtrip
| file                |       passed       |   size |      cost |
| :------------------ | :----------------: | -----: | --------: |
| roundtrip01.json    | :heavy_check_mark: |   6.0B | 580.000ns |
| roundtrip02.json    | :heavy_check_mark: |   6.0B | 120.000ns |
| roundtrip03.json    | :heavy_check_mark: |   7.0B | 120.000ns |
| roundtrip04.json    | :heavy_check_mark: |   3.0B | 190.000ns |
| roundtrip05.json    | :heavy_check_mark: |   7.0B | 530.000ns |
| roundtrip06.json    | :heavy_check_mark: |   2.0B |  70.000ns |
| roundtrip07.json    | :heavy_check_mark: |   2.0B |   1.660µs |
| roundtrip08.json    | :heavy_check_mark: |   5.0B | 370.000ns |
| roundtrip09.json    | :heavy_check_mark: |  13.0B | 810.000ns |
| roundtrip10.json    | :heavy_check_mark: |  22.0B | 750.000ns |
| roundtrip11.json    | :heavy_check_mark: |   4.0B | 190.000ns |
| roundtrip12.json    | :heavy_check_mark: |  13.0B | 250.000ns |
| roundtrip13.json    | :heavy_check_mark: |  22.0B | 310.000ns |
| roundtrip14.json    | :heavy_check_mark: |  22.0B | 310.000ns |
| roundtrip15.json    | :heavy_check_mark: |   3.0B | 140.000ns |
| roundtrip16.json    | :heavy_check_mark: |  12.0B | 180.000ns |
| roundtrip17.json    | :heavy_check_mark: |  12.0B | 170.000ns |
| roundtrip18.json    | :heavy_check_mark: |  21.0B | 210.000ns |
| roundtrip19.json    | :heavy_check_mark: |  21.0B | 210.000ns |
| roundtrip20.json    | :heavy_check_mark: |   5.0B |   4.520µs |
| roundtrip21.json    | :heavy_check_mark: |   6.0B | 220.000ns |
| roundtrip22.json    | :heavy_check_mark: |   8.0B | 240.000ns |
| roundtrip23.json    | :heavy_check_mark: |   9.0B | 230.000ns |
| roundtrip24.json    | :heavy_check_mark: |   8.0B |   2.920µs |
| roundtrip25.json    | :heavy_check_mark: |  24.0B | 540.000ns |
| roundtrip26.json    | :heavy_check_mark: |  25.0B | 350.000ns |
| roundtrip27.json    | :heavy_check_mark: |  24.0B | 350.000ns |

### Corner Cases
| file                |       passed       |   size |      cost |
| :------------------ | :----------------: | -----: | --------: |
| fail01_EXCLUDE.json |        :x:         |  60.0B | 570.000ns |
| fail02.json         | :heavy_check_mark: |  17.0B | 380.000ns |
| fail03.json         | :heavy_check_mark: |  37.0B | 130.000ns |
| fail04.json         | :heavy_check_mark: |  16.0B | 350.000ns |
| fail05.json         | :heavy_check_mark: |  24.0B | 360.000ns |
| fail06.json         | :heavy_check_mark: |  26.0B | 110.000ns |
| fail07.json         | :heavy_check_mark: |  26.0B | 380.000ns |
| fail08.json         | :heavy_check_mark: |  16.0B | 290.000ns |
| fail09.json         | :heavy_check_mark: |  22.0B | 610.000ns |
| fail10.json         | :heavy_check_mark: |  58.0B | 540.000ns |
| fail11.json         | :heavy_check_mark: |  29.0B | 540.000ns |
| fail12.json         | :heavy_check_mark: |  31.0B | 330.000ns |
| fail13.json         | :heavy_check_mark: |  43.0B | 600.000ns |
| fail14.json         | :heavy_check_mark: |  31.0B | 500.000ns |
| fail15.json         | :heavy_check_mark: |  34.0B | 340.000ns |
| fail16.json         | :heavy_check_mark: |   8.0B |  70.000ns |
| fail17.json         | :heavy_check_mark: |  34.0B | 330.000ns |
| fail18_EXCLUDE.json |        :x:         |  50.0B |   4.040µs |
| fail19.json         | :heavy_check_mark: |  22.0B | 270.000ns |
| fail20.json         | :heavy_check_mark: |  23.0B | 210.000ns |
| fail21.json         | :heavy_check_mark: |  32.0B | 280.000ns |
| fail22.json         | :heavy_check_mark: |  33.0B | 360.000ns |
| fail23.json         | :heavy_check_mark: |  20.0B | 290.000ns |
| fail24.json         | :heavy_check_mark: |  16.0B |  60.000ns |
| fail25.json         | :heavy_check_mark: |  29.0B | 110.000ns |
| fail26.json         | :heavy_check_mark: |  38.0B | 160.000ns |
| fail27.json         | :heavy_check_mark: |  14.0B | 150.000ns |
| fail28.json         | :heavy_check_mark: |  15.0B | 130.000ns |
| fail29.json         | :heavy_check_mark: |   4.0B | 100.000ns |
| fail30.json         | :heavy_check_mark: |   5.0B | 100.000ns |
| fail31.json         | :heavy_check_mark: |   7.0B | 100.000ns |
| fail32.json         | :heavy_check_mark: |  40.0B | 670.000ns |
| fail33.json         | :heavy_check_mark: |  12.0B | 280.000ns |
| pass01.json         | :heavy_check_mark: |   1.4K |  21.849µs |
| pass02.json         | :heavy_check_mark: |  52.0B |   1.480µs |
| pass03.json         | :heavy_check_mark: | 148.0B |   1.660µs |

### Big Files
| file                |       passed       |   size |      cost |
| :------------------ | :----------------: | -----: | --------: |
| canada.json         | :heavy_check_mark: |   2.1M |  23.988ms |
| twitter.json        | :heavy_check_mark: | 616.7K |   8.843ms |
| citm_catalog.json   | :heavy_check_mark: |   1.6M |  10.619ms |
