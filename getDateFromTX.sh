#!/bin/bash
 
txID="${1}"   #1 represent 1st argument

./toCurl.sh getrawtransaction $txID 1 | jq .time | xargs -i date -d @"{}"
