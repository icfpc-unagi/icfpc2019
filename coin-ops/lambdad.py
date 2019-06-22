#!/usr/bin/env python3
from werkzeug.wrappers import Request, Response
from werkzeug.serving import run_simple
from jsonrpc import JSONRPCResponseManager, dispatcher
from cachetools import cached, TTLCache
import urllib, urllib.parse
import requests
import json
import argparse
import threading
import os
import configparser
from datetime import datetime

# https://stackoverflow.com/questions/12435211/python-threading-timer-repeat-function-every-n-seconds
def every(interval):
    def decorator(function):
        def wrapper(*args, **kwargs):
            stopped = threading.Event()

            def loop(): # executed in another thread
                while not stopped.wait(interval): # until stopped
                    function(*args, **kwargs)

            t = threading.Thread(target=loop)
            t.daemon = True # stop if the program exits
            t.start()
            return stopped
        return wrapper
    return decorator

# In case of multi-threaded acceses: keep cache coherent
lock = threading.RLock()
CACHE_TIME = 5
REFRESH_TIME = CACHE_TIME + 0   # no reason for this to be smaller than CACHE_TIME

TASK_FILE = "task.desc"
PUZZLE_FILE = "puzzle.cond"
BALANCES_FILE = "balances.json"
TS_FILE = "timestamp.txt"
DONE_FILE = ".done"

CONFIG_FILE = 'lambda.conf'
# Populated by config
DEFAULT_BIND_ADDR = '127.0.0.1'
DEFAULT_PORT = 8332
DATA_DIR = 'blocks/'
BLOCKCHAIN_ENDPOINT = 'http://localhost:5000/lambda/'
PRIVATE_ID = None
PUBLIC_ID = None

# Totally decentralised!
@cached(cache=TTLCache(maxsize=10, ttl=CACHE_TIME), lock=lock)
def pass_through(method_name, arg=None):
    url = urllib.parse.urljoin(BLOCKCHAIN_ENDPOINT, method_name)
    if arg is not None:
        url = urllib.parse.urljoin(url + '/', str(arg))
    with urllib.request.urlopen(url) as s:
        return json.loads(s.read())

# JSON-RPC methods
@dispatcher.add_method
def getblockchaininfo():
    return pass_through('getblockchaininfo')

@dispatcher.add_method
def getmininginfo():
    return pass_through('getmininginfo')

@dispatcher.add_method
def getbalances():
    return pass_through('getbalances')

@dispatcher.add_method
def getbalance(id=None):
    if id is None:
        id = PUBLIC_ID
    return pass_through('getbalance', id)

@dispatcher.add_method
def getblockinfo(block_num=None):
    return pass_through('getblockinfo', block_num)

@dispatcher.add_method
def submit(block_num, sol_path, desc_path):
    url = urllib.parse.urljoin(BLOCKCHAIN_ENDPOINT, 'submit')
    data = {'private_id': PRIVATE_ID, 'block_num': block_num}
    files = {'solution': open(sol_path), 'puzzle': open(desc_path)}
    response = requests.post(url, data=data, files=files, allow_redirects=True)
    return response.json()

# Auto-update logic
def have_block(block_num):
    block_num = str(block_num)
    df = os.path.join(DATA_DIR, block_num, DONE_FILE)
    return os.path.exists(df)

def save_block(block_info):
    block_num = str(block_info['block'])
    ts = block_info['block_ts']
    balances = block_info['balances']
    task = block_info['task']
    puzzle = block_info['puzzle']

    bd = os.path.join(DATA_DIR, block_num)
    os.makedirs(bd, exist_ok=True)
    tsf = os.path.join(bd, TS_FILE)
    bf = os.path.join(bd, BALANCES_FILE)
    tf = os.path.join(bd, TASK_FILE)
    pf = os.path.join(bd, PUZZLE_FILE)
    df = os.path.join(bd, DONE_FILE)

    with open(tsf, 'w') as f:
        f.write(str(ts))
    with open(bf, 'w') as f:
        json.dump(balances, f)
    with open(tf, 'w') as f:
        f.write(task)
    with open(pf, 'w') as f:
        f.write(puzzle)

    # Create the DONE file
    with open(df, 'w') as f:
        f.close()

# Update every REFRESH_TIME seconds
@every(REFRESH_TIME)
def update():
    try:
        block_info = getblockinfo()
        block_num = block_info['block']

        if not have_block(block_num):
            save_block(block_info)

        # Fill in gaps if they exist
        for b in range(1, block_num):
            if not have_block(b):
                save_block(getblockinfo(b))
    except Exception as e:
        now = datetime.now().strftime("%c")
        print("[{}] Update exception: {}".format(now, e))

# Daemon
@Request.application
def application(request):
    response = JSONRPCResponseManager.handle(
        request.data, dispatcher)
    return Response(response.json, mimetype='application/json')

if __name__ == '__main__':
    config = configparser.ConfigParser()
    config.read(CONFIG_FILE)
    settings = config['DEFAULT']
    keys = config['SECRET']

    # Populate global settings
    DATA_DIR = settings.get('DataDir')
    BLOCKCHAIN_ENDPOINT = settings.get('DecentralisationProvider')
    DEFAULT_BIND_ADDR = settings.get('DefaultBindAddress')
    DEFAULT_PORT = settings.getint('DefaultPort')

    PRIVATE_ID = keys.get('PrivateKey')
    PUBLIC_ID = keys.get('PublicKey')

    # Parse arguments
    parser = argparse.ArgumentParser(description='JSON-RPC daemon for the LambdaCoin blockchain.')
    parser.add_argument('-b', '--bind', default=DEFAULT_BIND_ADDR, help='bind on address')
    parser.add_argument('-p', '--port', default=DEFAULT_PORT, help='listen on port')

    args = parser.parse_args()
    try:
        args.port = int(args.port)
    except ValueError:
        parser.error('Port must be an integer.')

    updater = update()
    run_simple(args.bind, args.port, application)
