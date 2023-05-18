# Semantic Search Path

The plan of this project is to create a semantic search engine (or vector search engine) of movies. Following the project that's developed in Empathy Academy Search Path.

## Getting started

**TODO add git clone instructions **

### Generating the movie dataset

After this you'll need to get a movie dataset for the engine. To do this you can get a file with all the IDs of the movies of The Movie Database from TMDB itself.

```sh
curl -XGET http://files.tmdb.org/p/exports/movie_ids_05_15_2023.json.gz -o data/movie_ids.json.gz
```

And then use this file with the utility `tmdb_crawler` that we are providing to retrieve the details of all those movies to get a complete dataset. To do this you can run:

```sh
python tmdb_crawler.py -t TMDB_API_TOKEN ./data/movie_ids.json.gz
```

The `tmdb_crawler` is a pretty naïve script built on assumptions over the files provided by TMDB, it sacrifies robustness for speed (the ID are read using substrings, not JSON parsing). You can provide a different file with a subset of movies if you want, but the format must be the same and it must be compressed with `gunzip`.

For example, if you want to use only the first fifty movies you can do:

```sh
gzip -d data/movie_ids.json.gz
cat data/movie_ids.json | head -n 50 > movie_ids_small.json
gzip movie_ids_small.json
python tmdb_crawler.py -t TMDB_API_TOKEN ./data/movie_ids_small.json.gz
```


## Next steps
- [x] Download movie dataset
- [ ] Create crawler service 
  - [x] Read compressed file
  - [x] Extract IDs
  - [x] Request details
  - [ ] Append content to file
  - [ ] Compress output file
  - [ ] [Optional] Accept output path and name 
- [ ] Improve embedding module
  - [ ] Read the docs from Kafka topic "docs"
  - [ ] Send the embedded docs to Kafka topic "embedded"
- [ ] Create index service
  - [ ] POST /index -> docs
  - [ ] Kafka producer to "docs" topic
- [ ] Create embedded consumer
  - [ ] Kafka consumer of "embedded" topic
  - [ ] Index the data into elastic/solr/opensearch whatever
- [ ] Search service
  - [ ] GET /search?q

