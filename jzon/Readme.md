## Jzon
A simple JSON library in Rust.

## Sample Results
Sample files from [JSON\_checker](http://www.json.org/JSON\_checker/).
P.S.: `fail01.json` is excluded as it is relaxed in RFC7159. `fail18.json` is excluded as depth of JSON is not specified.

### Roundtrip
| file                |       passed       |   size |      cost |
| ------------------- | ------------------ | ------ | --------- |
| roundtrip26.json    | :heavy_check_mark: |  25.0B |  13.299µs |
| roundtrip02.json    | :heavy_check_mark: |   6.0B | 270.000ns |
| roundtrip19.json    | :heavy_check_mark: |  21.0B | 720.000ns |
| roundtrip20.json    | :heavy_check_mark: |   5.0B | 340.000ns |
| roundtrip01.json    | :heavy_check_mark: |   6.0B | 240.000ns |
| roundtrip15.json    | :heavy_check_mark: |   3.0B | 320.000ns |
| roundtrip22.json    | :heavy_check_mark: |   8.0B | 400.000ns |
| roundtrip16.json    | :heavy_check_mark: |  12.0B | 390.000ns |
| roundtrip17.json    | :heavy_check_mark: |  12.0B | 350.000ns |
| roundtrip21.json    | :heavy_check_mark: |   6.0B | 320.000ns |
| roundtrip07.json    | :heavy_check_mark: |   2.0B |   2.360µs |
| roundtrip12.json    | :heavy_check_mark: |  13.0B | 420.000ns |
| roundtrip27.json    | :heavy_check_mark: |  24.0B | 930.000ns |
| roundtrip03.json    | :heavy_check_mark: |   7.0B | 240.000ns |
| roundtrip25.json    | :heavy_check_mark: |  24.0B | 830.000ns |
| roundtrip13.json    | :heavy_check_mark: |  22.0B | 490.000ns |
| roundtrip11.json    | :heavy_check_mark: |   4.0B | 260.000ns |
| roundtrip04.json    | :heavy_check_mark: |   3.0B | 280.000ns |
| roundtrip24.json    | :heavy_check_mark: |   8.0B | 740.000ns |
| roundtrip14.json    | :heavy_check_mark: |  22.0B | 490.000ns |
| roundtrip05.json    | :heavy_check_mark: |   7.0B | 730.000ns |
| roundtrip08.json    | :heavy_check_mark: |   5.0B | 620.000ns |
| roundtrip18.json    | :heavy_check_mark: |  21.0B | 500.000ns |
| roundtrip06.json    | :heavy_check_mark: |   2.0B | 140.000ns |
| roundtrip23.json    | :heavy_check_mark: |   9.0B | 360.000ns |
| roundtrip09.json    | :heavy_check_mark: |  13.0B |   1.520µs |
| roundtrip10.json    | :heavy_check_mark: |  22.0B |   1.550µs |

### Corner Cases
| file                |       passed       |   size |      cost |
| ------------------- | ------------------ | ------ | --------- |
| fail01_EXCLUDE.json | :heavy_check_mark: |  60.0B |   1.270µs |
| fail24.json         |        :x:         |  16.0B | 210.000ns |
| fail33.json         |        :x:         |  12.0B | 660.000ns |
| fail30.json         |        :x:         |   5.0B | 300.000ns |
| fail16.json         |        :x:         |   8.0B | 110.000ns |
| fail13.json         |        :x:         |  43.0B |   1.520µs |
| fail20.json         |        :x:         |  23.0B | 620.000ns |
| pass03.json         | :heavy_check_mark: | 148.0B |   6.610µs |
| fail08.json         |        :x:         |  16.0B | 620.000ns |
| fail12.json         |        :x:         |  31.0B | 710.000ns |
| fail10.json         |        :x:         |  58.0B |   1.140µs |
| fail29.json         |        :x:         |   4.0B | 260.000ns |
| fail03.json         |        :x:         |  37.0B | 180.000ns |
| fail22.json         |        :x:         |  33.0B | 760.000ns |
| pass02.json         | :heavy_check_mark: |  52.0B |   4.970µs |
| fail05.json         |        :x:         |  24.0B | 680.000ns |
| fail18_EXCLUDE.json | :heavy_check_mark: |  50.0B |   2.580µs |
| fail06.json         |        :x:         |  26.0B | 220.000ns |
| fail17.json         |        :x:         |  34.0B | 650.000ns |
| fail15.json         |        :x:         |  34.0B | 660.000ns |
| fail19.json         |        :x:         |  22.0B | 480.000ns |
| fail32.json         |        :x:         |  40.0B |   1.170µs |
| fail11.json         |        :x:         |  29.0B | 880.000ns |
| fail27.json         |        :x:         |  14.0B | 340.000ns |
| fail25.json         |        :x:         |  29.0B | 140.000ns |
| fail14.json         |        :x:         |  31.0B |   1.040µs |
| fail07.json         |        :x:         |  26.0B | 740.000ns |
| pass01.json         | :heavy_check_mark: |   1.4K |  47.649µs |
| fail21.json         |        :x:         |  32.0B | 720.000ns |
| fail31.json         |        :x:         |   7.0B | 260.000ns |
| fail28.json         |        :x:         |  15.0B | 360.000ns |
| fail04.json         |        :x:         |  16.0B | 580.000ns |
| fail02.json         |        :x:         |  17.0B | 460.000ns |
| fail23.json         |        :x:         |  20.0B | 520.000ns |
| fail09.json         |        :x:         |  22.0B |   1.030µs |
| fail26.json         |        :x:         |  38.0B | 310.000ns |

### Big Files
| file                |       passed       |   size |      cost |
| ------------------- | ------------------ | ------ | --------- |
| canada.json         | :heavy_check_mark: |   2.1M |  25.975ms |
| twitter.json        | :heavy_check_mark: | 616.7K |   5.873ms |
| citm_catalog.json   | :heavy_check_mark: |   1.6M |   9.062ms |
