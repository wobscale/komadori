#!/bin/sh

hydra migrate sql "${DATABASE_URL}"

exec hydra host --dangerous-force-http
