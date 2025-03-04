#! /bin/bash

systemd-run --scope -p CPUQuota="800%" --user  cargo bench
