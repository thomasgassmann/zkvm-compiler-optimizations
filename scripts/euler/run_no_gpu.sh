#!/bin/bash
OUT="${OUT:-out.txt}"
TIMED="${TIMED:-5}"
TIMEH="${TIMEH:-00}"

sbatch <<EOT
#!/bin/bash

#SBATCH -J ${OUT}
#SBATCH -c 16
#SBATCH --mem-per-cpu=1GB
#SBATCH --output=${OUT}
#SBATCH --error=${OUT}
#SBATCH --time=${TIMED}-${TIMEH}:00:00
#SBATCH --exclusive
#SBATCH --constraint EPYC_7742

module load stack/2024-06 openssl/3.1.3-zhfub4o cuda/12.1.1 gperftools/2.13 eth_proxy
./zkbench.sh $@

exit 0
EOT
