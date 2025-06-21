#!/bin/bash
TIMED="${TIMED:-5}"
TIMEH="${TIMEH:-00}"

sbatch <<EOT
#!/bin/bash

#SBATCH -J ${JOBNAME}
#SBATCH -c 16
#SBATCH --mem-per-cpu=1GB
${OUT:+#SBATCH --output=${OUT}}
${OUT:+#SBATCH --error=${OUT}}
#SBATCH --time=${TIMED}-${TIMEH}:00:00
#SBATCH --exclusive
#SBATCH --constraint EPYC_7742

module load stack/2024-06 openssl/3.1.3-zhfub4o cuda/12.1.1 gperftools/2.13
./zkbench.sh $@

exit 0
EOT
