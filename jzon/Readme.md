## Jzon
A simple JSON library in Rust.

## Sample Results
Sample files from [JSON\_checker](http://www.json.org/JSON\_checker/).
P.S.: `fail01.json` is excluded as it is relaxed in RFC7159. `fail18.json` is excluded as depth of JSON is not specified.


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
 | fail02.json          | :heavy_check_mark: | 240ns  | 
 | fail03.json          | :heavy_check_mark: | 70ns   | 
 | fail04.json          | :heavy_check_mark: | 320ns  | 
 | fail05.json          | :heavy_check_mark: | 340ns  | 
 | fail06.json          | :heavy_check_mark: | 110ns  | 
 | fail07.json          | :heavy_check_mark: | 290ns  | 
 | fail08.json          | :heavy_check_mark: | 240ns  | 
 | fail09.json          | :heavy_check_mark: | 490ns  | 
 | fail10.json          | :heavy_check_mark: | 450ns  | 
 | fail11.json          | :heavy_check_mark: | 500ns  | 
 | fail12.json          | :heavy_check_mark: | 330ns  | 
 | fail13.json          | :heavy_check_mark: | 860ns  | 
 | fail14.json          | :heavy_check_mark: | 530ns  | 
 | fail15.json          | :heavy_check_mark: | 320ns  | 
 | fail16.json          | :heavy_check_mark: | 70ns   | 
 | fail17.json          | :heavy_check_mark: | 300ns  | 
 | fail18\_EXCLUDE.json | :x:                | 1.28µs | 
 | fail19.json          | :heavy_check_mark: | 240ns  | 
 | fail20.json          | :heavy_check_mark: | 280ns  | 
 | fail21.json          | :heavy_check_mark: | 340ns  | 
 | fail22.json          | :heavy_check_mark: | 340ns  | 
 | fail23.json          | :heavy_check_mark: | 300ns  | 
 | fail24.json          | :heavy_check_mark: | 90ns   | 
 | fail25.json          | :heavy_check_mark: | 70ns   | 
 | fail26.json          | :heavy_check_mark: | 150ns  | 
 | fail27.json          | :heavy_check_mark: | 140ns  | 
 | fail28.json          | :heavy_check_mark: | 160ns  | 
 | fail29.json          | :heavy_check_mark: | 110ns  | 
 | fail30.json          | :heavy_check_mark: | 140ns  | 
 | fail31.json          | :heavy_check_mark: | 120ns  | 
 | fail32.json          | :heavy_check_mark: | 540ns  | 
 | fail33.json          | :heavy_check_mark: | 320ns  | 
 | pass01.json          | :heavy_check_mark: | 7.85µs | 
 | pass02.json          | :heavy_check_mark: | 3.3µs  | 
 | pass03.json          | :heavy_check_mark: | 1.71µs | 

### Big Files
 | file               | passed             | cost        | 
 | ------------------ | ------------------ | ----------- | 
 | canada.json        | :heavy_check_mark: | 20.578376ms | 
 | twitter.json       | :heavy_check_mark: | 6.466978ms  | 
 | citm\_catalog.json | :heavy_check_mark: | 9.749937ms  | 
