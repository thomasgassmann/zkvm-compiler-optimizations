#!/bin/bash
OUT="${OUT:-out.txt}"
TIMED="${TIMED:-5}"
GPUMEM="${GPUMEM:-24g}"

sbatch <<EOT
#!/bin/bash

#SBATCH --output=${OUT}
#SBATCH --error=${OUT}
#SBATCH --time=${TIMED}-00:00:00
#SBATCH --gpus=1
#SBATCH --gres=gpumem:${GPUMEM}

module load stack/2024-06 openssl/3.1.3-zhfub4o cuda/12.1.1 gperftools/2.13
./zkbench.sh $@

exit 0
EOT
