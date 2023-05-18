import argparse
import gzip

parser = argparse.ArgumentParser(
    prog="TMDB Crawler",
    description="Tool to automatically query TMDB API /movies to populate with data the provided list"
)

parser.add_argument("-t", "--token",
                  help="TMDB API token",
                  default=None,
                  required=True)

parser.add_argument("-f", "--file",
                  help="jsonl compressed file with the list of movies to populate",
                  default=None,
                  required=True)

args = parser.parse_args()

print("-- Starting TMDB crawler --")

with gzip.open(args.file, "rt") as file:
    for line in file:
        id = line[20:].split(",", 1)[0]
        print(id)

