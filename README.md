# Semantic Search Path

The plan of this project is to create a semantic search engine (or vector search engine) of movies. This project first started as a redo of the movie Search Engine I did while on Empathy's Academy but as I started to learn about semantic search I changed it to be a new engine with that extra feature.

## Getting started

**TODO add git clone instructions **

### Generating the movie dataset

After this you'll need to get a movie dataset for the engine. The original Academy Search Engine used the IMDB dataset, but the content of that dataset is not that suitable with semantic search, so, we'll need to build a dataset using The Movie Datbase API. To do this you can get a file with all the IDs of the movies from TMDB itself.

```sh
curl -XGET http://files.tmdb.org/p/exports/movie_ids_05_15_2023.json.gz -o data/movie_ids.json.gz
```

And then use this file with the utility `tmdb_crawler` that we are providing to retrieve the details of all those movies to get a complete dataset. To do this you can run:

```sh
python tmdb_crawler.py -t TMDB_API_TOKEN ./data/movie_ids.json.gz
```

This will generate a new file `./data/tmdb_movie_ids.json` with all the populated data. It's also possible to get the output zipped using the `-z` flag with the command above, but right now the index is only accepting raw text as input so there's no gain for that right now.

Take into account that the `tmdb_crawler` is a pretty naïve script built on assumptions over the files provided by TMDB, it sacrifies robustness for speed (the ID are read using substrings, not JSON parsing). You can provide a different file with a subset of movies if you want, but the format must be the same and it must be compressed in a way that `gzip` can read it.

For example, if you want to use only the first fifty movies you can do:

```sh
gzip -d data/movie_ids.json.gz
cat data/movie_ids.json | head -n 50 > movie_ids_small.json
gzip data/movie_ids_small.json
python tmdb_crawler.py -t TMDB_API_TOKEN ./data/movie_ids_small.json.gz
```

### Launching the services

To launch all the services you'll only need Docker Compose. The project is prepared to deploy in you local the `vectorize_service` and the `index_service`, plus the Kafka and Zookeper required by the last one. A little advise, these won't be lightweight images as we feature machine learning pre-trained models among other things, so it can take a bit of time and space to get everything ready.

```sh
docker compose up
```

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
  - [x] POST /docs -> docs
    - [x] Raw text
    - [ ] [Optional] JSON file
    - [ ] [Optional] gzip file
  - [ ] Kafka producer to "index.doc.raw" topic -> { index, doc }
- [ ] Create index_embedding_stream
  - [ ] Kafka stream from "index.doc.raw" topic to "index.doc.vectorized" topic
  - [ ] Encodes descriptions based on Schema
- [ ] Embedding service
  - [x] Dockerfile
  - [ ] REST API
    - [x] GET /text/:text_to_embed
    - [ ] POST /image
    - [ ] [Optional] POST /text
  - [ ] [Optional] Configurable through ENV
  - [ ] [Optional] Check axum examples for improvements, f.e. transform path error
- [ ] Create index_sink
  - [ ] Kafka consumer of "index.doc.vectorized" topic
  - [ ] Index the data into elastic/solr/opensearch/whatever
- [ ] Search service
  - [ ] GET /search?q
- [ ] [Optional] Schema Service
