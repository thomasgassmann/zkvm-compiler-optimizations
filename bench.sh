#! /bin/bash

systemd-run --scope -p CPUQuota="45%" --user  cargo bench
