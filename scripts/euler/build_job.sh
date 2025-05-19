#!/bin/bash
OUT="${OUT:-out.txt}"
TIMED="${TIMED:-5}"
GPUMEM="${GPUMEM:-1}"
# models currently working with sp1  : rtx_3090, a100-pci-40gb, a100_80gb
# models currently working with risc0: all tested so far
GPUS="${GPUS:-a100}"

sbatch <<EOT
#!/bin/bash

#SBATCH -J ${OUT}
#SBATCH --mem-per-cpu=8GB
#SBATCH --output=${OUT}
#SBATCH --error=${OUT}
#SBATCH --time=${TIMED}-00:00:00
#SBATCH --gpus=${GPUS}
#SBATCH --gres=gpumem:${GPUMEM}

module load stack/2024-06 openssl/3.1.3-zhfub4o cuda/12.1.1 gperftools/2.13 eth_proxy
./scripts/euler/build.sh

exit 0
EOT
