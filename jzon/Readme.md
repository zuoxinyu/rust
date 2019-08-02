## Jzon
A simple JSON library in Rust.

## Sample Results
Sample files from (JSON\_checker)[http://www.json.org/JSON\_checker/].

### Roundtrip
 | file             | passed             | cost   | 
 | ---------------- | ------             | ------ | 
 | roundtrip01.json | :heavy_check_mark: | 180ns  | 
 | roundtrip02.json | :heavy_check_mark: | 290ns  | 
 | roundtrip03.json | :heavy_check_mark: | 180ns  | 
 | roundtrip04.json | :heavy_check_mark: | 120ns  | 
 | roundtrip05.json | :heavy_check_mark: | 550ns  | 
 | roundtrip06.json | :heavy_check_mark: | 50ns   | 
 | roundtrip07.json | :heavy_check_mark: | 2.06µs | 
 | roundtrip08.json | :heavy_check_mark: | 360ns  | 
 | roundtrip09.json | :heavy_check_mark: | 2.18µs | 
 | roundtrip10.json | :heavy_check_mark: | 780ns  | 
 | roundtrip11.json | :heavy_check_mark: | 130ns  | 
 | roundtrip12.json | :heavy_check_mark: | 190ns  | 
 | roundtrip13.json | :heavy_check_mark: | 240ns  | 
 | roundtrip14.json | :heavy_check_mark: | 220ns  | 
 | roundtrip15.json | :heavy_check_mark: | 140ns  | 
 | roundtrip16.json | :heavy_check_mark: | 190ns  | 
 | roundtrip17.json | :heavy_check_mark: | 130ns  | 
 | roundtrip18.json | :heavy_check_mark: | 240ns  | 
 | roundtrip19.json | :heavy_check_mark: | 320ns  | 
 | roundtrip20.json | :heavy_check_mark: | 160ns  | 
 | roundtrip21.json | :heavy_check_mark: | 140ns  | 
 | roundtrip22.json | :heavy_check_mark: | 150ns  | 
 | roundtrip23.json | :heavy_check_mark: | 160ns  | 
 | roundtrip24.json | :heavy_check_mark: | 500ns  | 
 | roundtrip25.json | :heavy_check_mark: | 400ns  | 
 | roundtrip26.json | :heavy_check_mark: | 8.11µs | 
 | roundtrip27.json | :heavy_check_mark: | 710ns  | 

### Corner Cases 
 | file                 | passed             | cost   | 
 | -------------------  | ------             | ------ | 
 | fail01\_EXCLUDE.json | :x:                | 560ns  | 
 | fail24.json          | :heavy_check_mark: | 90ns   | 
 | fail33.json          | :heavy_check_mark: | 320ns  | 
 | fail30.json          | :heavy_check_mark: | 140ns  | 
 | fail16.json          | :heavy_check_mark: | 70ns   | 
 | fail13.json          | :heavy_check_mark: | 860ns  | 
 | fail20.json          | :heavy_check_mark: | 280ns  | 
 | pass03.json          | :heavy_check_mark: | 1.71µs | 
 | fail08.json          | :x:                | 240ns  | 
 | fail12.json          | :heavy_check_mark: | 330ns  | 
 | fail10.json          | :x:                | 450ns  | 
 | fail29.json          | :heavy_check_mark: | 110ns  | 
 | fail03.json          | :heavy_check_mark: | 70ns   | 
 | fail22.json          | :heavy_check_mark: | 340ns  | 
 | pass02.json          | :heavy_check_mark: | 3.3µs  | 
 | fail05.json          | :heavy_check_mark: | 340ns  | 
 | fail18\_EXCLUDE.json | :x:                | 1.28µs | 
 | fail06.json          | :heavy_check_mark: | 110ns  | 
 | fail17.json          | :heavy_check_mark: | 300ns  | 
 | fail15.json          | :heavy_check_mark: | 320ns  | 
 | fail19.json          | :heavy_check_mark: | 240ns  | 
 | fail32.json          | :heavy_check_mark: | 540ns  | 
 | fail11.json          | :heavy_check_mark: | 500ns  | 
 | fail27.json          | :heavy_check_mark: | 140ns  | 
 | fail25.json          | :heavy_check_mark: | 70ns   | 
 | fail14.json          | :heavy_check_mark: | 530ns  | 
 | fail07.json          | :x:                | 290ns  | 
 | pass01.json          | :x:                | 7.85µs | 
 | fail21.json          | :heavy_check_mark: | 340ns  | 
 | fail31.json          | :heavy_check_mark: | 120ns  | 
 | fail28.json          | :heavy_check_mark: | 160ns  | 
 | fail04.json          | :heavy_check_mark: | 320ns  | 
 | fail02.json          | :heavy_check_mark: | 240ns  | 
 | fail23.json          | :heavy_check_mark: | 300ns  | 
 | fail09.json          | :heavy_check_mark: | 490ns  | 
 | fail26.json          | :heavy_check_mark: | 150ns  | 

### Big Files
 | file               | passed             | cost        | 
 | canada.json        | :heavy_check_mark: | 20.578376ms | 
 | canada.json        | :heavy_check_mark: | 20.578376ms | 
 | twitter.json       | :heavy_check_mark: | 6.466978ms  | 
 | citm\_catalog.json | :heavy_check_mark: | 9.749937ms  | 
