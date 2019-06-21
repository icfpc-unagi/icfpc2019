#!/usr/bin/env bash

cloud_sql_proxy -instances=icfpc-primary:asia-northeast1:primary=tcp:3306 &
sleep 1
dev_appserver.py --host=0.0.0.0 --admin_host=0.0.0.0 app-dev.yaml
