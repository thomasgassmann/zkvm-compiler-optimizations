#!/bin/bash
OUT="${OUT:-out.txt}"
TIMED="${TIMED:-5}"

sbatch <<EOT
#!/bin/bash

#SBATCH -J ${OUT}
#SBATCH -c 32
#SBATCH --mem-per-cpu=32GB
#SBATCH --output=${OUT}
#SBATCH --error=${OUT}
#SBATCH --time=${TIMED}-00:00:00

module load stack/2024-06 openssl/3.1.3-zhfub4o cuda/12.1.1 gperftools/2.13
./zkbench.sh $@

exit 0
EOT
