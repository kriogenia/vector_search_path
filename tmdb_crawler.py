import argparse
import gzip
import time

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

MIN_WAIT = 1.0 / 40.0   # max of 40 requests per second
last_request_instant = time.time()
with gzip.open(args.file, "rt") as file:
    for line in file:
        ellapsed = time.time() - last_request_instant
        if ellapsed < MIN_WAIT:
            time.sleep(MIN_WAIT - ellapsed)
        last_request_instant = time.time()

        id = line[20:].split(",", 1)[0]
        
        print(f"Requesting details of {id}")
        
            
            

