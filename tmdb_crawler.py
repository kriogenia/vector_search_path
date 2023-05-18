import argparse
import gzip
import os
import requests
import threading
import time

parser = argparse.ArgumentParser(
    prog='TMDB Crawler',
    description='Tool to automatically query TMDB API /movies to populate with data the provided list'
)

parser.add_argument('-t', '--token',
                    help='Your TMDB API token',
                    default=None,
                    required=True)

parser.add_argument('-o', '--output',
                    help='Path and name to the output file',
                    default=None)

parser.add_argument('-z', '--zip',
                    help='Compresses the output file',
                    action='store_true',
                    default=False)

parser.add_argument('file',
                    help='jsonl compressed file with the list of movies to populate',
                    default=None)

args = parser.parse_args()

def request_tmdb(url, token=args.token):
    return requests.get(url, headers={
        'accept': 'application/json',
        'Authorization': f'Bearer {token}'
    })


failed_requests = {}
def request_movie_details(movie_id):
    response = request_tmdb(
        f'https://api.themoviedb.org/3/movie/{movie_id}?language=en-US')
    if response.status_code != 200:
        failed_requests[movie_id] = response.text
        return
    return response.text


print('> Starting TMDB crawler')


print('>> Authenticating to TMDB API')
authResponse = request_tmdb('https://api.themoviedb.org/3/authentication')
assert (authResponse.status_code == 200)
print('>> Authentication successful')


print('>> Launching status update thread')
start_time = time.time()
counter = 0
def print_status():
    while True:
        time.sleep(5)
        ellapsed = time.time() - start_time
        print(
            f'>>> Requested: {counter} movies. Time ellapsed: {ellapsed:.0f} s')
threading._start_new_thread(print_status, ())


print('>> Preparing output file')
output_path = args.output
if output_path == None:
    _, filename = os.path.split(args.file)
    filename = filename if args.zip else filename[:-3]
    output_path = f'./data/tmdb_{filename[:-3]}'
output_file = gzip.open(output_path, 'wb') if args.zip else open(output_path, 'w')

process = lambda a : a + '\n'
if args.zip:
    process = lambda a: bytes(a + '\n', 'utf-8') # gzip writting requires bytes


MIN_WAIT = 1.0 / 40.0   # max of 40 requests per second
last_request_instant = start_time


print('>> Requesting details of movies in the input file')
with gzip.open(args.file, 'rt') as file, output_file:
    for line in file:
        ellapsed = time.time() - last_request_instant
        if ellapsed < MIN_WAIT:
            time.sleep(MIN_WAIT - ellapsed)
        last_request_instant = time.time()

        id = line[20:].split(',', 1)[0]
        details = request_movie_details(id)

        output_file.write(process(details))
        counter += 1
print(
    f'>> Finishing movie population, requested {counter} lines in {time.time() - start_time:.0f} s')


if len(failed_requests) > 0:
    print('x Failed to retrieve the details of the following movies:')
    print(failed_requests)


print('> Closing TMDB crawler')