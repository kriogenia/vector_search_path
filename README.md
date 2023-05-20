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
python tmdb_crawler.py -z -t TMDB_API_TOKEN ./data/movie_ids.json.gz
```

This will generate a new file `./data/tmdb_movie_ids,json.gz` with all the populated data. If you want the raw output instead of the compressed file you can omit the `-z` flag, or can unzip the content youself.

Taking into account that the `tmdb_crawler` is a pretty naïve script built on assumptions over the files provided by TMDB, it sacrifies robustness for speed (the ID are read using substrings, not JSON parsing). You can provide a different file with a subset of movies if you want, but the format must be the same and it must be compressed in a way that `gzip` can read it.

For example, if you want to use only the first fifty movies you can do:

```sh
gzip -d data/movie_ids.json.gz
cat data/movie_ids.json | head -n 50 > movie_ids_small.json
gzip data/movie_ids_small.json
python tmdb_crawler.py -t TMDB_API_TOKEN ./data/movie_ids_small.json.gz
```

### Launching the services

TODO. if it's only a single `docker-compose up` it would be perfect.

Launch the `vectorize_service` -> `docker run -p 3000:3000`

### Building the index

TODO. curl the index, explain what happens

## Next steps
- [x] Download movie dataset
- [x] Create crawler service 
  - [x] Read compressed file
  - [x] Extract IDs
  - [x] Request details
  - [x] Append content to file
  - [x] Compress output file
  - [x] [Optional] Accept output path and name 
- [ ] Create index service
  - [ ] POST /index -> docs
  - [ ] Kafka producer to "docs" topic
- [ ] Create index_embedding_stream
  - [ ] Kafka stream from "docs" topic to "embedded_docs" topic
  - [ ] Encodes descriptions
- [ ] Embedding service
  - [ ] Dockerfile
  - [ ] Add image encoding
  - [ ] Create REST API
    - [x] GET /text/:text_to_embed 
    - [ ] [Optional] POST /text
  - [ ] [Optional] Configurable through ENV
  - [ ] Tests
  - [ ] [Optional] Check axum examples for improvements, f.e. transform path error
- [ ] Create embedded consumer
  - [ ] Kafka consumer of "embedded" topic
  - [ ] Index the data into elastic/solr/opensearch whatever
- [ ] Search service
  - [ ] GET /search?q

