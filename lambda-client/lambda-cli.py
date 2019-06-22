#!/usr/bin/env python3
from jsonrpc_requests import Server
import argparse
import configparser

CONFIG_FILE = 'lambda.conf'
# Populated by config
DEFAULT_BIND_ADDR = '127.0.0.1'
DEFAULT_PORT = 8332

if __name__ == '__main__':
    config = configparser.ConfigParser()
    config.read(CONFIG_FILE)
    settings = config['DEFAULT']

    # Populate global settings
    DEFAULT_BIND_ADDR = settings.get('DefaultBindAddress')
    DEFAULT_PORT = settings.getint('DefaultPort')

    parser = argparse.ArgumentParser(description='Command-line interface for the LambdaCoin daemon.')
    parser.add_argument('-b', '--bind', default=DEFAULT_BIND_ADDR, help='daemon address')
    parser.add_argument('-p', '--port', default=DEFAULT_PORT, help='daemon port')
    subparsers = parser.add_subparsers(dest='subcmd', help='sub-command help')

    parser_bi = subparsers.add_parser('getblockchaininfo')
    parser_bi.add_argument('item', nargs='?', default=None, help="return key 'item' of result")

    parser_mi = subparsers.add_parser('getmininginfo')
    parser_mi.add_argument('item', nargs='?', default=None, help="return key 'item' of result")

    parser_bs = subparsers.add_parser('getbalances')
    parser_bs.add_argument('item', nargs='?', default=None, help="return key 'item' of result")

    parser_bs = subparsers.add_parser('getbalance')

    parser_b = subparsers.add_parser('getblockinfo')
    parser_b.add_argument('block', nargs='?', default=None, help="block to get info for")
    parser_b.add_argument('item', nargs='?', default=None, help="return key 'item' of result")
    parser_b.add_argument('subitem', nargs='?', default=None, help="return key 'subitem' of 'item'")

    parser_s = subparsers.add_parser('submit')
    parser_s.add_argument('block', default=None, help="block to submit for")
    parser_s.add_argument('task_sol_path', default=None, help=".sol file for block task")
    parser_s.add_argument('puzzle_sol_path', default=None, help=".desc file for block puzzle")

    args = parser.parse_args()
    try:
        args.port = int(args.port)
    except ValueError:
        parser.error('Port must be an integer.')

    server = Server(f'http://{args.bind}:{args.port}')

    # Handle getblockinfo
    if args.subcmd == 'getblockinfo':
        bi = None
        if args.block is None:
            bi = server.getblockinfo()
        else:
            # Allow commands like `./lambda-cli.py getblockinfo block`
            # i.e. with block taken as item rather than item
            if not args.block.isdecimal():
                args.subitem = args.item
                args.item = args.block
                args.block = None
            bi = server.getblockinfo() if args.block is None else server.getblockinfo(args.block)

        res = None
        if args.item is None:
            res = bi
        else:
            if args.subitem is None:
                res = bi.get(args.item)
            # Only have subitem for balances
            elif args.item == 'balances':
                res = bi.get(args.item).get(args.subitem, 0)
            else:
                parser.error('Item "{}" does not have any sub-items: you cannot select "{}"!'.format(args.item, args.subitem))
        print(res)

    # Handle submit
    elif args.subcmd == 'submit':
        resp = server.submit(args.block, args.task_sol_path, args.puzzle_sol_path)
        print(resp)

    # Handle getbalance
    elif args.subcmd == 'getbalance':
        print(server.getbalance())

    # Handle all other commands
    elif args.subcmd in ['getblockchaininfo', 'getmininginfo', 'getbalances']:
        bi = getattr(server, args.subcmd)()
        if args.item is None:
            print(bi)
        else:
            # if balance, print 0 rather than None
            if args.subcmd == 'getbalances':
                print(bi.get(args.item, 0))
            else:
                print(bi.get(args.item))
    else:
        parser.print_help()

